# **Phase 1: Proof of Concept - Detailed Plan**

## **Goal**
Validate the technical feasibility of decentralized job listings on Nostr by building a minimal but functional implementation.

**Timeline**: 4-6 weeks (part-time) or 2-3 weeks (full-time)  
**Budget**: $0 (uses free public relays, open source tools)

---

## **Deliverables**

1. ✅ Complete NIP-XX specification document
2. ✅ Rust library (`nostr-jobs` crate)
3. ✅ CLI tool for posting/searching jobs
4. ✅ 10-20 test job listings on public relays
5. ✅ Documentation and examples
6. ✅ Technical validation report

---

## **Week-by-Week Breakdown**

### **Week 1: Specification & Design**

#### **Days 1-2: Research & Refinement**
- [ ] Study existing NIPs (especially NIP-01, 05, 33, 40)
- [ ] Research schema.org/JobPosting for compatibility
- [ ] Survey existing job board APIs (LinkedIn, Indeed, Greenhouse)
- [ ] Identify must-have vs nice-to-have fields
- [ ] Decision: Choose anti-spam mechanism for POC (suggest: simple PoW)

**Deliverable**: Requirements document with field mappings

#### **Days 3-5: Write Complete NIP Specification**

**Structure to follow**:
```markdown
NIP-XX
======

Decentralized Job Listings

`draft` `optional`

## Abstract
[200 word summary]

## Motivation
[Why this NIP is needed]

## Specification

### Event Kinds
- 31337: Job Listing
- 31338: Job Application  
- 31339: Job Status Update
- 7337: Job Reaction

### Job Listing Event (Kind 31337)
[Full schema with all tags]

### Required Tags
- `d` (identifier)
- `title`
- `company`
- `location-type`

### Optional Tags
- `salary`
- `skills`
- `experience`
[etc...]

### Tag Value Standards
[Controlled vocabularies for key fields]

### Anti-Spam Measures
[PoW requirements, relay policies]

### Privacy Considerations
[Application encryption, PII handling]

## Implementation Notes
[Guidance for relay and client implementers]

## Examples
[3-5 complete example events]

## Security Considerations
[Potential attacks and mitigations]

## References
- NIP-01: Basic protocol
- NIP-33: Parameterized replaceable events
- schema.org/JobPosting
```

**Tasks**:
- [ ] Write full specification (8-12 pages)
- [ ] Create 5 realistic example events (JSON)
- [ ] Document edge cases (expired jobs, salary ranges, multi-location)
- [ ] Peer review with Nostr community (GitHub/Discord)

**Deliverable**: `NIP-XX-draft.md` ready for community feedback

---

### **Week 2: Rust Library Development**

#### **Day 1: Project Setup**

```bash
cargo new nostr-jobs --lib
cd nostr-jobs
```

**Dependencies** (`Cargo.toml`):
```toml
[package]
name = "nostr-jobs"
version = "0.1.0"
edition = "2021"

[dependencies]
nostr-sdk = "0.37"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
anyhow = "1.0"
thiserror = "2.0"
url = "2.5"
geo = "0.30"  # for location handling

[dev-dependencies]
tokio = { version = "1", features = ["full"] }
```

**Project structure**:
```
nostr-jobs/
├── src/
│   ├── lib.rs
│   ├── types.rs          # Job, Application structs
│   ├── events.rs         # Nostr event conversion
│   ├── tags.rs           # Tag builders and parsers
│   ├── filters.rs        # Search filters
│   ├── validation.rs     # Schema validation
│   └── error.rs          # Custom errors
├── examples/
│   ├── post_job.rs
│   └── search_jobs.rs
└── tests/
    └── integration_tests.rs
```

#### **Days 2-4: Core Data Types**

**`src/types.rs`** - Define core structures:
```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobListing {
    pub id: String,  // unique identifier
    pub title: String,
    pub company: String,
    pub company_url: Option<String>,
    pub location: Location,
    pub location_type: LocationType,
    pub description: String,  // markdown
    pub salary: Option<SalaryRange>,
    pub experience_level: ExperienceLevel,
    pub employment_type: EmploymentType,
    pub categories: Vec<String>,
    pub required_skills: Vec<String>,
    pub preferred_skills: Vec<String>,
    pub published_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub apply_methods: Vec<ApplyMethod>,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationType {
    Remote,
    Hybrid,
    Onsite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,  // "San Francisco, CA"
    pub coordinates: Option<(f64, f64)>,  // lat, lon
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryRange {
    pub min: u64,
    pub max: u64,
    pub currency: String,  // ISO 4217 code
    pub period: SalaryPeriod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SalaryPeriod {
    Annual,
    Monthly,
    Hourly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperienceLevel {
    Intern,
    Junior,
    Mid,
    Senior,
    Staff,
    Principal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmploymentType {
    FullTime,
    PartTime,
    Contract,
    Internship,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplyMethod {
    Url(String),
    Email(String),
    Nostr,  // accepts kind 31338 applications
}

// Validation methods
impl JobListing {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Title length check
        // Required fields present
        // URL validation
        // Date logic (expires_at > published_at)
    }
}
```

