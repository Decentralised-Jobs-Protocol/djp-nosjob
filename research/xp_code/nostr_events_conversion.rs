// src/events.rs
// Converts JobListing to/from Nostr events
// Stores Schema.org JSON-LD in content + searchable tags

use crate::types::*;
use nostr_sdk::prelude::*;
use anyhow::{anyhow, Result};

impl JobListing {
    /// Convert JobListing to Nostr event (kind 5001)
    /// 
    /// Structure:
    /// - content: Full Schema.org JSON-LD (for LLMs, web compatibility)
    /// - tags: Searchable metadata for Nostr clients
    pub fn to_nostr_event(&self, keys: &Keys) -> Result<Event> {
        // 1. Serialize as Schema.org JSON-LD in content
        let content = serde_json::to_string_pretty(self)?;
        
        // 2. Build Nostr tags for filtering/searching
        let mut tags = vec![
            // Identifier (required for replaceable events)
            Tag::identifier(&self.identifier),
            
            // Core searchable fields
            Tag::custom(TagKind::Custom("title".into()), vec![&self.title]),
            Tag::custom(TagKind::Custom("company".into()), vec![&self.hiring_organization.name]),
            Tag::custom(TagKind::Custom("date-posted".into()), vec![&self.date_posted]),
            
            // Hashtag for discovery
            Tag::hashtag("jobstr"),
        ];
        
        // Add employment type tags
        for emp_type in &self.employment_type {
            let type_str = format!("{:?}", emp_type).to_lowercase();
            tags.push(Tag::custom(TagKind::Custom("employment-type".into()), vec![type_str]));
        }
        
        // Add location type tags
        if let Some(location_types) = &self.job_location_type {
            for loc_type in location_types {
                let type_str = format!("{:?}", loc_type).to_lowercase();
                tags.push(Tag::custom(TagKind::Custom("location-type".into()), vec![type_str]));
            }
        }
        
        // Add salary range if present (for filtering)
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
        
        // Add skills as tags
        if let Some(skills) = &self.skills {
            for skill in skills {
                tags.push(Tag::custom(TagKind::Custom("skill".into()), vec![skill.clone()]));
            }
        }
        
        // Add location tags
        for location in &self.job_location {
            if let Some(address) = &location.address {
                if let Some(country) = &address.address_country {
                    tags.push(Tag::custom(TagKind::Custom("country".into()), vec![country.clone()]));
                }
                if let Some(region) = &address.address_region {
                    tags.push(Tag::custom(TagKind::Custom("region".into()), vec![region.clone()]));
                }
            }
        }
        
        // Add expiry date if present
        if let Some(expiry) = &self.valid_through {
            tags.push(Tag::custom(TagKind::Custom("expires".into()), vec![expiry.clone()]));
        }
        
        // Add Nostr-specific tags
        if let Some(pubkey) = &self.nostr_employer_pubkey {
            tags.push(Tag::custom(TagKind::Custom("employer-pubkey".into()), vec![pubkey.clone()]));
        }
        
        if let Some(ln_address) = &self.lightning_address {
            tags.push(Tag::custom(TagKind::Custom("lightning".into()), vec![ln_address.clone()]));
        }
        
        // Add company URL if present
        if let Some(url) = &self.hiring_organization.url {
            tags.push(Tag::custom(TagKind::Custom("company-url".into()), vec![url.clone()]));
        }
        
        // 3. Create and sign event
        EventBuilder::new(Kind::from(5001), content)
            .tags(tags)
            .sign_with_keys(keys)
    }

    /// Parse JobListing from Nostr event
    /// 
    /// Attempts to parse Schema.org JSON-LD from content field.
    /// Falls back to constructing from tags if content is not valid JSON.
    pub fn from_nostr_event(event: &Event) -> Result<Self> {
        // Verify event kind
        if event.kind != Kind::from(5001) {
            return Err(anyhow!("Invalid event kind: expected 5001, got {}", event.kind));
        }
        
        // Try to parse content as Schema.org JSON-LD
        if let Ok(job) = serde_json::from_str::<JobListing>(&event.content) {
            // Validate the parsed job
            job.validate()?;
            return Ok(job);
        }
        
        // Fallback: construct from tags (for backward compatibility)
        Self::from_tags(event)
    }
    
