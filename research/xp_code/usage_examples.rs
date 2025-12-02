// examples/post_job.rs
// Example: Post a job to Nostr relays

use nostr_sdk::prelude::*;
use anyhow::Result;

// Import your types (adjust path as needed)
// use nostr_jobs::{JobListing, EmploymentType, JobLocationType};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Posting job to Nostr relays...\n");

    // 1. Generate or load keys
    let keys = Keys::generate();
    println!("📝 Using pubkey: {}", keys.public_key().to_bech32()?);

    // 2. Create job listing using builder
    let job = JobListing::builder(
        "acme-rust-dev-001".to_string(),
        "Senior Rust Developer".to_string(),
        "Build decentralized infrastructure on Nostr using Rust, Tokio, and WebSockets.".to_string(),
        "Acme Corp".to_string(),
        "2025-01-15".to_string(),
    )
    .valid_through("2025-03-15".to_string())
    .employment_type(vec![EmploymentType::FullTime])
    .location_type(vec![JobLocationType::Telecommute])
    .salary(150000.0, 220000.0, "USD".to_string(), "YEAR".to_string())
    .skills(vec![
        "Rust".to_string(),
        "Tokio".to_string(),
        "Nostr".to_string(),
        "WebSockets".to_string(),
    ])
    .nostr_pubkey(keys.public_key().to_hex())
    .lightning_address("jobs@acme.example.com".to_string())
    .build()?;

    println!("✅ Job created:");
    println!("   Title: {}", job.title);
    println!("   Company: {}", job.hiring_organization.name);
    println!("   Type: {:?}", job.employment_type);
    println!("   Location: {:?}", job.job_location_type);

    // 3. Convert to Nostr event
    let event = job.to_nostr_event(&keys)?;
    println!("\n📦 Event created:");
    println!("   ID: {}", event.id);
    println!("   Kind: {}", event.kind);
    println!("   Tags: {} tags", event.tags.len());

    // 4. Connect to relays
    let client = Client::new(&keys);
    
    let relays = vec![
        "wss://relay.damus.io",
        "wss://relay.nostr.band",
        "wss://nostr.wine",
    ];
    
    for relay_url in &relays {
        client.add_relay(relay_url).await?;
        println!("   ➕ Added relay: {}", relay_url);
    }
    
    client.connect().await;
    println!("\n🔌 Connected to relays");

    // 5. Publish event
    println!("\n📤 Publishing job listing...");
    let output = client.send_event(event.clone()).await?;
    
    println!("✅ Job posted successfully!");
    println!("   Event ID: {}", output.id);
    println!("   Published to {} relays", output.success.len());
    
    for relay in output.success {
        println!("      ✓ {}", relay);
    }
    
    if !output.failed.is_empty() {
        println!("   Failed relays:");
        for (relay, msg) in output.failed {
            println!("      ✗ {}: {}", relay, msg);
        }
    }

    println!("\n🔗 View your job:");
    println!("   Event: {}", event.id.to_bech32()?);
    println!("   Pubkey: {}", keys.public_key().to_bech32()?);

    Ok(())
}

// ============================================================================

// examples/search_jobs.rs
// Example: Search for jobs on Nostr relays

use nostr_sdk::prelude::*;
use anyhow::Result;

// Import your types
// use nostr_jobs::{JobListing, JobsFilter};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Searching for jobs on Nostr...\n");

    // 1. Create client (no keys needed for reading)
    let keys = Keys::generate();
    let client = Client::new(&keys);

    // 2. Add relays
    let relays = vec![
        "wss://relay.damus.io",
        "wss://relay.nostr.band",
        "wss://nostr.wine",
    ];
    
    for relay_url in relays {
        client.add_relay(relay_url).await?;
    }
    
    client.connect().await;
    println!("🔌 Connected to relays\n");

    // 3. Create filter for job listings
    let filter = Filter::new()
        .kind(Kind::from(5001))  // Job listing kind
        .hashtag("jobstr")       // Jobs hashtag
        .limit(50);              // Limit results

    println!("📥 Fetching jobs (kind 5001 with #jobstr)...\n");

    // 4. Query events
    let events = client.get_events_of(vec![filter], None).await?;
    
    println!("Found {} job events\n", events.len());
    println!("{}", "=".repeat(80));

    // 5. Parse and display jobs
    let mut jobs_found = 0;
    
    for event in events {
        match JobListing::from_nostr_event(&event) {
            Ok(job) => {
                jobs_found += 1;
                print_job(&job);
                println!("{}", "-".repeat(80));
            }
            Err(e) => {
                println!("⚠️  Failed to parse event {}: {}", event.id, e);
            }
        }
    }

    println!("\n✅ Successfully parsed {}/{} jobs", jobs_found, events.len());

    Ok(())
}

