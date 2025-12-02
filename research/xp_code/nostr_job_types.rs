// src/types.rs
// Schema.org-aligned job listing types for Nostr
// Combines Schema.org JobPosting with Nostr-specific extensions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main job listing structure
/// Follows Schema.org JobPosting with Nostr extensions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobListing {
    // === Schema.org Core Fields ===
    #[serde(rename = "@context")]
    pub context: String, // "https://schema.org"
    
    #[serde(rename = "@type")]
    pub schema_type: String, // "JobPosting"
    
    pub identifier: String, // Unique ID (maps to Nostr "d" tag)
    pub title: String,
    pub description: String, // Full markdown description
    pub date_posted: String, // ISO 8601 (YYYY-MM-DD)
    pub valid_through: Option<String>, // Expiry date (ISO 8601)
    
    pub hiring_organization: HiringOrganization,
    pub job_location: Vec<JobLocation>,
    pub employment_type: Vec<EmploymentType>,
    
    // === Schema.org Recommended Fields ===
    pub base_salary: Option<BaseSalary>,
    pub job_location_type: Option<Vec<JobLocationType>>,
    pub qualifications: Option<String>,
    pub responsibilities: Option<String>,
    pub skills: Option<Vec<String>>,
    pub work_hours: Option<String>,
    pub applicant_location_requirements: Option<Vec<ApplicantLocationRequirement>>,
    pub job_benefits: Option<Vec<String>>,
    pub experience_requirements: Option<ExperienceRequirement>,
    
    // === Nostr-Specific Extensions ===
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nostr_employer_pubkey: Option<String>, // npub or hex
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_via_nostr: Option<bool>, // Accept encrypted applications
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lightning_address: Option<String>, // For Lightning payments
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nip05_verified: Option<String>, // Employer verification
    
    // === Extensions ===
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Organization hiring for the position
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HiringOrganization {
    #[serde(rename = "@type")]
    pub schema_type: String, // "Organization"
    
    pub name: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>, // Image URL
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_as: Option<Vec<String>>, // Social media profiles
}

/// Physical or remote location
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobLocation {
    #[serde(rename = "@type")]
    pub schema_type: String, // "Place"
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostalAddress>,
}

/// Postal address structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostalAddress {
    #[serde(rename = "@type")]
    pub schema_type: String, // "PostalAddress"
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_locality: Option<String>, // City
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_region: Option<String>, // State/Province
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>, // ISO 3166-1 (US, GB, etc.)
}

/// Employment type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EmploymentType {
    FullTime,
    PartTime,
    Contractor,
    Temporary,
    Intern,
    Volunteer,
    PerDiem,
    Other,
}

/// Job location type (remote, onsite, hybrid)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobLocationType {
    Telecommute, // Remote work
    OnSite,      // Physical office
    Hybrid,      // Mix of both (custom addition)
}

/// Salary information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseSalary {
    #[serde(rename = "@type")]
    pub schema_type: String, // "MonetaryAmount"
    
    pub currency: String, // ISO 4217 (USD, EUR, GBP, etc.)
    pub value: MonetaryValue,
}

/// Monetary value (single amount or range)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonetaryValue {
    Single(MonetaryAmountDistribution),
    Range(QuantitativeValue),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonetaryAmountDistribution {
    #[serde(rename = "@type")]
    pub schema_type: String, // "MonetaryAmountDistribution"
    
    pub duration: String, // "P1Y" (annual), "P1M" (monthly), "PT1H" (hourly)
    pub median: Option<f64>,
    pub percentile10: Option<f64>,
    pub percentile25: Option<f64>,
    pub percentile75: Option<f64>,
    pub percentile90: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuantitativeValue {
    #[serde(rename = "@type")]
    pub schema_type: String, // "QuantitativeValue"
    
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub unit_text: String, // "YEAR", "MONTH", "HOUR"
}

/// Applicant location requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicantLocationRequirement {
    #[serde(rename = "@type")]
    pub schema_type: String, // "Country" or "State"
    
    pub name: String, // Country or state name
}

/// Experience requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExperienceRequirement {
    #[serde(rename = "@type")]
    pub schema_type: String, // "OccupationalExperienceRequirements"
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub months_of_experience: Option<u32>,
}

impl JobListing {
    /// Create a new job listing with required fields
    pub fn new(
        identifier: String,
        title: String,
        description: String,
        company_name: String,
        date_posted: String,
    ) -> Self {
        Self {
            context: "https://schema.org".to_string(),
            schema_type: "JobPosting".to_string(),
            identifier,
            title,
            description,
            date_posted,
            valid_through: None,
            hiring_organization: HiringOrganization {
                schema_type: "Organization".to_string(),
                name: company_name,
                url: None,
                logo: None,
                same_as: None,
            },
            job_location: vec![],
            employment_type: vec![],
            base_salary: None,
            job_location_type: None,
            qualifications: None,
            responsibilities: None,
            skills: None,
            work_hours: None,
            applicant_location_requirements: None,
            job_benefits: None,
            experience_requirements: None,
            nostr_employer_pubkey: None,
            apply_via_nostr: None,
            lightning_address: None,
            nip05_verified: None,
            extra: HashMap::new(),
        }
    }