    /// Construct JobListing from Nostr tags (legacy support)
    fn from_tags(event: &Event) -> Result<Self> {
        // Extract required fields from tags
        let identifier = event
            .tags
            .iter()
            .find_map(|t| match t {
                Tag::Identifier(id) => Some(id.to_string()),
                _ => None,
            })
            .ok_or_else(|| anyhow!("Missing identifier tag"))?;
        
        let title = Self::find_tag_value(&event.tags, "title")
            .ok_or_else(|| anyhow!("Missing title tag"))?;
        
        let company = Self::find_tag_value(&event.tags, "company")
            .ok_or_else(|| anyhow!("Missing company tag"))?;
        
        let date_posted = Self::find_tag_value(&event.tags, "date-posted")
            .unwrap_or_else(|| event.created_at.to_human_datetime());
        
        // Use content as description if it's not JSON
        let description = event.content.clone();
        
        // Build job with required fields
        let mut job = JobListing::new(
            identifier,
            title,
            description,
            company,
            date_posted,
        );
        
        // Parse optional fields from tags
        
        // Employment types
        job.employment_type = event
            .tags
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
            event
                .tags
                .iter()
                .filter_map(|t| Self::parse_custom_tag(t, "location-type"))
                .filter_map(|s| match s.to_uppercase().as_str() {
                    "TELECOMMUTE" => Some(JobLocationType::Telecommute),
                    "ONSITE" => Some(JobLocationType::OnSite),
                    "HYBRID" => Some(JobLocationType::Hybrid),
                    _ => None,
                })
                .collect(),
        );
        
        // Skills
        job.skills = Some(
            event
                .tags
                .iter()
                .filter_map(|t| Self::parse_custom_tag(t, "skill"))
                .collect(),
        );
        
        // Expiry date
        job.valid_through = Self::find_tag_value(&event.tags, "expires");
        
        // Nostr-specific fields
        job.nostr_employer_pubkey = Self::find_tag_value(&event.tags, "employer-pubkey");
        job.lightning_address = Self::find_tag_value(&event.tags, "lightning");
        
        // Company URL
        if let Some(url) = Self::find_tag_value(&event.tags, "company-url") {
            job.hiring_organization.url = Some(url);
        }
        
        Ok(job)
    }
    
    /// Helper: find a tag value by name
    fn find_tag_value(tags: &[Tag], name: &str) -> Option<String> {
        tags.iter().find_map(|t| match t.as_slice() {
            [tag_name, value, ..] if tag_name == name => Some(value.to_string()),
            _ => None,
        })
    }
    
    /// Helper: parse custom tag
    fn parse_custom_tag(tag: &Tag, name: &str) -> Option<String> {
        match tag.as_slice() {
            [tag_name, value, ..] if tag_name == name => Some(value.to_string()),
            _ => None,
        }
    }
}

/// Example jobs filter builder
pub struct JobsFilter {
    filter: Filter,
}

impl JobsFilter {
    pub fn new() -> Self {
        Self {
            filter: Filter::new().kind(Kind::from(5001)),
        }
    }
    
    pub fn hashtag(mut self, tag: &str) -> Self {
        self.filter = self.filter.hashtag(tag);
        self
    }
    
    pub fn employment_type(mut self, emp_type: EmploymentType) -> Self {
        let type_str = format!("{:?}", emp_type).to_lowercase();
        self.filter = self.filter.custom_tag(
            SingleLetterTag::lowercase(Alphabet::T),
            vec![type_str]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_conversion() {
        let keys = Keys::generate();
        
        let job = JobListing::builder(
            "job-001".to_string(),
            "Rust Developer".to_string(),
            "Build awesome stuff".to_string(),
            "Test Corp".to_string(),
            "2025-01-15".to_string(),
        )
        .employment_type(vec![EmploymentType::FullTime])
        .remote()
        .skills(vec!["Rust".to_string(), "Nostr".to_string()])
        .build()
        .unwrap();
        
        // Convert to event
        let event = job.to_nostr_event(&keys).unwrap();
        
        // Verify event structure
        assert_eq!(event.kind, Kind::from(5001));
        assert!(event.content.contains("JobPosting"));
        assert!(event.content.contains("schema.org"));
        
        // Parse back
        let parsed = JobListing::from_nostr_event(&event).unwrap();
        
        // Verify fields match
        assert_eq!(parsed.title, job.title);
        assert_eq!(parsed.identifier, job.identifier);
        assert_eq!(parsed.hiring_organization.name, job.hiring_organization.name);
    }
    
    #[test]
    fn test_tags_created() {
        let keys = Keys::generate();
        
        let job = JobListing::builder(
            "job-001".to_string(),
            "Test Job".to_string(),
            "Test".to_string(),
            "Test Corp".to_string(),
            "2025-01-15".to_string(),
        )
        .skills(vec!["Rust".to_string()])
        .build()
        .unwrap();
        
        let event = job.to_nostr_event(&keys).unwrap();
        
        // Check that hashtag exists
        let has_jobstr_tag = event.tags.iter().any(|t| {
            matches!(t, Tag::Hashtag(tag) if tag == "jobstr")
        });
        assert!(has_jobstr_tag);
        
        // Check that skill tag exists
        let has_skill_tag = event.tags.iter().any(|t| {
            t.as_slice().get(0) == Some(&"skill".to_string())
        });
        assert!(has_skill_tag);
    }
}
