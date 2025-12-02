use nostr_sdk::prelude::*;
use anyhow::{Result, Context};
use dotenvy::dotenv;
use std::fs;
use nosjob::{JobListing, EmploymentType, JobLocationType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Simplified job JSON format
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimpleJobJson {
    #[serde(rename = "type")]
    job_type: String,
    version: u32,
    title: String,
    company: String,
    location: String,
    employment_type: String,
    description: String,
    skills: Vec<String>,
    salary: SalaryJson,
    #[serde(default)]
    identifier: Option<String>,
    #[serde(default)]
    valid_through: Option<String>,
    #[serde(default)]
    lightning_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SalaryJson {
    currency: String,
    min: f64,
    max: f64,
}

/// Load environment variables from .env
fn load_env() {
    dotenv().ok();
}

/// Ensure a private key exists: load from .env or generate
fn ensure_private_key() -> String {
    load_env();

    if let Ok(key) = std::env::var("PRIVATE_KEY") {
        println!("🔑 Loaded existing PRIVATE_KEY from .env");
        key
    } else {
        let keys = Keys::generate();
        let sk_bech32 = keys.secret_key().to_bech32().expect("Failed to convert secret key to bech32");
        fs::write(".env", format!("PRIVATE_KEY={}\n", sk_bech32)).expect("Failed to write .env");
        println!("🔑 Generated new PRIVATE_KEY and saved to .env");
        sk_bech32
    }
}

/// Parse employment type from string
fn parse_employment_type(s: &str) -> EmploymentType {
    match s.to_lowercase().as_str() {
        "full-time" | "fulltime" => EmploymentType::FullTime,
        "part-time" | "parttime" => EmploymentType::PartTime,
        "contractor" | "contract" => EmploymentType::Contractor,
        "intern" | "internship" => EmploymentType::Intern,
        "temporary" | "temp" => EmploymentType::Temporary,
        "volunteer" => EmploymentType::Volunteer,
        _ => EmploymentType::Other,
    }
}

/// Parse location type from string
fn parse_location_type(s: &str) -> JobLocationType {
    match s.to_lowercase().as_str() {
        "remote" | "telecommute" => JobLocationType::Telecommute,
        "onsite" | "on-site" | "office" => JobLocationType::OnSite,
        "hybrid" => JobLocationType::Hybrid,
        _ => JobLocationType::Telecommute,
    }
}

/// Load job from JSON file
fn load_job_from_file(path: &str) -> Result<SimpleJobJson> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read job file: {}", path))?;
    
    let job: SimpleJobJson = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from: {}", path))?;
    
    Ok(job)
}

/// Convert SimpleJobJson to JobListing with UUID
fn convert_to_job_listing(simple: &SimpleJobJson, pubkey: &str) -> Result<JobListing> {
    // Generate UUID for uniqueness
    let uuid = Uuid::new_v4();
    let uuid_short = uuid.to_string().split('-').next().unwrap().to_string();
    
    let identifier = simple.identifier.clone()
        .unwrap_or_else(|| format!("{}-{}-{}", 
            simple.company.to_lowercase().replace(' ', "-"),
            simple.title.to_lowercase().replace(' ', "-").chars().take(20).collect::<String>(),
            uuid_short
        ));

    let date_posted = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let mut builder = JobListing::builder(
        identifier,
        simple.title.clone(),
        simple.description.clone(),
        simple.company.clone(),
        date_posted,
        "".to_string(),
    )
    .employment_type(vec![parse_employment_type(&simple.employment_type)])
    .location_type(vec![parse_location_type(&simple.location)])
    .salary(
        simple.salary.min,
        simple.salary.max,
        simple.salary.currency.clone(),
        "YEAR".to_string(),
    )
    .skills(simple.skills.clone())
    .nostr_pubkey(pubkey.to_string());

    if let Some(valid_through) = &simple.valid_through {
        builder = builder.valid_through(valid_through.clone());
    }

    if let Some(lightning) = &simple.lightning_address {
        builder = builder.lightning_address(lightning.clone());
    }

    Ok(builder.build()?)
}

/// Print help message
fn print_help() {
    println!("🚀 Nostr Job Poster\n");
    println!("USAGE:");
    println!("  cargo run -- <job.json> [--publish]    Post a job listing");
    println!("  cargo run -- view_jobs                  View recent job listings");
    println!("  cargo run -- help                       Show this help\n");
    println!("EXAMPLES:");
    println!("  cargo run -- my-job.json               Dry run (preview)");
    println!("  cargo run -- my-job.json --publish     Publish to relays");
    println!("  cargo run -- view_jobs                 View recent jobs");
}

/// View recent job listings from relays
async fn view_jobs() -> Result<()> {
    println!("🔍 Fetching recent job listings...\n");
    
    let keys = Keys::generate(); // Anonymous viewing
    let client = Client::new(keys);
    
    let relays = vec![
        "wss://relay.damus.io",
        "wss://relay.nostr.band",
        "wss://nostr.wine",
    ];
    
    for relay in &relays {
        println!("   Connecting to: {}", relay);
        client.add_relay(*relay).await?;
    }
    
    client.connect().await;
    
    let filter = nosjob::JobsFilter::new()
        .limit(10)
        .build();
    
    println!("\n📡 Fetching jobs from relays...");
    let timeout = std::time::Duration::from_secs(10);
    let events = client.fetch_events(filter, timeout).await?;
    
    println!("\n📋 Found {} job listings:\n", events.len());
    
    for (i, event) in events.iter().enumerate() {
        match JobListing::from_nostr_event(event) {
            Ok(job) => {
                println!("{}. {}", i + 1, job.title);
                println!("   Company: {}", job.hiring_organization.name);
                println!("   Type: {:?}", job.employment_type);
                println!("   Location: {:?}", job.job_location_type.as_ref()
                    .and_then(|v| v.first())
                    .unwrap_or(&JobLocationType::Telecommute));
                if let Some(skills) = &job.skills {
                    println!("   Skills: {}", skills.join(", "));
                }
                println!("   Event ID: {}", event.id.to_bech32()?);
                println!();
            }
            Err(e) => {
                println!("{}. [Parse error: {}]", i + 1, e);
                println!("   Event ID: {}", event.id.to_bech32()?);
                println!();
            }
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    // Check if first arg is a command
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    
    match command {
        "view_jobs" => {
            view_jobs().await?;
            return Ok(());
        }
        "help" | "--help" | "-h" => {
            print_help();
            return Ok(());
        }
        _ => {
            // Treat as job file path
        }
    }
    
    let job_file = command;
    let publish = args.iter().any(|arg| arg == "--publish" || arg == "-p");

    println!("🚀 Nostr Job Poster (Kind 39993 with UUID)\n");

    // 1. Load job from JSON file
    println!("📄 Loading job from: {}", job_file);
    let simple_job = load_job_from_file(job_file)?;
    println!("✅ Job loaded: {}\n", simple_job.title);

    // 2. Load or generate keys
    let private_key = ensure_private_key();
    let keys = Keys::parse(&private_key)?;
    println!("📝 Using pubkey: {}\n", keys.public_key().to_bech32()?);

    // 3. Convert to full JobListing with UUID
    let job = convert_to_job_listing(&simple_job, &keys.public_key().to_hex())?;

    println!("📋 Job Details:");
    println!("   ID: {}", job.identifier);
    println!("   Title: {}", job.title);
    println!("   Company: {}", job.hiring_organization.name);
    println!("   Type: {:?}", job.employment_type);
    println!("   Location: {:?}", job.job_location_type);
    println!("   Skills: {}", job.skills.as_ref().map(|s| s.join(", ")).unwrap_or_default());
    
    if let Some(salary) = &job.base_salary {
        if let nosjob::types::MonetaryValue::Range(range) = &salary.value {
            if let (Some(min), Some(max)) = (range.min_value, range.max_value) {
                println!("   Salary: ${} - ${} {} per {}", 
                    min as u64, max as u64, salary.currency, range.unit_text);
            }
        }
    }

    // 4. Convert to Nostr event
    let event = job.to_nostr_event(&keys)?;
    println!("\n📦 Event Created:");
    println!("   ID: {}", event.id);
    println!("   Kind: {} (39993 job listing)", event.kind);
    println!("   Tags: {} tags", event.tags.len());

    // 5. Show event tags
    println!("\n🏷️  Event Tags:");
    for (i, tag) in event.tags.iter().take(10).enumerate() {
        println!("   {}: {:?}", i + 1, tag.as_slice());
    }
    if event.tags.len() > 10 {
        println!("   ... and {} more", event.tags.len() - 10);
    }

    // 6. Relay list
    let relays = vec![
        "wss://relay.damus.io",
        "wss://relay.nostr.band",
        "wss://nostr.wine",
    ];

    if publish {
        println!("\n📡 Publishing to relays...");
        let client = Client::new(keys);
        
        for relay in &relays {
            println!("   Connecting to: {}", relay);
            client.add_relay(*relay).await?;
        }
        
        client.connect().await;
        let output = client.send_event(&event.clone()).await?;
        
        println!("\n✅ Published successfully!");
        println!("   Success: {} relays", output.success.len());
        println!("   Failed: {} relays", output.failed.len());
        
        for relay in output.success {
            println!("   ✓ {}", relay);
        }
        
        if !output.failed.is_empty() {
            for (relay, err) in output.failed {
                println!("   ✗ {}: {}", relay, err);
            }
        }

        println!("\n🔗 Event Identifiers:");
        println!("   Event ID: {}", event.id.to_bech32()?);
        println!("   Pubkey: {}", event.pubkey.to_bech32()?);
    } else {
        println!("\n🔍 DRY RUN MODE (use --publish or -p to actually publish)");
        println!("\n📡 Would publish to:");
        for relay in &relays {
            println!("   • {}", relay);
        }

        println!("\n💡 To publish this job, run:");
        println!("   cargo run -- {} --publish", job_file);
    }

    Ok(())
}