**Tasks**:
- [ ] Implement all data structures
- [ ] Add builder patterns for ergonomic construction
- [ ] Write validation logic
- [ ] Add comprehensive doc comments

#### **Days 5-7: Nostr Event Conversion**

**`src/events.rs`** - Convert between JobListing and Nostr events:
```rust
use nostr_sdk::prelude::*;

impl JobListing {
    /// Convert JobListing to Nostr event (kind 31337)
    pub fn to_event(&self, keys: &Keys) -> Result<Event, Error> {
        let mut tags = vec![
            Tag::identifier(&self.id),
            Tag::custom(TagKind::Custom("title".into()), vec![&self.title]),
            Tag::custom(TagKind::Custom("company".into()), vec![&self.company]),
            // ... build all tags
        ];

        // Add optional tags
        if let Some(salary) = &self.salary {
            tags.push(Tag::custom(
                TagKind::Custom("salary".into()),
                vec![
                    &salary.min.to_string(),
                    &salary.max.to_string(),
                    &salary.currency,
                    &format!("{:?}", salary.period).to_lowercase(),
                ],
            ));
        }

        EventBuilder::new(Kind::from(31337), self.description.clone())
            .tags(tags)
            .sign_with_keys(keys)
    }

    /// Parse JobListing from Nostr event
    pub fn from_event(event: &Event) -> Result<Self, Error> {
        if event.kind != Kind::from(31337) {
            return Err(Error::InvalidKind);
        }

        // Parse required tags
        let id = event.tags.iter()
            .find_map(|t| match t {
                Tag::Identifier(id) => Some(id.clone()),
                _ => None,
            })
            .ok_or(Error::MissingRequiredTag("d"))?;

        // ... parse all tags

        Ok(JobListing {
            id,
            title,
            company,
            description: event.content.clone(),
            // ... all fields
        })
    }
}
```

**Tasks**:
- [ ] Implement `to_event()` method
- [ ] Implement `from_event()` method
- [ ] Handle all tag types (required, optional, multi-value)
- [ ] Add error handling for malformed events
- [ ] Write unit tests for round-trip conversion

---

### **Week 3: CLI Tool Development**

#### **Day 1: CLI Setup**

```bash
cargo new nostr-jobs-cli --bin
cd nostr-jobs-cli
```

**Dependencies**:
```toml
[dependencies]
nostr-jobs = { path = "../nostr-jobs" }
nostr-sdk = "0.37"
clap = { version = "4.5", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
anyhow = "1.0"
dialoguer = "0.11"  # interactive prompts
colored = "2.1"
```

**CLI structure**:
```
Commands:
  post      Post a new job listing
  search    Search for jobs
  get       Get a specific job by ID
  update    Update an existing job
  close     Mark a job as closed
  list      List your posted jobs
  apply     Apply to a job (encrypted)
```

#### **Days 2-4: Implement CLI Commands**

