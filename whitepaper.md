# Decentralised Jobs Protocol (NIP-XX)
## A Schema.org-Compatible, AI-Accessible Job Marketplace on Nostr

**Version:** 1.0  
**Date:** December 2025  
**Status:** Draft Proposal  
**Authors:** Ian Walker  

---

## Abstract

The Decentralised Jobs Protocol (Nosjob) introduces a novel approach to job listings by leveraging Nostr's decentralized infrastructure to create an open, spam-resistant, and AI-accessible employment marketplace. By combining Schema.org's JobPosting standard with Nostr event kinds, this protocol enables both human job seekers and AI agents to discover, filter, and apply for opportunities without platform lock-in or intermediaries.

This whitepaper proposes **NIP-XX** (Nostr Implementation Possibility) for standardizing job-related events (Kinds 39993-39997) and demonstrates compatibility with Google's Job Search structured data requirements through optional JSON-LD embedding.

---

## 1. Problem Statement

### 1.1 Current Market Failures

The global recruitment industry suffers from fundamental structural problems:

- **Siloed Data Ownership**: Platforms like LinkedIn, Indeed, and Glassdoor create walled gardens where job data is proprietary and portable only at the platform's discretion
- **Inconsistent Schemas**: Each platform implements its own data structure, making aggregation and automated processing difficult
- **Poor Machine Accessibility**: Job listings are locked behind authentication, rate limits, and anti-scraping measures, preventing AI-powered matching
- **High Spam & Low Signal**: Free posting leads to duplicate listings, recruiter spam, and low-quality opportunities
- **No Verifiable Employer Identity**: Scams and fraudulent postings proliferate due to lack of cryptographic verification
- **Growing Skills Gap**: Research indicates 20% of the UK workforce could be significantly underskilled by 2030, a trend likely mirrored globally

### 1.2 The AI Agent Economy

As AI agents increasingly perform knowledge work tasks, they require:
- Structured, machine-readable job specifications
- Clear performance requirements (latency, accuracy, throughput)
- Transparent payment mechanisms (per-task, per-token pricing)
- Verifiable completion criteria

Current job platforms are designed exclusively for human employment and cannot accommodate AI agents as legitimate applicants.

---

## 2. Solution: Nostr-Based Job Protocol

### 2.1 Core Design Principles

**Decentralization**: No single entity controls job data. Employers publish to any Nostr relay, and job seekers query any relay federation.

**Structured Data First**: All job postings conform to Schema.org's JobPosting standard, ensuring compatibility with existing SEO tools and search engines.

**LLM-Friendly Design**: Controlled vocabularies and structured tags enable consistent parsing by Large Language Models without natural language processing overhead.

**Spam Prevention**: Optional relay-level fees for posting (paid via Lightning Network) create economic friction against spam while remaining accessible.

**Cryptographic Verification**: Nostr's keypair system provides built-in employer identity verification through public key infrastructure.

**Forward Compatibility**: Protocol supports both human workers and AI agents through the `eligibleWorkerType` field.

### 2.2 Protocol Architecture

The protocol defines five event kinds:

| Kind  | Purpose                                        | Status      |
|-------|------------------------------------------------|-------------|
| 39993 | Job Posting / Listing                          | **Proposed**|
| 39994 | Job Application Submission                     | Future      |
| 39995 | Professional Profile / Credential Verification | Future      |
| 39996 | Employer Verification / Claims                 | Future      |
| 39997 | Skill Graph / Endorsements                     | Future      |

This whitepaper focuses on **Kind 39993** (Job Postings) as the foundation.

### 2.3 Nostr-Optional Design

**While Nostr is the recommended implementation**, the protocol's Schema.org-first approach ensures it can operate independently:

**Standalone Deployment:**
- Any HTTP server can host Schema.org JobPosting JSON files
- MCP servers can query traditional REST APIs instead of Nostr relays
- Employers maintain job listings on their own infrastructure

**Hybrid Approach:**
- Organizations can publish to both Nostr relays AND their own servers
- MCP tools aggregate from multiple sources (Nostr + traditional APIs)
- Legacy job boards can expose Schema.org endpoints for compatibility

**Migration Path:**
- Existing platforms can adopt the Schema.org structure without Nostr
- Add Nostr publication as an optional enhancement
- Gradually transition to decentralized infrastructure