    /// Validate required fields
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.identifier.is_empty() {
            return Err(ValidationError::MissingField("identifier"));
        }
        if self.title.is_empty() {
            return Err(ValidationError::MissingField("title"));
        }
        if self.description.is_empty() {
            return Err(ValidationError::MissingField("description"));
        }
        if self.hiring_organization.name.is_empty() {
            return Err(ValidationError::MissingField("hiring_organization.name"));
        }
        
        // Validate date format (basic check)
        if !self.date_posted.contains('-') {
            return Err(ValidationError::InvalidDateFormat("date_posted"));
        }
        
        Ok(())
    }

    /// Builder pattern helper
    pub fn builder(identifier: String, title: String, description: String, company: String, date: String) -> JobListingBuilder {
        JobListingBuilder::new(identifier, title, description, company, date)
    }
}

/// Builder for JobListing
pub struct JobListingBuilder {
    job: JobListing,
}

impl JobListingBuilder {
    pub fn new(identifier: String, title: String, description: String, company: String, date: String) -> Self {
        Self {
            job: JobListing::new(identifier, title, description, company, date),
        }
    }

    pub fn valid_through(mut self, date: String) -> Self {
        self.job.valid_through = Some(date);
        self
    }

    pub fn employment_type(mut self, types: Vec<EmploymentType>) -> Self {
        self.job.employment_type = types;
        self
    }

    pub fn location_type(mut self, types: Vec<JobLocationType>) -> Self {
        self.job.job_location_type = Some(types);
        self
    }

    pub fn salary(mut self, min: f64, max: f64, currency: String, period: String) -> Self {
        self.job.base_salary = Some(BaseSalary {
            schema_type: "MonetaryAmount".to_string(),
            currency,
            value: MonetaryValue::Range(QuantitativeValue {
                schema_type: "QuantitativeValue".to_string(),
                min_value: Some(min),
                max_value: Some(max),
                unit_text: period,
            }),
        });
        self
    }

    pub fn skills(mut self, skills: Vec<String>) -> Self {
        self.job.skills = Some(skills);
        self
    }

    pub fn remote(mut self) -> Self {
        self.job.job_location_type = Some(vec![JobLocationType::Telecommute]);
        self
    }

    pub fn nostr_pubkey(mut self, pubkey: String) -> Self {
        self.job.nostr_employer_pubkey = Some(pubkey);
        self
    }

    pub fn lightning_address(mut self, address: String) -> Self {
        self.job.lightning_address = Some(address);
        self
    }

    pub fn build(self) -> Result<JobListing, ValidationError> {
        self.job.validate()?;
        Ok(self.job)
    }
}

/// Validation errors
#[derive(Debug, Clone)]
pub enum ValidationError {
    MissingField(&'static str),
    InvalidDateFormat(&'static str),
    InvalidUrl(&'static str),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidationError::MissingField(field) => write!(f, "Missing required field: {}", field),
            ValidationError::InvalidDateFormat(field) => write!(f, "Invalid date format in: {}", field),
            ValidationError::InvalidUrl(field) => write!(f, "Invalid URL in: {}", field),
        }
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_listing_creation() {
        let job = JobListing::builder(
            "job-001".to_string(),
            "Senior Rust Developer".to_string(),
            "Build decentralized systems".to_string(),
            "Acme Corp".to_string(),
            "2025-01-15".to_string(),
        )
        .employment_type(vec![EmploymentType::FullTime])
        .remote()
        .salary(120000.0, 180000.0, "USD".to_string(), "YEAR".to_string())
        .skills(vec!["Rust".to_string(), "Nostr".to_string()])
        .build();

        assert!(job.is_ok());
        let job = job.unwrap();
        assert_eq!(job.title, "Senior Rust Developer");
        assert_eq!(job.job_location_type, Some(vec![JobLocationType::Telecommute]));
    }

    #[test]
    fn test_validation_missing_title() {
        let mut job = JobListing::new(
            "job-001".to_string(),
            "".to_string(), // Empty title
            "Description".to_string(),
            "Company".to_string(),
            "2025-01-15".to_string(),
        );
        
        assert!(job.validate().is_err());
    }

    #[test]
    fn test_serialization() {
        let job = JobListing::builder(
            "job-001".to_string(),
            "Test Job".to_string(),
            "Test Description".to_string(),
            "Test Corp".to_string(),
            "2025-01-15".to_string(),
        )
        .build()
        .unwrap();

        let json = serde_json::to_string_pretty(&job).unwrap();
        assert!(json.contains("JobPosting"));
        assert!(json.contains("schema.org"));
    }
}