fn print_job(job: &JobListing) {
    println!("📋 {}", job.title);
    println!("🏢 {}", job.hiring_organization.name);
    
    if let Some(url) = &job.hiring_organization.url {
        println!("🔗 {}", url);
    }
    
    println!("📅 Posted: {}", job.date_posted);
    
    if let Some(expiry) = &job.valid_through {
        println!("⏰ Expires: {}", expiry);
    }
    
    if !job.employment_type.is_empty() {
        println!("💼 Type: {:?}", job.employment_type);
    }
    
    if let Some(location_types) = &job.job_location_type {
        println!("📍 Location: {:?}", location_types);
    }
    
    if let Some(salary) = &job.base_salary {
        if let MonetaryValue::Range(range) = &salary.value {
            if let (Some(min), Some(max)) = (range.min_value, range.max_value) {
                println!(
                    "💰 Salary: ${:,.0} - ${:,.0} {} ({})",
                    min, max, salary.currency, range.unit_text
                );
            }
        }
    }
    
    if let Some(skills) = &job.skills {
        println!("🛠️  Skills: {}", skills.join(", "));
    }
    
    if let Some(ln_addr) = &job.lightning_address {
        println!("⚡ Lightning: {}", ln_addr);
    }
    
    // Truncate description
    let desc = if job.description.len() > 200 {
        format!("{}...", &job.description[..200])
    } else {
        job.description.clone()
    };
    println!("\n{}\n", desc);
}

// ============================================================================

// examples/filter_jobs.rs
// Example: Search with filters (remote, salary, skills)

use nostr_sdk::prelude::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Advanced job search with filters\n");

    let keys = Keys::generate();
    let client = Client::new(&keys);

    client.add_relay("wss://relay.damus.io").await?;
    client.connect().await;

    // Fetch all jobs
    let filter = Filter::new()
        .kind(Kind::from(5001))
        .hashtag("jobstr")
        .limit(100);

    let events = client.get_events_of(vec![filter], None).await?;
    
    println!("📥 Fetched {} events\n", events.len());

    // Parse jobs
    let jobs: Vec<JobListing> = events
        .into_iter()
        .filter_map(|e| JobListing::from_nostr_event(&e).ok())
        .collect();

    println!("✅ Parsed {} valid jobs\n", jobs.len());

    // Filter 1: Remote only
    println!("🏠 Remote jobs:");
    let remote_jobs: Vec<_> = jobs
        .iter()
        .filter(|j| {
            j.job_location_type
                .as_ref()
                .map(|types| types.contains(&JobLocationType::Telecommute))
                .unwrap_or(false)
        })
        .collect();
    
    println!("   Found {} remote jobs\n", remote_jobs.len());

    // Filter 2: High salary (>$150k)
    println!("💰 High-paying jobs (>$150k):");
    let high_salary_jobs: Vec<_> = jobs
        .iter()
        .filter(|j| {
            if let Some(salary) = &j.base_salary {
                if let MonetaryValue::Range(range) = &salary.value {
                    return range.min_value.unwrap_or(0.0) >= 150000.0;
                }
            }
            false
        })
        .collect();
    
    println!("   Found {} high-paying jobs\n", high_salary_jobs.len());

    // Filter 3: Rust jobs
    println!("🦀 Rust jobs:");
    let rust_jobs: Vec<_> = jobs
        .iter()
        .filter(|j| {
            j.title.to_lowercase().contains("rust") ||
            j.description.to_lowercase().contains("rust") ||
            j.skills.as_ref().map(|s| {
                s.iter().any(|skill| skill.to_lowercase().contains("rust"))
            }).unwrap_or(false)
        })
        .collect();
    
    println!("   Found {} Rust jobs\n", rust_jobs.len());

    // Display first result from each category
    if let Some(job) = remote_jobs.first() {
        println!("Example remote job: {}", job.title);
    }
    if let Some(job) = high_salary_jobs.first() {
        println!("Example high-salary job: {}", job.title);
    }
    if let Some(job) = rust_jobs.first() {
        println!("Example Rust job: {}", job.title);
    }

    Ok(())
}