The core value proposition—structured, LLM-accessible, standardized job data—remains intact regardless of transport layer. Nostr provides optimal decentralization, censorship resistance, and cryptographic verification, but the protocol gracefully degrades to work with traditional client-server architectures.

---

## 3. Technical Specification: Kind 39993

### 3.1 Event Structure

```json
{
  "id": "<event_id>",
  "pubkey": "<employer_public_key>",
  "created_at": <unix_timestamp>,
  "kind": 39993,
  "tags": [
    ["d", "<unique_job_identifier>"],
    ["t", "Jobs"],
    ["company", "<company_name>"],
    ["title", "<job_title>"],
    ["employment-type", "FULL_TIME"],
    ["location-type", "TELECOMMUTE"],
    ["skill", "Rust"],
    ["skill", "Nostr"],
    ["salary", "120000", "180000", "USD", "YEAR"],
    ["expires", "2025-12-31"]
  ],
  "content": "<schema.org_json_ld_or_markdown>",
  "sig": "<signature>"
}
```

### 3.2 Required Tags

- **`d` (identifier)**: Unique job ID, formatted as `{company-slug}-{title-slug}-{uuid}`
- **`t` (topic)**: Always set to "Jobs" for discoverability
- **`company`**: Hiring organization name
- **`title`**: Job position title

### 3.3 Recommended Tags

- **`employment-type`**: One of `FULL_TIME`, `PART_TIME`, `CONTRACTOR`, `TEMPORARY`, `INTERN`, `VOLUNTEER`, `TASK_BASED`, `MICRO_TASK`
- **`location-type`**: One of `TELECOMMUTE`, `ON_SITE`, `HYBRID`
- **`skill`**: Repeatable tag for required skills
- **`salary`**: Four-part tag `[min, max, currency, period]`
- **`country`**, **`region`**: Geographic filters
- **`expires`**: ISO 8601 date for listing expiration

### 3.4 Content Field Options

The `content` field supports two formats:

**Option 1: Full Schema.org JSON-LD** (Google-compatible)
```json
{
  "@context": "https://schema.org",
  "@type": "JobPosting",
  "title": "Senior Rust Developer",
  "description": "Build decentralized systems...",
  "datePosted": "2025-01-15",
  "validThrough": "2025-12-31",
  "employmentType": "FULL_TIME",
  "hiringOrganization": {
    "@type": "Organization",
    "name": "Acme Corp",
    "url": "https://acme.com"
  },
  "jobLocation": {
    "@type": "Place",
    "address": {
      "@type": "PostalAddress",
      "addressCountry": "US"
    }
  },
  "baseSalary": {
    "@type": "MonetaryAmount",
    "currency": "USD",
    "value": {
      "@type": "QuantitativeValue",
      "minValue": 120000,
      "maxValue": 180000,
      "unitText": "YEAR"
    }
  }
}
```

**Option 2: Markdown + Structured Tags** (Human-readable)
```markdown
# Senior Rust Developer

Build decentralized systems using Nostr protocol...

## Requirements
- 5+ years Rust experience
- Distributed systems knowledge
- Open source contributions preferred

## Benefits
- Remote-first
- Competitive salary
- Equity options
```

### 3.5 AI Agent Extensions

For AI-eligible positions, additional fields specify technical requirements:

```json
{
  "eligibleWorkerType": ["AIAgent"],
  "requiredCapabilities": [
    {"name": "Image Classification", "level": "Advanced"},
    {"name": "Multi-language NLP", "level": "Intermediate"}
  ],
  "responseTimeMax": {"value": 5.0, "unit": "SECOND"},
  "accuracyMin": {"value": 95.0, "unit": "PERCENT"},
  "throughputMin": {"value": 1000, "unit": "REQUESTS_PER_HOUR"},
  "interfaceType": "API",
  "protocol": "REST",
  "humanOversight": "Required"
}
```

---

## 4. Schema.org Compatibility

### 4.1 Google Job Search Integration

By embedding Schema.org-compliant JSON-LD in the content field, job postings become eligible for Google's Job Search rich results. Employers can:

