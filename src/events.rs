// src/events.rs
// Converts JobListing to/from Nostr events
// Kind 39993: Job Listings with UUID support

use crate::types::*;
use nostr_sdk::prelude::*;
use anyhow::{anyhow, Result};

// ==================== Job Listing Kind Constants ====================
pub const KIND_JOB_LISTING: u16 = 39993;  // Job posting
pub const KIND_JOB_APPLICATION: u16 = 39994;  // Job application (future use)

// ==================== JobListing ↔ Nostr Event ====================
impl JobListing {
    /// Convert JobListing to Nostr event (kind 39993)
    pub fn to_nostr_event(&self, keys: &Keys) -> Result<Event> {
        // 1. Serialize as JSON content
        let content = serde_json::to_string_pretty(self)?;

        // 2. Build searchable tags following 39993 spec
        let d_tag_value = format!("{}-{}-{}", 
            self.hiring_organization.name.to_lowercase().replace(' ', "-"),
            &self.identifier,
            &keys.public_key().to_hex()[..8]
        );

        let mut tags = vec![
            Tag::identifier(&d_tag_value),
            Tag::custom(TagKind::Custom("t".into()), vec!["Jobs"]),
            Tag::custom(TagKind::Custom("company".into()), vec![&self.hiring_organization.name]),
            Tag::custom(TagKind::Custom("job-id".into()), vec![&self.identifier]),
            Tag::custom(TagKind::Custom("title".into()), vec![&self.title]),
        ];

        // Location
        tags.push(Tag::custom(TagKind::Custom("location".into()), 
            vec![self.job_location_type.as_ref()
                .and_then(|types| types.first())
                .map(|t| format!("{:?}", t))
                .unwrap_or_else(|| "Remote".to_string())]
        ));

        // Employment type
        for emp_type in &self.employment_type {
            tags.push(Tag::custom(
                TagKind::Custom("employment-type".into()), 
                vec![format!("{:?}", emp_type)]
            ));
        }

        // Skills
        if let Some(skills) = &self.skills {
            for skill in skills {
                tags.push(Tag::custom(TagKind::Custom("skill".into()), vec![skill.clone()]));
            }
        }

        // Salary range
        if let Some(salary) = &self.base_salary {
            if let MonetaryValue::Range(range) = &salary.value {
                if let (Some(min), Some(max)) = (range.min_value, range.max_value) {
                    tags.push(Tag::custom(
                        TagKind::Custom("salary".into()),
                        vec![
                            min.to_string(),
                            max.to_string(),
                            salary.currency.clone(),
                            range.unit_text.clone(),
                        ],
                    ));
                }
            }
        }

        // Location tags (country/region)
        for location in &self.job_location {
            if let Some(addr) = &location.address {
                if let Some(country) = &addr.address_country {
                    tags.push(Tag::custom(TagKind::Custom("country".into()), vec![country.clone()]));
                }
                if let Some(region) = &addr.address_region {
                    tags.push(Tag::custom(TagKind::Custom("region".into()), vec![region.clone()]));
                }
            }
        }

        // Expiry date
        if let Some(expiry) = &self.valid_through {
            tags.push(Tag::custom(TagKind::Custom("expires".into()), vec![expiry.clone()]));
        }

        // Nostr-specific
        if let Some(pubkey) = &self.nostr_employer_pubkey {
            tags.push(Tag::custom(TagKind::Custom("employer-pubkey".into()), vec![pubkey.clone()]));
        }
        if let Some(ln) = &self.lightning_address {
            tags.push(Tag::custom(TagKind::Custom("lightning".into()), vec![ln.clone()]));
        }
        if let Some(url) = &self.hiring_organization.url {
            tags.push(Tag::custom(TagKind::Custom("company-url".into()), vec![url.clone()]));
        }

        // 3. Build and sign event
        Ok(EventBuilder::new(Kind::from(KIND_JOB_LISTING), content)
            .tags(tags)
            .sign_with_keys(keys)?)
    }

    /// Parse JobListing from kind 39993 Nostr event
    pub fn from_nostr_event(event: &Event) -> Result<Self> {
        if event.kind != Kind::from(KIND_JOB_LISTING) {
            return Err(anyhow!(
                "Invalid event kind: expected {}, got {}",
                KIND_JOB_LISTING,
                event.kind
            ));
        }

        // Try JSON content first
        if let Ok(job) = serde_json::from_str::<JobListing>(&event.content) {
            job.validate()?;
            return Ok(job);
        }

        // Fallback: construct from tags
        Self::from_tags(event)
    }

