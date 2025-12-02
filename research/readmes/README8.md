```markdown
# jobstr.work - Frequently Asked Questions

## For Employers & Recruiters

### How do I post a job to Nostr?

**Three ways:**

1. **Easy: Use jobstr.work** (recommended)
   - Fill in our web form
   - We publish to Nostr relays for you
   - Optional: Pay for featured placement ($100-200)

2. **Technical: Use our SDK** (coming Q1 2026)
   ```rust
   // Example
   let job = JobPost::new()
       .title("Senior Rust Engineer")
       .location("Remote")
       .salary_range(90000, 120000)
       .publish_to_relays()?;
   ```

3. **Advanced: Publish directly**
   - Use any Nostr client library
   - Follow our job event schema (NIP-XXX)
   - Post to public relays yourself

**Your job appears everywhere** - on jobstr.work, any other aggregator reading Nostr, and accessible to LLMs like Claude/ChatGPT.

---

### How much does it cost?

**Pricing:**
- **Free**: Post via Nostr clients, appears in basic feeds
- **$50-100**: Post via jobstr.work interface (easier, guided)
- **$100-200**: Featured placement (top of feeds, highlighted for LLMs)
- **$500**: Verified employer badge (one-time, shows company is legitimate)

**Why pay?** Your job gets:
- Higher visibility in feeds
- Priority placement for LLM searches
- Trust signals (verified badge)
- Analytics dashboard

---

### Can I edit or delete jobs?

**Yes.** Jobs on Nostr can be updated or deleted:
- **Via jobstr.work**: Edit in your dashboard
- **Via SDK**: Send updated event or deletion event
- **Via Nostr clients**: Replace or delete the event

Changes propagate to all aggregators automatically.

---

### How do I verify my company?

**Verification process:**
1. Apply at jobstr.work/verify
2. Provide: company website, LinkedIn, business registration
3. We verify you own the domain (email confirmation)
4. Link your Nostr pubkey to your company
5. Get verified badge on all job posts

**Cost:** $500 one-time fee

**Benefits:**
- Verified checkmark on all posts
- Listed in `/feed/verified-employers.txt`
- Higher applicant trust
- Reduced spam filtering

---

### Why should I use this instead of LinkedIn/Indeed?

**Post once, appear everywhere:**

**Traditional job boards:**
- Post to LinkedIn ❌
- Post to Indeed ❌
- Post to Glassdoor ❌
- Post to AngelList ❌
- = 4 platforms, 4 logins, 4 interfaces, expensive

**Nostr jobs:**
- Post to Nostr once ✅
- = Appears on jobstr.work, rustjobs.fyi, euro.remote, accessible to Claude/ChatGPT, and any future aggregator
- = 1 post, infinite distribution, lower cost

**Additional benefits:**
- ✅ Verifiable identity (no fake companies)
- ✅ Can't be taken down (decentralized)
- ✅ LLM-native (AI assistants can find/share your jobs)
- ✅ No platform lock-in (you own your data)

---

## For Job Seekers

### How do I search jobs?

**Multiple ways:**

1. **Browse jobstr.work**
   - Web interface with filters
   - Search by skill, location, salary, remote/on-site
   - Save searches, set up alerts (coming soon)

2. **Ask your AI assistant:**
   ```
   "Claude, check jobstr.work/feed/latest-100.txt 
    for remote Rust jobs paying over $100k"
   
   "ChatGPT, find senior engineering roles at 
    jobstr.work/feed/remote.txt"
   ```

3. **Use specialist job boards**
   Any site can aggregate Nostr jobs:
   - rustjobs.fyi (Rust specialist)
   - ukhealth.jobs (healthcare)
   - euro.remote (EU remote jobs)
   
   All reading from the same open Nostr relays!

4. **Subscribe via RSS/text feeds**
   - Get updates in your feed reader
   - Monitor specific tags or locations
   - Example: `jobstr.work/feed/rust-remote.txt`

5. **Build your own tools**
   - Data is open
   - Use our SDK (coming soon)
   - Subscribe to Nostr relays directly

---

### Is it free to search and apply?

**Yes.** 100% free for job seekers. Always.

We charge employers for premium features, never job seekers.

---

### How do I apply to jobs?

Jobs include application instructions:
- Apply via company website link
- Email directly to hiring manager
- Apply through company's ATS
- Contact via Nostr DM (if supported)

**Coming soon:** Apply with your Nostr profile (resume/portfolio stored on Nostr)

---

### Can I set up job alerts?

**Coming Q1 2025:**
- Email alerts for new jobs matching your criteria
- Nostr DM notifications
- RSS feed subscriptions (available now)

**Today:** Subscribe to specific text feeds:
- `jobstr.work/feed/rust-remote.txt`
- `jobstr.work/feed/by-location/eu.txt`
- Check feeds with your LLM daily

---

## For Developers

### Can I build my own job board using this data?

**Absolutely! We encourage it.**

**How:**
1. Subscribe to Nostr relays (e.g., `wss://relay.damus.io`)
2. Filter for job events (kind: `30402`)
3. Parse and display however you want
4. Examples of what others might build:
   - Geographic: jobs.berlin, remote.africa
   - Industry: biotech.jobs, fintech.careers
   - Experience: junior.dev, exec.roles
   - Language: trabajos.nostr (Spanish), emplois.nostr (French)