**`src/main.rs`** - Basic structure:
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nostr-jobs")]
#[command(about = "Decentralized job listings on Nostr")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Post a new job listing
    Post {
        /// Job file (JSON or interactive if omitted)
        #[arg(short, long)]
        file: Option<String>,
    },
    
    /// Search for jobs
    Search {
        /// Search query
        query: Option<String>,
        
        /// Filter by location type
        #[arg(long)]
        location_type: Option<String>,
        
        /// Filter by experience level
        #[arg(long)]
        experience: Option<String>,
        
        /// Minimum salary
        #[arg(long)]
        min_salary: Option<u64>,
    },
    
    /// Get a specific job
    Get {
        /// Job ID
        id: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Post { file } => handle_post(file).await,
        Commands::Search { query, .. } => handle_search(query).await,
        Commands::Get { id } => handle_get(id).await,
    }
}
```

**Interactive job posting** (using dialoguer):
```rust
async fn handle_post(file: Option<String>) -> Result<()> {
    let job = if let Some(path) = file {
        // Load from JSON file
        JobListing::from_json_file(path)?
    } else {
        // Interactive prompts
        let title = Input::new()
            .with_prompt("Job title")
            .interact_text()?;
        
        let company = Input::new()
            .with_prompt("Company name")
            .interact_text()?;
        
        // ... more prompts
        
        JobListing::builder()
            .title(title)
            .company(company)
            // ...
            .build()?
    };
    
    // Connect to relays
    let keys = Keys::generate();  // TODO: load from config
    let client = Client::new(&keys);
    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay("wss://relay.nostr.band").await?;
    client.connect().await;
    
    // Publish job
    let event = job.to_event(&keys)?;
    client.send_event(event).await?;
    
    println!("✅ Job posted! Event ID: {}", event.id);
    Ok(())
}
```

**Tasks**:
- [ ] Implement `post` command (interactive & file-based)
- [ ] Implement `search` command with filters
- [ ] Implement `get` command
- [ ] Add colored output for better UX
- [ ] Add config file support (~/.nostr-jobs/config.toml)
- [ ] Store keys securely (ask for password?)

#### **Days 5-7: Search & Display**

**Search implementation**:
```rust
async fn handle_search(query: Option<String>) -> Result<()> {
    let client = Client::new(&Keys::generate());
    
    // Add relays
    client.add_relay("wss://relay.damus.io").await?;
    client.connect().await;
    
    // Subscribe to kind 31337 events
    let filter = Filter::new()
        .kind(Kind::from(31337))
        .limit(50);
    
    let events = client.get_events_of(vec![filter], None).await?;
    
    // Parse into jobs
    let mut jobs: Vec<JobListing> = events
        .into_iter()
        .filter_map(|e| JobListing::from_event(&e).ok())
        .collect();
    
    // Apply filters
    if let Some(q) = query {
        jobs.retain(|j| {
            j.title.to_lowercase().contains(&q.to_lowercase()) ||
            j.description.to_lowercase().contains(&q.to_lowercase())
        });
    }
    
    // Display results
    for job in jobs {
        println!("{}", format!("━━━━━━━━━━━━━━━").cyan());
        println!("{} {}", "📋".bold(), job.title.bold());
        println!("{} {}", "🏢", job.company);
        println!("{} {:?}", "📍", job.location_type);
        if let Some(salary) = job.salary {
            println!("{} ${}-{} {}", "💰", salary.min, salary.max, salary.currency);
        }
        println!("{} {}", "🆔", job.id.dimmed());
    }
    
    Ok(())
}
```

**Tasks**:
- [ ] Implement search with keyword matching
- [ ] Add filter support (location, salary, experience)
- [ ] Pretty print results with colors
- [ ] Add pagination for large result sets
- [ ] Export results to JSON/CSV

---

### **Week 4: Testing & Documentation**

#### **Days 1-2: Integration Testing**

Create test scenarios:
```rust
#[tokio::test]
async fn test_post_and_retrieve_job() {
    // Post a job
    let job = JobListing::builder()
        .title("Test Job")
        .company("Test Corp")
        .build()
        .unwrap();
    
    let keys = Keys::generate();
    let event = job.to_event(&keys).unwrap();
    
    // Verify round-trip
    let parsed = JobListing::from_event(&event).unwrap();
    assert_eq!(parsed.title, job.title);
}
```

**Tasks**:
- [ ] Write 20+ unit tests
- [ ] Write 5+ integration tests
- [ ] Test with real public relays
- [ ] Post 10-20 realistic test jobs
- [ ] Verify events can be retrieved
- [ ] Test edge cases (very long descriptions, special characters)

#### **Days 3-4: Documentation**

**Create**:
- [ ] `README.md` with quick start guide
- [ ] `ARCHITECTURE.md` explaining design decisions
- [ ] `NIP-XX.md` final draft
- [ ] API documentation (cargo doc)
- [ ] Example JSON job files (5 different industries)
- [ ] Video demo (optional, 5-10 min)

**Example README sections**:
```markdown
# nostr-jobs

Decentralized job listings on Nostr.

## Quick Start

### Install
```bash
cargo install nostr-jobs-cli
```

### Post a Job
```bash
nostr-jobs post
# Follow interactive prompts

# Or from file
nostr-jobs post --file job.json
```

### Search Jobs
```bash
nostr-jobs search "rust developer"
nostr-jobs search --location-type remote --min-salary 100000
```