    /// Legacy / fallback from tags
    fn from_tags(event: &Event) -> Result<Self> {
        let tags_vec: Vec<Tag> = event.tags.iter().cloned().collect();

        let identifier = tags_vec
            .iter()
            .find_map(|t| {
                let slice = t.as_slice();
                if !slice.is_empty() && slice[0] == "d" && slice.len() > 1 {
                    Some(slice[1].clone())
                } else { None }
            })
            .ok_or_else(|| anyhow!("Missing identifier tag"))?;

        let title = Self::find_tag_value(&tags_vec, "title")
            .ok_or_else(|| anyhow!("Missing title tag"))?;
        let company = Self::find_tag_value(&tags_vec, "company")
            .ok_or_else(|| anyhow!("Missing company tag"))?;
        let date_posted = Self::find_tag_value(&tags_vec, "date-posted")
            .unwrap_or_else(|| event.created_at.to_human_datetime());
        let description = event.content.clone();

        let mut job = JobListing::new(
            identifier,
            title,
            description,
            company,
            date_posted,
            Some("".to_string()),
            vec![EligibleWorkerType::Human],
        );

        // Employment types
        job.employment_type = tags_vec
            .iter()
            .filter_map(|t| Self::parse_custom_tag(t, "employment-type"))
            .filter_map(|s| match s.to_uppercase().as_str() {
                "FULLTIME" => Some(EmploymentType::FullTime),
                "PARTTIME" => Some(EmploymentType::PartTime),
                "CONTRACTOR" => Some(EmploymentType::Contractor),
                "INTERN" => Some(EmploymentType::Intern),
                _ => None,
            })
            .collect();

        // Location types
        job.job_location_type = Some(
            tags_vec
                .iter()
                .filter_map(|t| Self::parse_custom_tag(t, "location-type"))
                .filter_map(|s| match s.to_uppercase().as_str() {
                    "TELECOMMUTE" | "REMOTE" => Some(JobLocationType::Telecommute),
                    "ONSITE" => Some(JobLocationType::OnSite),
                    "HYBRID" => Some(JobLocationType::Hybrid),
                    _ => None,
                })
                .collect(),
        );

        // Skills
        job.skills = Some(
            tags_vec.iter()
                .filter_map(|t| Self::parse_custom_tag(t, "skill"))
                .collect()
        );

        // Expiry
        job.valid_through = Self::find_tag_value(&tags_vec, "expires");

        // Nostr-specific
        job.nostr_employer_pubkey = Self::find_tag_value(&tags_vec, "employer-pubkey");
        job.lightning_address = Self::find_tag_value(&tags_vec, "lightning");
        if let Some(url) = Self::find_tag_value(&tags_vec, "company-url") {
            job.hiring_organization.url = Some(url);
        }

        Ok(job)
    }

    fn find_tag_value(tags: &[Tag], name: &str) -> Option<String> {
        tags.iter().find_map(|t| match t.as_slice() {
            [tag_name, value, ..] if tag_name == name => Some(value.to_string()),
            _ => None,
        })
    }

    fn parse_custom_tag(tag: &Tag, name: &str) -> Option<String> {
        match tag.as_slice() {
            [tag_name, value, ..] if tag_name == name => Some(value.to_string()),
            _ => None,
        }
    }
}

// ==================== Jobs Filter ====================
pub struct JobsFilter {
    filter: Filter,
}

impl JobsFilter {
    pub fn new() -> Self {
        Self {
            filter: Filter::new()
                .kind(Kind::from(KIND_JOB_LISTING))
                .custom_tag(SingleLetterTag::lowercase(Alphabet::T), "Jobs"),
        }
    }

    pub fn company(mut self, name: &str) -> Self {
        self.filter = self.filter.custom_tag(
            SingleLetterTag::lowercase(Alphabet::C), 
            name.to_string()
        );
        self
    }

    pub fn employment_type(mut self, emp_type: EmploymentType) -> Self {
        let s = format!("{:?}", emp_type);
        self.filter = self.filter.custom_tag(
            SingleLetterTag::lowercase(Alphabet::E), 
            s
        );
        self
    }

    pub fn skill(mut self, skill: &str) -> Self {
        self.filter = self.filter.custom_tag(
            SingleLetterTag::lowercase(Alphabet::S), 
            skill.to_string()
        );
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.filter = self.filter.limit(limit);
        self
    }

    pub fn build(self) -> Filter {
        self.filter
    }
}

impl Default for JobsFilter {
    fn default() -> Self {
        Self::new()
    }
}