**You don't need permission** - the data is open. Build whatever serves your audience!

---

### Is there an SDK or API?

**Current options:**

**Text feeds** (available now):
- `/feed/latest-100.txt` - Last 100 jobs
- `/feed/today.txt` - Today's jobs
- `/feed/remote.txt` - Remote jobs only
- `/feed/by-tag/rust.txt` - By skill/technology
- `/feed/by-location/eu.txt` - By location
- `/feed/verified-employers.txt` - Verified companies only

**SDK** (coming Q1 2025):
- Rust, TypeScript, Python
- Easy job posting and searching
- Built on standard Nostr libraries

**MCP Server** (coming Q1 2025):
- Native Claude Desktop integration
- Real-time job queries
- Subscribe to updates

**Search API** (planned):
- `/search.txt?tag=rust&remote=true&days=7&limit=50`
- Dynamic filtering
- Always capped at reasonable result sizes

---

### What's the job event schema?

**Based on Nostr Event format (NIP-XXX proposal):**

```json
{
  "kind": 30402,
  "pubkey": "employer_pubkey_hex",
  "created_at": 1234567890,
  "tags": [
    ["d", "unique_job_id"],
    ["title", "Senior Rust Engineer"],
    ["company", "Acme Corp"],
    ["location", "Remote"],
    ["location_type", "remote"],
    ["type", "full-time"],
    ["salary_min", "90000"],
    ["salary_max", "120000"],
    ["salary_currency", "USD"],
    ["tag", "rust"],
    ["tag", "backend"],
    ["tag", "distributed-systems"],
    ["apply_url", "https://acme.com/careers/rust-eng"],
    ["expires", "1735689600"]
  ],
  "content": "Full job description in markdown...",
  "sig": "signature_hex"
}
```

**Key features:**
- Replaceable events (update/delete by pubkey + "d" tag)
- Standard tags for filtering
- Expiration dates
- Verifiable publisher (pubkey)

**Full spec:** [Link to NIP document or GitHub]

---

### Which Nostr relays should I use?

**Popular public relays for job events:**
- wss://relay.damus.io
- wss://nos.lol
- wss://relay.nostr.band
- wss://relay.snort.social

**Coming soon:** Specialized job relays for better performance

**Best practice:** 
- Subscribe to 3-5 relays for redundancy
- Use relay hints in job posts
- Consider running your own relay for guaranteed access

---

## About Nostr Jobs

### What is Nostr?

**Nostr** = "Notes and Other Stuff Transmitted by Relays"

A decentralized protocol using:
- **Cryptographic keys** for identity (no usernames/passwords)
- **Relays** for data distribution (no central servers)
- **Events** for all data (posts, profiles, jobs, etc.)

**For jobs, this means:**
- Employers have verifiable identities (pubkeys)
- No single company controls the data
- Can't be shut down or censored
- Lightning payments built-in (optional spam prevention)
- Anyone can build clients/aggregators