## Library Usage
```rust
use nostr_jobs::*;

let job = JobListing::builder()
    .title("Senior Rust Developer")
    .company("Acme Corp")
    .location_type(LocationType::Remote)
    .build()?;
```
```

#### **Day 5: Community Feedback**

- [ ] Post NIP draft to Nostr GitHub for review
- [ ] Share CLI tool in Nostr developer Discord/Telegram
- [ ] Post on Nostr using your tool (meta!)
- [ ] Gather feedback on usability
- [ ] Document issues and feature requests

---

## **Testing Strategy**

### **Unit Tests** (50+ tests)
- ✅ Data structure validation
- ✅ Tag parsing/serialization
- ✅ Salary range validation
- ✅ Date handling (timezones, expiration)
- ✅ Builder patterns

### **Integration Tests** (10+ tests)
- ✅ Post job to relay
- ✅ Retrieve job by ID
- ✅ Search with filters
- ✅ Update existing job
- ✅ Handle malformed events

### **Manual Testing Checklist**
- [ ] Post jobs to 3+ public relays
- [ ] Verify events appear in other Nostr clients
- [ ] Test on Windows, Mac, Linux
- [ ] Test with slow/unreliable network
- [ ] Verify PoW difficulty works
- [ ] Test with very large job descriptions (20KB+)

---

## **Relay Selection for POC**

**Recommended public relays**:
1. `wss://relay.damus.io` - General purpose, popular
2. `wss://relay.nostr.band` - Good for testing
3. `wss://nostr.wine` - Paid relay (for spam testing)
4. `wss://relay.snort.social` - Well-maintained

**Test plan**:
- Post same job to all 4 relays
- Verify consistency across relays
- Measure propagation time
- Test relay failures (disconnect mid-post)

---

## **Success Criteria**

**Phase 1 is complete when**:
- ✅ NIP spec is well-documented and reviewed
- ✅ Rust library compiles without warnings
- ✅ 90%+ test coverage on critical paths
- ✅ CLI tool successfully posts/retrieves jobs
- ✅ 20+ test jobs visible on public relays
- ✅ Documentation is clear for new users
- ✅ 2-3 community members have tested it

---

## **Risks & Mitigations**

| Risk | Impact | Mitigation |
|------|--------|------------|
| Nostr relays reject events | High | Test with multiple relays, implement retry logic |
| Event format incompatibility | Medium | Follow NIP-01 strictly, add schema version |
| Poor search performance | Medium | Document as known limitation, plan indexer for Phase 2 |
| Spam on test jobs | Low | Use PoW, monitor test job IDs |
| Community rejects NIP | High | Engage early, incorporate feedback iteratively |

---

## **Open Questions to Resolve**

1. **PoW difficulty**: What's reasonable? (20? 25? 30?)
2. **Job ID format**: UUID? Hash? Human-readable slug?
3. **Relay recommendations**: Should NIP specify preferred relays?
4. **Salary privacy**: Should min/max be optional to allow "competitive"?
5. **Multi-language**: Should `content` support multiple languages?

---

## **Deliverables Checklist**

**Code**:
- [ ] `nostr-jobs` library published to crates.io (or GitHub)
- [ ] `nostr-jobs-cli` binary working on Linux/Mac/Windows
- [ ] 100+ passing tests
- [ ] GitHub repo with MIT/Apache license

**Documentation**:
- [ ] `NIP-XX.md` (8-12 pages)
- [ ] `README.md` with examples
- [ ] `ARCHITECTURE.md` with diagrams
- [ ] Inline code documentation (rustdoc)
- [ ] 5+ example job JSON files

**Validation**:
- [ ] 20 test jobs posted to public relays
- [ ] Events verified in ≥2 Nostr clients (Damus, Amethyst)
- [ ] 3+ people have successfully run CLI
- [ ] GitHub Issues tracking feedback

---

## **After Phase 1**

If successful, you'll have:
1. **Proven concept** - Jobs can be published/discovered on Nostr
2. **Technical foundation** - Reusable library and spec
3. **Community signal** - Interest level from Nostr devs
4. **Decision point** - Continue to Phase 2 or pivot?

**Go/No-Go Decision Criteria**:
- Can post/retrieve jobs reliably? ✅ → Continue
- Community interest? ✅ → Continue  
- Performance acceptable? ✅ → Continue
- Fatal flaws discovered? ❌ → Pivot or abandon

Want me to create:
1. **Starter code templates** for the Rust library?
2. **Example job JSON files** for testing?
3. **GitHub issue templates** for the project?
4. **A Gantt chart or project board**?

Let's build this! 🚀
