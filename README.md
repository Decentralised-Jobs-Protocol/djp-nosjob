# Decentralised Jobs Protocol
![Generated Image November 27, 2025 - 1_12PM](https://github.com/user-attachments/assets/e40bda02-3a0f-4172-89f8-a16803112d3d)

## Better Job Listings for humans and AI

#### Structured. LLM friendly. Searchable.

Job searches suck. Recruiters beware.

Let's make jobs accessible to the whole world and to AI. Yes do it for AI as well, it's inevitable, so let's make it structured before it gets chaotic.

## Problem

Matching employers with candidates should be simpler for both parties, more trustworthy, and cheaper

At present the legacy systems of posting and hosting vacancies is hit-and-miss and is not future proof or viably capable of automation.

Current issues with recruitment listings:

- Siloed data - each platform owns the data (LinkedIn, Indeed, etc.) 
- No standard format - each site has different schemas Poor LLM accessibility 
- locked behind auth, rate limits, inconsistent structure Spam and low quality,
- no cost to post garbage Recruiter spam
- same job cross-posted everywhere No verifiable employer identity

20% of the UK workforce could be significantly underskilled for their jobs by 2030, according to research from the Industrial Skills Strategy Council.

This issue is likely common across the world. A decentralised jobs protocol is needed.

Google have a protocol that we can build upon, to maintain compatibility with existing systems.

https://schema.org/JobPosting

https://search.google.com/test/rich-results

https://developers.google.com/search/docs/appearance/structured-data/job-posting

## Solution

An open protocol that enables everyone including AI to search for jobs. The goal is to offer a much improved job hunting experience and help mitigate the infamous "skills gap".

This protocol includes forward compatibilty for the hybrid world of the near future where AI Agents are also considered for tasks in addition to humans.
Rather than a fee or salary, the 'Agent' will require payment for tokens used and power consumed.

A decentralized architecture is required, not a walled garden. Nostr already provides all of the required capabilities but lacks a widely agreed protocol for job listing 'events'.

*Nostr is an apolitical communication commons. A simple standard that defines a scalable architecture of clients and servers that can be used to spread information freely* ~ https://nostr.com/

I propose the using an MCP Server to query Nostr, which can be used by "AI" Foundation models, allowing systems like "Claude" and "ChatGPT" to fetch accurate, up to date, and relevant job postings extracted from Nostr.

To support Nostr, a fee could be paid to post jobs, this would fund the relays and increase wider adoption of Nostr.
Paying to post a job to Nostr would also stop SPAM.

### Is it essential to use Nostr?
No, although it is arguably the optimum solution, jobs using the schema could be sent to any server(s) but the decentralised nature of Nostr makes the concept very robust and open.



https://primal.net/e/nevent1qqsw97qflxpfwr7d0arml30pmjx3amkg4sy3twsv4aauq7wcz4l53tcv6vahz

<img width="698" height="417" alt="image" src="https://github.com/user-attachments/assets/204510d6-94a9-480a-85c6-81e506fdbeab" />



## Concept based on Nostr 'events'

| Kind  | Purpose                                        |
| ----- | ---------------------------------------------- |
| 39993 | Job Posting / Professional Profile Assertion   |
| 39994 | Proof / credential / professional verification |
| 39995 | Job Application submission                     |
| 39996 | Employer verification / claims                 |
| 39997 | Skill graph / connections / endorsements       |


## Usage

JSON file support - Reads from job.json by default, or specify a custom file.

Dry run by default - Does not publish unless you explicitly use --publish or -p flag

Auto-generated job ID - If not specified in JSON, creates one from company + title

Better output - Cleaner formatting showing all job details, event info, and tags

Simple JSON format - Uses your simple JSON format and converts it to the full Schema.org structure

## Examples

```bash
    cargo run -- example-job-39993.json --publish
```
```bash
    cargo run -- example view_jobs
```

```bash
    cargo run -- view_jobs
```

```bash
    cargo run --example view_jobs
   Compiling nosjob v0.1.0 (/home/pop/rust/nosjob)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.61s
     Running `target/debug/examples/view_jobs`
🔍 Nostr Job Viewer (Kind 39993)

📡 Connecting to relays...
   • wss://relay.damus.io
   • wss://relay.nostr.band
   • wss://nos.lol

🔎 Searching for job listings...

✅ Found 3 job listing(s)

================================================================================

📌 Job #1
   Event ID: e4b7ddeb6ab3c197e7931e8ce48454c29e3cfab92aaccef20694f170d1a602ab
   Posted: 2025-11-30T15:00:33Z
   Pubkey: npub1j2xq0k3l7vkhzffvwmxcwaj29ehh00rvtlr4vr0lxsfzetrvfk7se9me2g
   📝 Title: Content Mod
```
## MCP server tested via MCP Inspector
<img width="1919" height="967" alt="ss2" src="https://github.com/user-attachments/assets/38231f2c-81ab-4a06-b2d1-58a4db39bf7d" />

## MCP server - succesful retrieval of results via Gemini and Goose CLI 
<img width="1920" height="1080" alt="ss3" src="https://github.com/user-attachments/assets/dc0772f0-62d8-4557-a607-d4b369b394f7" />

## Asking goose to use the MCP to find suitable job for a Rust developer..

```bash
Context: ○○○○○○○○○○ 0% (3226/1000000 tokens)
( O)> use your jobmcp and fetch latest listings

─── search_jobs | jobmcp ──────────────────────────

Of course! I found the following recent job listings:

1.  **🏢 Boogle AI** - Content Moderation Agent
    *   **📍 Location:** Telecommute
    *   **💼 Type:** Other
    *   **🛠️ Skills:** Natural Language Processing, Image Classification, Multi-language Support

2.  **🏢 Fictional Tech Co.** - Web Developer
    *   **📍 Location:** Telecommute
    *   **💼 Type:** Full-time
    *   **🛠️ Skills:** JavaScript, TypeScript, React, Node.js, CSS, HTML

3.  **🏢 Nostr Labs** - Senior Rust Developer
    *   **📍 Location:** Telecommute
    *   **💼 Type:** Full-time
    *   **🛠️ Skills:** Rust, Nostr, Distributed Systems, WebAssembly

Given that you are a Rust developer, the **Senior Rust Developer** position at **Nostr Labs** seems like a perfect fit!

Would you like me to get more details about this specific job?

⏱️  Elapsed time: 8.63s
Context: ○○○○○○○○○○ 0% (3682/1000000 tokens)
( O)> Press Enter to send, Ctrl-J for new line

```

<img width="1384" height="771" alt="Screenshot from 2025-12-01 12-10-27" src="https://github.com/user-attachments/assets/c9cff144-3ba5-443e-8ecb-06bad5979e20" />


---

This provides a detailed framework for achieving **LLM-Friendly data design** by integrating **structured tags and controlled vocabularies** into a decentralized protocol specification (Nosjob, NIP-XX), addressing the current challenges posed by inconsistent data formats.

### The Need for Structured Data and LLM Accessibility

The push for structured data design stems from recognizing critical issues in traditional recruitment listings, primarily **poor LLM accessibility**. This inaccessibility is due to job listings being locked behind authentication and rate limits, and most importantly, having an **inconsistent structure**. Current platforms lack a **standard format** as "each site has different schemas".

The objective of the proposed NIP-XX is to enable **structured data for LLM consumption**, making job data truly **open and LLM-accessible**.

### Core Principles of LLM-Friendly Design

The design philosophy prioritises clarity and consistency for machine processing:

1.  **Structured Tags:** The protocol advocates for the use of **structured tags** which are **easy to parse without NLP** (Natural Language Processing).
2.  **Controlled Vocabularies:** It requires **controlled vocabularies** for key fields to ensure **standardized values**.
3.  **Schema Versioning:** An **embedded schema version** tag ensures that the protocol can evolve over time **without breaking** client applications.
4.  **Content Format:** While the core structure is handled by tags, the main `content` field is designed to use **Markdown**, which is readable by both humans and LLMs.
5.  **Schema.org Compatibility:** It is possible to optionally include **JSON-LD in the content** to provide the full `schema.org/JobPosting` information.
6.  **Endpoints:** The future indexing service should expose **LLM-friendly JSON endpoints**.

### Implementing Structured Tags and Controlled Vocabularies

*   **Standardized Fields:** The event uses tags like `["title", "Senior Rust Developer"]`, `["company", "Acme Corp"]`, and `["apply-url", "https://..."]`.
*   **Multi-Value and Standardized Fields (Controlled Vocabularies):** Fields that traditionally vary are strictly defined. For example:
    *   **Salary:** The `salary` tag is multi-valued, standardising the minimum, maximum, currency, and period (e.g., annual).
    *   **Experience:** The `experience` tag uses **standardized levels** (e.g., mid, senior, staff). The Rust implementation defines specific `ExperienceLevel` enums (Intern, Junior, Mid, Senior, Staff, Principal).
    *   **Employment Type:** The `employment-type` tag uses a controlled set of values (e.g., full-time, part-time, contract, internship).
    *   **Location Type:** The `location-type` tag uses standardized values (remote, hybrid, onsite).

### Structured Data Design: Extensibility and Namespacing

In the larger context of Structured Data Design, the sources address the need for a protocol that can handle evolving requirements and industry variations without collapsing into fragmentation.

The sources advocate for an extensible protocol design that includes:

1.  **Core Required Fields:** These are the universal elements every job listing must have (e.g., `title`, `summary`, `company`).
2.  **Optional Extension Fields:** These fields accommodate industry-specific, company-specific, or regional/legal requirements (e.g., security clearances, certifications).

The key mechanism for enabling customization while maintaining machine-readability is **custom namespacing**. This means that optional fields are nested under named scopes within the data structure (e.g., `finance_domain`, `security`, `company_internal`).

This approach works because it:
*   **Preserves a clean standard** for core fields.
*   Allows tools to **ignore unknown namespaces**.
*   Enables **infinite customization** without sacrificing the machine-readability of the core data.

Ultimately, this structured approach ensures that the data is not only readable by humans but also provides the consistency needed to make **AI parsing more consistent** and facilitates easier indexing and candidate matching for sophisticated tools, including Large Language Models.

# djp-nosjob