**Think:** "RSS for everything" but with identity and payments

---

### Why decentralized jobs?

**Problems with current job boards:**

1. **Siloed data** - LinkedIn/Indeed own everything
2. **Platform lock-in** - Post to 5 sites = 5 logins, 5 UIs
3. **Spam everywhere** - Free to post = low quality
4. **No verifiable identity** - Scam jobs, fake companies
5. **Expensive** - $200-500 per post adds up
6. **Not LLM-friendly** - Locked behind auth, rate limits

**Nostr solves this:**

1. ✅ **Open data** - Post once, appears everywhere
2. ✅ **Verifiable employers** - Cryptographic identity
3. ✅ **Spam resistance** - Optional Lightning payment per post
4. ✅ **Cost effective** - Pay jobstr.work or post directly (free)
5. ✅ **LLM-native** - Text feeds designed for AI assistants
6. ✅ **Can't be shut down** - Decentralized by design

---

### Who runs jobstr.work?

**jobstr.work is one aggregator** reading from open Nostr relays.

- We provide: web interface, text feeds, verification, analytics
- We don't own: the job data (on public relays)
- Others can: build competing aggregators (we encourage it!)

**Philosophy:**
- Open protocol > closed platform
- Curation > ownership
- Enable ecosystem > control everything

**Contact:** [Your contact info]

---

### How do you make money?

**We charge employers, never job seekers:**

**Revenue sources:**
- Employer verification ($500 one-time)
- Featured job placement ($100-200 per post)
- Analytics dashboard ($100/month)
- White-label solutions ($1000+/month)

**Always free:**
- All text feeds for LLMs
- Job seeker browsing/searching
- Basic employer job posts (via Nostr directly)
- Developer access to open data

**Model:** Like GitHub (free core product, charge for premium features), not LinkedIn (paywalled features for job seekers)

---

### Who owns the data?

**Employers own their job posts** via their Nostr private keys.

**Public Nostr relays store copies** - anyone can read, anyone can build aggregators.

**We (jobstr.work) don't own anything** - we just aggregate, curate, and display.

**This means:**
- Employers can delete/edit jobs anytime
- Other aggregators can show the same jobs
- If jobstr.work shuts down, jobs remain on Nostr
- You're never locked in

---

### Can jobstr.work shut down my job post?

**No.** Once published to Nostr relays:
- We can delist from jobstr.work (spam/ToS violations)
- Other aggregators might still show it
- Relays decide their own retention policies
- Only you (employer) can truly delete it

**This is intentional** - decentralization means no single point of control.

**Quality control:** We verify employers, filter spam, and curate feeds, but we don't own the underlying data.

---

## Getting Started

### I'm an employer - what's the quickest way to post?

1. Go to jobstr.work/post
2. Fill in job details (title, location, salary, etc.)
3. Click "Publish to Nostr"
4. Done! Appears in feeds within minutes

**Pro tip:** Get verified ($500) for higher trust and visibility.

---

### I'm a job seeker - how do I start?

1. Browse jobstr.work
2. Or ask your AI assistant:
   ```
   "Claude, check jobstr.work/feed/latest-100.txt 
    for [your criteria]"
   ```
3. Apply via the links in job posts
4. Optional: Set up RSS subscription for alerts

---

### I'm a developer - how do I build on this?

1. Read the job event schema (NIP-XXX)
2. Subscribe to Nostr relays
3. Filter for job events (kind: 30402)
4. Build your niche aggregator
5. Optional: Use our SDK when released (Q1 2025)

**Join the community:** [Discord/Telegram/Nostr group link]

---

## Still have questions?

**Contact us:**
- Email: hello@jobstr.work
- Nostr: npub1... [your pubkey]
- GitHub: github.com/jobstr-work
- Feedback: jobstr.work/feedback

**Join the conversation:**
- Twitter: @jobstrwork
- Nostr community: [link]
- Developer Discord: [link]

---

*Last updated: November 2024*
*Data retention: We index jobs from the last 30 days. Archives available at jobstr.work/archive*
```
