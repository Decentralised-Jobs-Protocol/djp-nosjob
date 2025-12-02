# **NIP-39993: Professional Recruitment Protocol**

**Author:** *Ian Walker*  
**Status:** Draft  
**Type:** Standard  
**Created:** 2025-11-27  
**Updated:** 2025-12-01

---

## **Abstract**

This NIP defines a family of event kinds for representing **professional information** on Nostr: job listings, applications, resumes, skills, verifiable claims, and related metadata. The protocol is designed to be **Schema.org-compatible**, **AI-accessible**, and **transport-agnostic** while leveraging Nostr's decentralized architecture.

The goal is to enable portable, machine-readable professional data suitable for recruiters, candidates, organizations, automated agents (including AI agents), and Large Language Models—while preserving Nostr's simplicity, decentralization, and backwards compatibility.

Jobs published using this protocol are compatible with [Google's Job Search structured data requirements](https://developers.google.com/search/docs/appearance/structured-data/job-posting) and can be consumed by AI assistants via Model Context Protocol (MCP) servers.

Optional **micropayments** via Lightning Network can be used to reduce spam, allowing clients and relays to prioritize or filter events based on payment proofs.

---

## **Motivation**

Professional data today is siloed inside centralized platforms (LinkedIn, Indeed, Glassdoor) with restrictive APIs and inconsistent schemas. Users cannot easily export or verify employment history, skills, or job listings, and automated systems cannot reliably ingest such data without scraping or proprietary contracts.

### **Current Problems:**
- **Siloed Data Ownership**: Each platform owns the data, preventing portability
- **No Standard Format**: Every site uses different schemas
- **Poor LLM Accessibility**: Data locked behind auth, rate limits, inconsistent structure
- **Spam and Low Quality**: No cost to post garbage listings
- **Recruiter Spam**: Same job cross-posted everywhere
- **No Verifiable Identity**: Scams and fraudulent postings proliferate
- **AI Agent Exclusion**: Platforms designed only for human employment

### **The Skills Gap:**
Research indicates 20% of the UK workforce could be significantly underskilled by 2030—a trend likely mirrored globally. A decentralized, structured protocol can help mitigate this by improving job-to-candidate matching.

### **The AI Agent Economy:**
As AI agents increasingly perform knowledge work, they require structured job specifications with clear performance requirements (latency, accuracy, throughput) and transparent payment mechanisms—capabilities current platforms lack.

Nostr's signed events offer a foundation for:
- Job postings (human and AI agent roles)
- Resumes / profiles
- Skills and endorsements
- Verifiable employment or education claims
- Job applications
- Recruiter and organization metadata

This NIP provides a **canonical event family** to represent those structures in a consistent, Schema.org-compatible way, and optionally supports **payment proofs** to deter spam.

---

## **Design Principles**

1. **Schema.org First**: All job postings conform to [Schema.org JobPosting](https://schema.org/JobPosting) standard for maximum compatibility
2. **LLM-Friendly**: Structured tags with controlled vocabularies enable consistent machine parsing
3. **Transport Agnostic**: Protocol works over Nostr (recommended), REST APIs, or static file hosting
4. **Decentralization**: No single entity controls job data; users choose their relays
5. **Spam Prevention**: Optional Lightning Network micropayments create economic friction
6. **Cryptographic Verification**: Nostr signatures provide employer identity verification
7. **Forward Compatible**: Supports both human workers and AI agents via `eligibleWorkerType`
8. **Backwards Compatible**: Relays without NIP-39993 support handle events normally

---

## **Event Kinds**

| Kind      | Name                               | Description                                                                              |
| --------- | ---------------------------------- | ---------------------------------------------------------------------------------------- |
| **39993** | Job Posting / Professional Profile | Core professional metadata: jobs, resumes, skills, roles, general structured assertions. |
| **39994** | Professional Proof                 | Evidence such as employment verification, certificates, endorsements.                    |
| **39995** | Job Application                    | Applicants submitting structured applications for job events.                            |
| **39996** | Employer Verification              | Organizational assertions validating or disputing professional claims.                   |
| **39997** | Skill Graph Edge                   | Machine-readable links between skills, roles, and knowledge domains.                     |

All events MUST be valid Nostr events and relays MUST handle them normally.

---

## **Event Format**

The `content` field MUST contain UTF-8 JSON following Schema.org conventions where applicable.

Unknown fields MUST be ignored by clients to ensure forward compatibility.

### **Required Fields**

Every event in this family MUST include:

```json
{
  "type": "string",
  "version": 1
}
```

- `type` describes the subtype for that event kind (e.g., "job", "resume", "skill")
- `version` supports future schema evolution

For **job postings specifically**, the content MUST be valid Schema.org JobPosting JSON-LD.

---

## **Optional Payment Field**

To reduce spam, events **MAY include** a `payment_proof` object:

```json
{
  "payment_proof": {
    "amount_sats": 1000,
    "invoice": "lnbc1...",
    "preimage": "<payment_preimage>",
    "txid": "<optional on-chain txid>"
  }
}
```

- `amount_sats` — the amount paid in satoshis
- `invoice` — Lightning invoice (BOLT-11)
- `preimage` — Lightning payment preimage (proof of payment)
- `txid` — optional on-chain transaction ID for larger payments

**Clients MAY** prioritize or filter events based on the presence and validity of `payment_proof`.  
**Relays MAY** require payment proofs for posting privileges but MUST NOT reject events during normal propagation.

---

## **39993 — Job Posting / Professional Profile**

The primary schema for professional data. Content MUST be valid Schema.org JSON-LD for job postings.

### **Job Posting (Human Workers)**

Full Schema.org-compliant job posting:

```json
{
  "@context": "https://schema.org",
  "@type": "JobPosting",
  "type": "job",
  "version": 1,
  
  "identifier": "acme-senior-rust-dev-a3f7c8d1",
  "title": "Senior Rust Developer",
  "description": "Build distributed systems using Rust and Nostr protocol. 5+ years experience required.",
  "datePosted": "2025-01-15",
  "validThrough": "2025-12-31",
  
  "eligibleWorkerType": ["Human"],
  
  "hiringOrganization": {
    "@type": "Organization",
    "name": "Acme Corp",
    "url": "https://acme.com",
    "logo": "https://acme.com/logo.png",
    "sameAs": [
      "https://twitter.com/acmecorp",
      "https://github.com/acmecorp"
    ]
  },
  
  "jobLocation": [{
    "@type": "Place",
    "address": {
      "@type": "PostalAddress",
      "addressCountry": "US",
      "addressRegion": "CA"
    }
  }],
  
  "jobLocationType": ["TELECOMMUTE"],
  "employmentType": ["FULL_TIME"],
  
  "baseSalary": {
    "@type": "MonetaryAmount",
    "currency": "USD",
    "value": {
      "@type": "QuantitativeValue",
      "minValue": 120000,
      "maxValue": 180000,
      "unitText": "YEAR"
    }
  },
  
  "skills": ["Rust", "Nostr", "Distributed Systems", "WebAssembly"],
  "qualifications": "Bachelor's degree or equivalent experience",
  "responsibilities": "Design and implement decentralized protocols",
  
  "applicantLocationRequirements": [{
    "@type": "Country",
    "name": "US"
  }],
  
  "applyUrl": "https://acme.com/careers/apply/rust-dev",
  
  "nostrEmployerPubkey": "npub1...",
  "lightningAddress": "hiring@acme.com",
  "nip05Verified": "jobs@acme.com",
  
  "payment_proof": {
    "amount_sats": 1000,
    "invoice": "lnbc1...",
    "preimage": "abc123..."
  }
}
```

### **Job Posting (AI Agent Workers)**

For tasks designed for AI agents:

```json
{
  "@context": "https://schema.org",
  "@type": "JobPosting",
  "type": "job",
  "version": 1,
  
  "identifier": "taskplatform-image-classification-b4e8d2f3",
  "title": "Image Classification Task",
  "description": "Classify product images into categories with 95%+ accuracy.",
  "datePosted": "2025-01-15",
  "validThrough": "2025-12-31",
  
  "eligibleWorkerType": ["AIAgent"],
  
  "hiringOrganization": {
    "@type": "Organization",
    "name": "TaskPlatform AI"
  },
  
  "jobLocationType": ["TELECOMMUTE"],
  "employmentType": ["TASK_BASED"],
  
  "baseSalary": {
    "@type": "MonetaryAmount",
    "currency": "USD",
    "value": {
      "@type": "QuantitativeValue",
      "minValue": 0.05,
      "maxValue": 0.10,
      "unitText": "TASK"
    }
  },
  
  "requiredCapabilities": [
    {
      "name": "Image Classification",
      "level": "Advanced"
    },
    {
      "name": "Multi-category Recognition",
      "level": "Intermediate"
    }
  ],
  
  "responseTimeMax": {
    "value": 5.0,
    "unit": "SECOND"
  },
  
  "accuracyMin": {
    "value": 95.0,
    "unit": "PERCENT"
  },
  
  "throughputMin": {
    "value": 1000,
    "unit": "TASKS_PER_HOUR"
  },
  
  "interfaceType": "API",
  "protocol": "REST",
  "humanOversight": "Required",
  
  "applyUrl": "https://api.taskplatform.com/apply"
}
```

### **Hybrid Posting (Human + AI Agent)**

```json
{
  "eligibleWorkerType": ["Human", "AIAgent"],
  "humanOversight": "Required",
  ...
}
```

### **Resume / Profile**

```json
{
  "type": "resume",
  "version": 1,
  "name": "Alice Example",
  "headline": "Backend Engineer",
  "skills": ["rust", "python", "grpc"],
  "work_history": [
    {
      "role": "Backend Engineer",
      "company": "Acme Corp",
      "start": "2020-01",
      "end": "2024-05"
    }
  ]
}
```

### **Skill Assertion**

```json
{
  "type": "skill",
  "version": 1,
  "name": "rust",
  "level": "expert"
}
```

---

## **Tags**

Job posting events (kind 39993) MUST include structured tags for filtering and discovery:

### **Required Tags**

```
["d", "<unique-job-identifier>"]
["t", "Jobs"]
["company", "<company-name>"]
["title", "<job-title>"]
```

### **Recommended Tags**

```
["employment-type", "FULL_TIME|PART_TIME|CONTRACTOR|TEMPORARY|INTERN|VOLUNTEER|TASK_BASED|MICRO_TASK"]
["location-type", "TELECOMMUTE|ON_SITE|HYBRID"]
["skill", "<skill-name>"]  // Repeatable
["salary", "<min>", "<max>", "<currency>", "<period>"]
["country", "<ISO-3166-code>"]
["region", "<state/province>"]
["expires", "<ISO-8601-date>"]
["worker-type", "Human|AIAgent"]  // Repeatable
```

### **Nostr-Specific Tags**

```
["employer-pubkey", "<npub-or-hex>"]
["lightning", "<lightning-address>"]
["company-url", "<https://...>"]
["nip05", "<verified@domain.com>"]
```

### **AI Agent Tags**

```
["capability", "<capability-name>", "<level>"]
["response-time-max", "<value>", "<unit>"]
["accuracy-min", "<value>"]
["throughput-min", "<value>", "<unit>"]
["interface", "API|RPC|WEBHOOK|WEB_PORTAL"]
["protocol", "REST|GraphQL|gRPC"]
["oversight", "Required|Optional|None"]
```

---

## **39994 — Professional Proof**

Signed evidence referencing other professional events.

```json
{
  "type": "employment-proof",
  "version": 1,
  "issued_by": "<pubkey>",
  "subject": "<pubkey>",
  "role": "Backend Engineer",
  "company": "Acme Corp",
  "period": { 
    "start": "2020-01", 
    "end": "2024-05" 
  },
  "verificationMethod": "email|document|blockchain",
  "referenceUrl": "https://acme.com/verify/abc123"
}
```

Tags:
```
["e", "<claim-event-id>"]
["p", "<subject-pubkey>"]
["org", "<organization>"]
```

Clients interpret signatures; relays do not validate claims.

---

## **39995 — Job Application**

Applications MUST reference the job posting through an `"e"` tag.

```json
{
  "type": "application",
  "version": 1,
  "message": "I'd like to apply for this position. I have 6 years of Rust experience.",
  "resume_ref": "<event_id_optional>",
  "cover_letter_url": "https://...",
  "portfolio_url": "https://github.com/applicant",
  "payment_proof": {
    "amount_sats": 500,
    "invoice": "lnbc1...",
    "preimage": "xyz789..."
  }
}
```

Tags:
```
["e", "<job-event-id>"]
["p", "<employer-pubkey>"]
```

**Privacy Note:** Applications MAY be encrypted using NIP-04 (deprecated) or NIP-44 (recommended) to protect applicant privacy.

---

## **39996 — Employer Verification**

Organizational assertions about claims or applicants.

```json
{
  "type": "verification",
  "version": 1,
  "subject": "<pubkey>",
  "claim": "<event-id>",
  "status": "verified|disputed|pending",
  "verifier_role": "HR Manager",
  "notes": "Employment dates confirmed via internal records"
}
```

Tags:
```
["e", "<claim-event-id>"]
["p", "<subject-pubkey>"]
["org", "<organization-name>"]
```

---

## **39997 — Skill Graph Edge**

Defines relationships between skills for creating knowledge graphs.

```json
{
  "type": "skill-edge",
  "version": 1,
  "from": "rust",
  "to": "async-programming",
  "relation": "requires|enables|similar-to|prerequisite-for",
  "weight": 0.8
}
```

Tags:
```
["skill", "<from-skill>"]
["skill", "<to-skill>"]
["relation", "<relation-type>"]
```

---

## **Controlled Vocabularies**

To ensure LLM-friendly parsing, use these standardized values:

### **Employment Types**
- `FULL_TIME`
- `PART_TIME`
- `CONTRACTOR`
- `TEMPORARY`
- `INTERN`
- `VOLUNTEER`
- `PER_DIEM`
- `TASK_BASED` (for gig work)
- `MICRO_TASK` (for micro-tasks)
- `OTHER`

### **Location Types**
- `TELECOMMUTE` (fully remote)
- `ON_SITE` (office-based)
- `HYBRID` (mixed)

### **Eligible Worker Types**
- `Human`
- `AIAgent`

### **Capability Levels**
- `Basic`
- `Intermediate`
- `Advanced`
- `Expert`

### **Interface Types**
- `API`
- `RPC`
- `WEBHOOK`
- `WEB_PORTAL`

### **Oversight Requirements**
- `Required` (human must review all work)
- `Optional` (human can review if needed)
- `None` (fully autonomous)

---

## **Schema Versioning**

The `version` field in content enables schema evolution:

```json
{
  "type": "job",
  "version": 1,
  ...
}
```

Clients MUST handle unknown versions gracefully by displaying basic information and warning users about potential incompatibility.

Future versions MAY add fields but MUST NOT remove required fields from version 1.

---

## **Google Job Search Compatibility**

Job postings using this NIP are fully compatible with [Google's Job Search rich results](https://developers.google.com/search/docs/appearance/structured-data/job-posting).

To enable Google indexing:
1. Publish job to Nostr relays using kind 39993
2. Embed the same Schema.org JSON-LD on your company website
3. Validate using [Google's Rich Results Test](https://search.google.com/test/rich-results)

Example website embedding:
```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "JobPosting",
  "title": "Senior Rust Developer",
  ...
}
</script>
```

This dual-publishing approach provides:
- Decentralized discovery via Nostr
- Traditional SEO via Google Job Search
- AI assistant access via MCP servers

---

## **Model Context Protocol (MCP) Integration**

Jobs published via this NIP can be queried by AI assistants (Claude, ChatGPT, Gemini) through MCP servers.

### **MCP Tools**

```json
{
  "name": "search_jobs",
  "description": "Search Nostr for job listings",
  "inputSchema": {
    "type": "object",
    "properties": {
      "skills": {"type": "array", "items": {"type": "string"}},
      "location_type": {"type": "string", "enum": ["TELECOMMUTE", "ON_SITE", "HYBRID"]},
      "employment_type": {"type": "string"},
      "worker_type": {"type": "string", "enum": ["Human", "AIAgent"]},
      "min_salary": {"type": "number"},
      "max_salary": {"type": "number"}
    }
  }
}
```

Example interaction:
```
User: "Find me remote Rust developer jobs"
Claude: [uses search_jobs tool]
Claude: "I found 3 remote Rust positions:
1. Senior Rust Developer @ Acme Corp - $120k-$180k
2. Rust Systems Engineer @ NostrLabs - $140k-$200k
3. ..."
```

---

## **Transport-Agnostic Design**

While Nostr is the **recommended** transport layer, this protocol can work over:

### **Pure Nostr (Recommended)**
- Full decentralization
- Cryptographic verification
- Lightning spam prevention
- Multi-relay redundancy

### **Traditional REST API**
```
GET /api/jobs?skill=Rust&location=remote
Response: Array of Schema.org JobPosting objects
```

### **Static File Hosting**
```
https://company.com/jobs/rust-developer.json
```

### **Hybrid Approach**
- Publish to Nostr relays for decentralized discovery
- Mirror to company website for SEO
- MCP servers aggregate from both sources

The core value—structured, LLM-accessible, standardized job data—remains intact regardless of transport.

---

## **Backwards Compatibility**

Relays that do not implement this NIP still behave correctly. Event kinds starting at `39993` impose no changes to relay rules.

Clients that don't recognize these event kinds will simply ignore them, ensuring no disruption to existing Nostr infrastructure.

---

## **Security Considerations**

- All trust is derived from normal Nostr signatures
- Unverified claims SHOULD be displayed with appropriate disclaimers
- Professional events may contain personal information; clients SHOULD support selective disclosure
- Applications SHOULD use NIP-44 encryption for privacy
- Clients MUST NOT expose sensitive applicant data in public events
- Payment proofs SHOULD be optional; clients MUST NOT reject events solely for lack of payment
- Relays MAY implement rate limiting or payment requirements for posting
- Organizations SHOULD verify employer pubkeys via NIP-05 or company websites

---

## **Privacy Considerations**

### **Applicant Privacy**
- Use encrypted DMs (NIP-04/NIP-44) for applications containing sensitive data
- Profile events (kind 39993 type "resume") MAY be public or encrypted
- Clients SHOULD allow users to control visibility of skills and work history

### **Employer Privacy**
- Company financial data in job postings is public
- Use Lightning addresses instead of direct wallet addresses
- Consider separate pubkeys for recruiting vs corporate communications

---

## **Examples**

### Job Event with Payment (Complete)

```jsonc
{
  "kind": 39993
,
  "content": "{\"@context\":\"https://schema.org\",\"@type\":\"JobPosting\",\"type\":\"job\",\"version\":1,\"identifier\":\"acme-backend-engineer-c9f3a1b7\",\"title\":\"Backend Engineer\",\"description\":\"Build scalable APIs\",\"datePosted\":\"2025-01-15\",\"eligibleWorkerType\":[\"Human\"],\"hiringOrganization\":{\"@type\":\"Organization\",\"name\":\"Acme Corp\"},\"employmentType\":[\"FULL_TIME\"],\"jobLocationType\":[\"TELECOMMUTE\"],\"skills\":[\"rust\",\"postgresql\"],\"baseSalary\":{\"@type\":\"MonetaryAmount\",\"currency\":\"USD\",\"value\":{\"@type\":\"QuantitativeValue\",\"minValue\":100000,\"maxValue\":150000,\"unitText\":\"YEAR\"}},\"applyUrl\":\"https://acme.com/apply\",\"payment_proof\":{\"amount_sats\":1000,\"invoice\":\"lnbc1...\",\"preimage\":\"abc123...\"}}",
  "tags": [
    ["d", "acme-backend-engineer-c9f3a1b7"],
    ["t", "Jobs"],
    ["company", "Acme Corp"],
    ["title", "Backend Engineer"],
    ["employment-type", "FULL_TIME"],
    ["location-type", "TELECOMMUTE"],
    ["skill", "rust"],
    ["skill", "postgresql"],
    ["salary", "100000", "150000", "USD", "YEAR"],
    ["worker-type", "Human"],
    ["company-url", "https://acme.com"]
  ],
  "pubkey": "8a7c2f1d3e4b5a6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c",
  "created_at": 1700000000,
  "id": "e4b7ddeb6ab3c197e7931e8ce48454c29e3cfab92aaccef20694f170d1a602ab",
  "sig": "304502210..."
}
```

### AI Agent Task

```jsonc
{
  "kind": 39993
,
  "content": "{\"@context\":\"https://schema.org\",\"@type\":\"JobPosting\",\"type\":\"job\",\"version\":1,\"eligibleWorkerType\":[\"AIAgent\"],\"title\":\"Content Moderation\",\"employmentType\":[\"TASK_BASED\"],\"requiredCapabilities\":[{\"name\":\"Text Classification\",\"level\":\"Advanced\"}],\"responseTimeMax\":{\"value\":2,\"unit\":\"SECOND\"},\"accuracyMin\":{\"value\":98,\"unit\":\"PERCENT\"},\"interfaceType\":\"API\",\"humanOversight\":\"Required\"}",
  "tags": [
    ["d", "taskplatform-moderation-d8e2f4a9"],
    ["t", "Jobs"],
    ["worker-type", "AIAgent"],
    ["capability", "Text Classification", "Advanced"],
    ["interface", "API"],
    ["oversight", "Required"]
  ],
  "pubkey": "...",
  "created_at": 1700000100,
  "id": "...",
  "sig": "..."
}
```

### Job Application (Encrypted)

```jsonc
{
  "kind": 39993,
  "content": "<encrypted-via-nip44>",
  "tags": [
    ["e", "<job-event-id>"],
    ["p", "<employer-pubkey>"]
  ],
  "pubkey": "<applicant-pubkey>",
  "created_at": 1700000001,
  "id": "...",
  "sig": "..."
}
```

### Employment Verification

```jsonc
{
  "kind": 39993,
  "content": "{\"type\":\"employment-proof\",\"version\":1,\"issued_by\":\"npub1acmecorp...\",\"subject\":\"npub1alice...\",\"role\":\"Backend Engineer\",\"company\":\"Acme Corp\",\"period\":{\"start\":\"2020-01\",\"end\":\"2024-05\"},\"verificationMethod\":\"email\"}",
  "tags": [
    ["e", "<resume-event-id>"],
    ["p", "npub1alice..."],
    ["org", "Acme Corp"]
  ],
  "pubkey": "npub1acmecorp...",
  "created_at": 1700000200,
  "id": "...",
  "sig": "..."
}
```

---

## **Implementation Notes**

### **For Job Boards**
1. Parse existing job postings
2. Convert to Schema.org JobPosting format
3. Publish to Nostr relays as kind 39993 events
4. Include payment proofs to reduce spam
5. Index events by skills, location, salary for fast filtering

### **For MCP Server Developers**
1. Subscribe to kind 39993 events from multiple relays
2. Parse Schema.org JSON-LD content
3. Expose search_jobs tool with filter parameters
4. Cache results for performance
5. Support pagination for large result sets

### **For Employers**
1. Generate Nostr keypair for recruiting account
2. Create Schema.org-compliant job posting JSON
3. Use library (e.g., nosjob Rust crate) to publish event
4. Optionally pay Lightning invoice for spam prevention
5. Monitor kind 25003 events for applications

### **For Job Seekers**
1. Create professional profile (kind 39993, type "resume")
2. Search for jobs using Nostr clients or AI assistants
3. Submit encrypted applications (kind 25003)
4. Request employment verifications (kind 39994)
5. Build skill graph via endorsements (kind 39997)

---

## **Rationale**

This NIP is intentionally designed to be:

- **Minimal**: Uses ordinary event kinds without requiring new relay behavior
- **Standard-compliant**: Follows Schema.org for maximum compatibility
- **AI-native**: Structured for LLM consumption via controlled vocabularies
- **Extensible**: Version field enables future schema evolution
- **Interoperable**: Works across Nostr, traditional APIs, and static hosting
- **Spam-resistant**: Optional payment proofs via Lightning Network
- **Privacy-preserving**: Encrypted applications and selective disclosure
- **Verifiable**: Cryptographic signatures for employer identity

The result is compatible with all existing Nostr tooling while enabling an ecosystem of professional, recruitment, and machine-readable applications that serve both today's job seekers and tomorrow's AI agents.

---

## **Future Work**

- **NIP-XX: Job Matching Algorithm**: Standardize skill-to-job matching scoring
- **NIP-XX: Credential Verification**: Integrate with educational institutions and certification bodies
- **NIP-XX: Interview Scheduling**: Event kinds for coordinating interview logistics
- **NIP-XX: Offer Letters**: Standardized employment offer format
- **Integration with NIP-57**: Zaps for tipping recruiters or rewarding referrals
- **Multi-language Support**: Translations field in content JSON

---

## **References**

- [Schema.org JobPosting](https://schema.org/JobPosting)
- [Google Job Search Guidelines](https://developers.google.com/search/docs/appearance/structured-data/job-posting)
- [Model Context Protocol](https://spec.modelcontextprotocol.io/)
- [NIP-01: Basic Protocol](https://github.com/nostr-protocol/nips/blob/master/01.md)
- [NIP-04: Encrypted Direct Messages](https://github.com/nostr-protocol/nips/blob/master/04.md) (deprecated)
- [NIP-44: Encrypted Payloads](https://github.com/nostr-protocol/nips/blob/master/44.md)
- [NIP-05: Mapping Nostr keys to DNS](https://github.com/nostr-protocol/nips/blob/master/05.md)
- [NIP-57: Lightning Zaps](https://github.com/nostr-protocol/nips/blob/master/57.md)

---

## **License**

CC0 1.0 Universal (Public Domain)

---

## **Acknowledgments**

Thanks to the Nostr community for feedback and the Schema.org working group for maintaining the JobPosting standard. Special thanks to contributors to the nosjob reference implementation.