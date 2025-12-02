// examples/view_jobs.rs
// View and search for job listings on Nostr (kind 39993)

use nostr_sdk::prelude::*;
use nosjob::JobsFilter;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Nostr Job Viewer (Kind 39993)\n");

    // Create a client (no keys needed for reading)
    let client = Client::default();

    // Connect to relays
    let relays = vec![
        "wss://relay.damus.io",
        "wss://relay.nostr.band",
        "wss://nos.lol",
    ];

    println!("📡 Connecting to relays...");
    for relay in &relays {
        client.add_relay(*relay).await?;
        println!("   • {}", relay);
    }
    client.connect().await;
    println!();

    // Build filter for all job listings
    let filter = JobsFilter::new()
        .limit(20)
        .build();

    println!("🔎 Searching for job listings...\n");

    // Fetch events from relays
    let timeout = std::time::Duration::from_secs(10);
    let events = client
        .fetch_events(filter, timeout)
        .await?;

    if events.is_empty() {
        println!("❌ No job listings found");
        println!("\n💡 Tip: Make sure jobs are published with kind 39993 and 't' tag = 'Jobs'");
        return Ok(());
    }

    println!("✅ Found {} job listing(s)\n", events.len());
    println!("{}", "=".repeat(80));

    for (i, event) in events.iter().enumerate() {
        println!("\n📌 Job #{}", i + 1);
        println!("   Event ID: {}", event.id);
        println!("   Posted: {}", event.created_at.to_human_datetime());
        println!("   Pubkey: {}", event.pubkey.to_bech32()?);

        // Extract key tags
        let tags: Vec<_> = event.tags.iter().collect();
        
        if let Some(title) = find_tag_value(&tags, "title") {
            println!("   📝 Title: {}", title);
        }
        
        if let Some(company) = find_tag_value(&tags, "company") {
            println!("   🏢 Company: {}", company);
        }
        
        if let Some(location) = find_tag_value(&tags, "location") {
            println!("   📍 Location: {}", location);
        }
        
        let employment_types: Vec<_> = tags
            .iter()
            .filter_map(|t| {
                let slice = t.as_slice();
                if slice.len() >= 2 && slice[0] == "employment-type" {
                    Some(slice[1].to_string())
                } else {
                    None
                }
            })
            .collect();
        
        if !employment_types.is_empty() {
            println!("   💼 Type: {}", employment_types.join(", "));
        }

        // Skills
        let skills: Vec<_> = tags
            .iter()
            .filter_map(|t| {
                let slice = t.as_slice();
                if slice.len() >= 2 && slice[0] == "skill" {
                    Some(slice[1].to_string())
                } else {
                    None
                }
            })
            .collect();
        
        if !skills.is_empty() {
            println!("   🛠️  Skills: {}", skills.join(", "));
        }

        // Salary
        if let Some(salary_tag) = tags.iter().find(|t| {
            let slice = t.as_slice();
            !slice.is_empty() && slice[0] == "salary"
        }) {
            let slice = salary_tag.as_slice();
            if slice.len() >= 5 {
                println!("   💰 Salary: ${} - ${} {} per {}", 
                    slice[1], slice[2], slice[3], slice[4]);
            }
        }

        // Show job ID
        if let Some(job_id) = find_tag_value(&tags, "job-id") {
            println!("   🆔 Job ID: {}", job_id);
        }

        // Lightning address
        if let Some(ln) = find_tag_value(&tags, "lightning") {
            println!("   ⚡ Lightning: {}", ln);
        }

        println!("\n   📄 Content preview:");
        let content_preview: String = event.content.chars().take(200).collect();
        println!("   {}", content_preview);
        if event.content.len() > 200 {
            println!("   ... (truncated)");
        }

        println!("\n{}", "-".repeat(80));
    }

    println!("\n✅ Done! Found {} job listing(s)", events.len());
    
    Ok(())
}

fn find_tag_value(tags: &[&Tag], name: &str) -> Option<String> {
    tags.iter().find_map(|t| {
        let slice = t.as_slice();
        if slice.len() >= 2 && slice[0] == name {
            Some(slice[1].to_string())
        } else {
            None
        }
    })
}