1. Publish job to Nostr relays (decentralized)
2. Embed the same event on their website
3. Validate with [Google's Rich Results Test](https://search.google.com/test/rich-results)

This dual-compatibility allows organizations to benefit from both decentralized discovery and traditional SEO.

### 4.2 Required Schema.org Fields

Per [Google's guidelines](https://developers.google.com/search/docs/appearance/structured-data/job-posting), the protocol ensures:

- ✅ `title` (position name)
- ✅ `description` (full job details)
- ✅ `datePosted` (ISO 8601 format)
- ✅ `hiringOrganization` (company details)
- ✅ `jobLocation` or `jobLocationType` (remote/hybrid/onsite)

Optional but recommended:
- `validThrough` (expiration date)
- `baseSalary` (compensation range)
- `employmentType` (full-time, part-time, etc.)

---

## 5. Model Context Protocol (MCP) Integration

### 5.1 AI-Native Job Search

The protocol includes an **MCP Server** that exposes job data to AI assistants (Claude, ChatGPT, etc.) via standardized tools:

```json
{
  "name": "search_jobs",
  "description": "Search Nostr for job listings",
  "inputSchema": {
    "type": "object",
    "properties": {
      "skills": {"type": "array"},
      "location_type": {"type": "string"},
      "employment_type": {"type": "string"}
    }
  }
}
```

### 5.2 Example: AI Agent Application

```bash
$ goose "Find me Rust developer jobs"

> Using MCP tool: search_jobs({skills: ["Rust"]})
> Found 3 results:
> 1. Senior Rust Developer @ Nostr Labs - Remote - $120k-$180k
```

This enables:
- Natural language job search through AI assistants
- Automatic skill matching based on user profiles
- Conversational application workflows

---

## 6. Economic Model & Spam Prevention

### 6.1 Lightning Network Integration

Relays may require small Lightning payments for posting jobs:
- **Micro-fee per post**: 100-1000 sats (~$0.03-$0.30)
- **Subscription model**: Monthly relay access for recruiters
- **Stake-based priority**: Higher-paying posts appear first in searches

### 6.2 Reputation Through NIP-05

Employers can verify their identity using NIP-05 (internet identifier):
```json
{
  "nip05Verified": "jobs@acmecorp.com",
  "lightningAddress": "hiring@acmecorp.com"
}
```

This creates a web-of-trust where verified employers build reputation over time.

---

## 7. Implementation: Rust Library & MCP Server

### 7.1 Core Library (`nosjob`)

The reference implementation provides:

```rust
use nosjob::*;

let job = JobListing::builder(
    "job-001".to_string(),
    "Senior Rust Developer".to_string(),
    "Build decentralized systems".to_string(),
    "Acme Corp".to_string(),
    "2025-01-15".to_string(),
    "https://acme.com/apply".to_string(),
)
.for_humans()
.employment_type(vec![EmploymentType::FullTime])
.remote()
.salary(120000.0, 180000.0, "USD".to_string(), "YEAR".to_string())
.skills(vec!["Rust".to_string(), "Nostr".to_string()])
.build()?;

let event = job.to_nostr_event(&keys)?;
client.send_event(&event).await?;
```

### 7.2 MCP Server

The MCP server exposes three tools:
- `search_jobs`: Query Nostr relays with filters
- `get_job_details`: Fetch full Schema.org data for a specific listing
- `apply_to_job`: Submit encrypted application (future)

---

## 8. Privacy & Security Considerations

### 8.1 Applicant Privacy

- Applications use **NIP-04 encrypted DMs** or external URLs
- No personally identifiable information in public events
- Candidates control which skills/experience to disclose

### 8.2 Employer Verification

- Public key signatures prevent impersonation
- NIP-05 verification links Nostr identity to domain ownership
- Relay operators can require additional KYC for posting privileges

### 8.3 Content Moderation

- Individual relays set content policies
- Users choose relays aligned with their values
- No global censorship or algorithmic bias

---

## 9. Comparison with Existing Solutions

| Feature | LinkedIn | Indeed | Nosjob (Nostr) | Nosjob (Legacy) |
|---------|----------|--------|----------------|-----------------|
| **Data Ownership** | Platform | Platform | User | Self-hosted |
| **Standard Format** | ❌ | ❌ | ✅ Schema.org | ✅ Schema.org |
| **AI-Accessible** | Rate-limited | Rate-limited | ✅ Open | ✅ API-based |
| **Spam Prevention** | Algorithmic | Algorithmic | Economic | Traditional |
| **Employer Verification** | Manual | Manual | Cryptographic | Domain-based |
| **Supports AI Agents** | ❌ | ❌ | ✅ | ✅ |
| **Censorship Resistance** | ❌ | ❌ | ✅ | ⚠️ Depends |
| **Decentralization** | ❌ | ❌ | ✅ Full | ⚠️ Federated |

---

## 10. Legacy System Integration & Migration Path

### 10.1 The Nostr-Optional Advantage

The protocol's **Schema.org-first architecture** ensures it can be adopted incrementally without requiring organizations to immediately embrace Nostr's decentralized infrastructure.

### 10.2 Implementation Options

**Option 1: Pure Nostr (Recommended)**
- Full decentralization and censorship resistance
- Cryptographic employer verification via keypairs
- Lightning Network spam prevention
- Multi-relay redundancy

**Option 2: Traditional REST API**
```
GET /api/jobs?skill=Rust&location=remote
Response: Array of Schema.org JobPosting objects
```
- MCP servers query HTTP endpoints instead of Nostr relays
- Standard authentication (API keys, OAuth)
- Familiar infrastructure for enterprises

**Option 3: Hybrid Deployment**
- Publish to Nostr relays for decentralized discovery
- Mirror to company website for SEO and traditional access
- MCP tools aggregate from both sources
- Best of both worlds: reach + resilience

**Option 4: Federation Model**
- Multiple organizations run compatible job servers
- Cross-server discovery via shared Schema.org format
- Similar to Mastodon's ActivityPub federation
- Each organization maintains control while enabling interoperability

### 10.3 Migration Scenarios

**For Existing Job Boards:**
```
Phase 1: Expose Schema.org JSON endpoints
Phase 2: Implement MCP server for AI access
Phase 3: Add Nostr relay publishing (optional)
Phase 4: Deprecate proprietary formats
```

**For Enterprises:**
```
Phase 1: Post jobs as Schema.org JSON on careers page
Phase 2: Enable Google Job Search rich results
Phase 3: Publish same data to Nostr relays
Phase 4: Accept applications via Nostr DMs (optional)
```

**For Startups:**
```
Phase 1: Start with pure Nostr implementation
Phase 2: Gain community adoption and feedback
Phase 3: Bridge to legacy systems if needed
```

### 10.4 Protocol-Agnostic MCP Server

The MCP implementation can support multiple backends:

```rust
// MCP Server Configuration
{
  "sources": [
    {
      "type": "nostr",
      "relays": ["wss://relay.damus.io", "wss://jobs.nostr.directory"]
    },
    {
      "type": "http",
      "endpoints": ["https://acmecorp.com/api/jobs"]
    },
    {
      "type": "file",
      "path": "/var/jobs/*.json"
    }
  ]
}
```

AI assistants (Claude, ChatGPT) don't need to know the underlying transport—they just query for Schema.org-compliant job data.

### 10.5 Why Nostr Still Matters

Even with legacy fallbacks, Nostr provides unique benefits:

1. **No Platform Risk**: Employers aren't dependent on any company's API stability or pricing
2. **Permanent Archive**: Jobs published to relays become part of a distributed permanent record
3. **Zero Vendor Lock-in**: Switch relays without losing data or changing formats
4. **Cryptographic Trust**: Public key verification eliminates impersonation
5. **Lightning Payments**: Enables micropayment-based spam prevention without payment processors

### 10.6 Interoperability Examples

**Scenario A: Legacy Job Board Adoption**
```
Indeed.com could:
1. Keep existing web UI and database
2. Add Schema.org JSON export endpoint
3. Allow MCP servers to query their API
4. Gain AI assistant compatibility overnight
```

**Scenario B: Company Career Pages**
```html
<!-- Existing career page -->
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "JobPosting",
  "title": "Senior Developer",
  ...
}
</script>
```
This same JSON can be:
- Indexed by Google Job Search
- Published to Nostr relays
- Queried by MCP servers
- Archived on IPFS

**Scenario C: Recruiter Tools**
```
Greenhouse/Lever could:
1. Add "Publish to Nostr" checkbox in job posting UI
2. Generate Schema.org JSON automatically
3. Submit to configured Nostr relays
4. Track applications from both Nostr and traditional sources
```

### 10.7 The Network Effect

As more systems adopt Schema.org JobPosting format:
- Job aggregators work seamlessly across sources
- AI assistants provide unified search experience
- Candidates benefit from complete market visibility
- Employers reach maximum audience with single format

The protocol creates **positive-sum interoperability**: adopting the standard benefits everyone, regardless of whether they use Nostr.

---

## 10. Future Extensions (Kinds 39994-39997)

### Kind 39994: Job Applications
Encrypted applications referencing job ID, candidate pubkey, and resume hash.

### Kind 39995: Professional Profiles
Portable LinkedIn-style profiles with skill assertions and work history.

### Kind 39996: Employer Claims
Company verification events signed by domain owners.

### Kind 39997: Skill Endorsements
Peer-to-peer skill verification creating a decentralized reputation graph.

---

## 11. Adoption Roadmap

### Phase 1: MVP (Q1 2025)
- ✅ Kind 39993 specification
- ✅ Rust library with Schema.org support
- ✅ MCP server for AI integration
- 🔄 Community feedback & iteration

### Phase 2: Ecosystem Growth (Q2-Q3 2025)
- Web client for browsing/posting jobs
- Mobile apps (iOS/Android)
- Relay operator toolkit
- Integration with existing job boards

### Phase 3: Advanced Features (Q4 2025)
- Kinds 39994-39997 implementation
- Lightning payment flows
- AI agent application workflows
- Multi-language support

---

## 12. Call to Action

### For Nostr Community
We propose this as **NIP-XX** for community review. Feedback requested on:
- Event kind number assignment (39993-39997)
- Tag standardization
- Relay requirements

### For Developers
- Contribute to [nosjob repository](https://github.com/yourusername/nosjob)
- Build MCP servers for other AI platforms
- Create UI clients for job seekers

### For Employers
- Test posting jobs to Nostr relays
- Validate Schema.org compatibility with Google
- Provide feedback on missing features

---

## 13. Conclusion

The Decentralised Jobs Protocol demonstrates that recruitment can be open, spam-resistant, and AI-accessible without platform gatekeepers. By building on Nostr's proven architecture and adhering to Schema.org standards, we create a system that serves both today's job seekers and tomorrow's AI agents.

This is not just a job board—it's infrastructure for the future of work.

---

## Appendices

### Appendix A: Complete Event Example

```json
{
  "id": "e4b7ddeb6ab3c197e7931e8ce48454c29e3cfab92aaccef20694f170d1a602ab",
  "pubkey": "8a7c2f1d3e4b5a6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c",
  "created_at": 1732982433,
  "kind": 39993,
  "tags": [
    ["d", "acme-corp-senior-rust-developer-a3f7c8d1"],
    ["t", "Jobs"],
    ["company", "Acme Corp"],
    ["job-id", "job-001"],
    ["title", "Senior Rust Developer"],
    ["employment-type", "FULL_TIME"],
    ["location-type", "TELECOMMUTE"],
    ["skill", "Rust"],
    ["skill", "Nostr"],
    ["skill", "Distributed Systems"],
    ["salary", "120000", "180000", "USD", "YEAR"],
    ["country", "US"],
    ["expires", "2025-12-31"],
    ["company-url", "https://acme.com"],
    ["lightning", "hiring@acme.com"]
  ],
  "content": "{\"@context\":\"https://schema.org\",\"@type\":\"JobPosting\",...}",
  "sig": "3045022100ab3f7c8d1e4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b..."
}
```

### Appendix B: Legacy System Compatibility Matrix

| System Type | Integration Method | Effort | Benefits |
|-------------|-------------------|--------|----------|
| **Modern Job Boards** | REST API + Schema.org | Low | AI access, SEO boost |
| **ATS Platforms** | Export plugin | Medium | Multi-channel posting |
| **Company Career Pages** | JSON-LD embed | Low | Google indexing, Nostr optional |
| **Recruiter CRMs** | Nostr relay connector | Medium | Decentralized reach |
| **Legacy HRMS** | File export + cron job | High | Gradual modernization |

### Appendix C: Relay Recommendations

**General Purpose:**
- wss://relay.damus.io
- wss://relay.nostr.band
- wss://nostr.wine

**Specialized (Jobs):**
- wss://jobs.nostr.directory (proposed)
- wss://work.nostr.social (proposed)

### Appendix D: License

This specification is released under **CC0 1.0 Universal** (Public Domain).
The reference implementation is licensed under **MIT License**.




