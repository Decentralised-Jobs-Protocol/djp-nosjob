// Note: For kind 39993, content field contains simplified JSON
// Tags handle filtering, content provides full details

// src/types.rs
// Schema.org-aligned job listing types for Nostr
// Combines Schema.org JobPosting with Nostr-specific extensions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main job listing structure
/// Follows Schema.org JobPosting with Nostr extensions for NIP-104
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

    // === NIP-104: Worker Type (REQUIRED) ===
    pub eligible_worker_type: Vec<EligibleWorkerType>,

    pub hiring_organization: HiringOrganization,
    pub job_location: Vec<JobLocation>,
    pub employment_type: Vec<EmploymentType>,

    // === Application ===
    pub apply_url: Option<String>, 

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

    // === NIP-104: Structured Capability Requirements ===
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_capabilities: Option<Vec<CapabilityRequirement>>,

    // === NIP-104: AI Agent Requirements (Optional) ===
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_max: Option<PerformanceRequirement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy_min: Option<PerformanceRequirement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_min: Option<PerformanceRequirement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_type: Option<InterfaceType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>, // "REST", "GraphQL", "gRPC"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_oversight: Option<OversightRequirement>,

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

    // === Multi-language support ===
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations: Option<HashMap<String, TranslatedJob>>,
}

/// NIP-104: Eligible worker types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EligibleWorkerType {
    Human,
    AIAgent,
}

/// NIP-104: Structured capability requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirement {
    pub name: String,
    pub level: CapabilityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CapabilityLevel {
    Basic,
    Intermediate,
    Advanced,
    Expert,
}

/// NIP-104: Performance requirements for AI agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirement {
    pub value: f64,
    pub unit: String,
}

/// NIP-104: Interface types for AI agents
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InterfaceType {
    Api,
    Rpc,
    Webhook,
    WebPortal,
}

/// NIP-104: Human oversight requirements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OversightRequirement {
    Required,
    Optional,
    None,
}

/// Translated job content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslatedJob {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiringOrganization {
    #[serde(rename = "@type")]
    pub schema_type: String, // "Organization"

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_as: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobLocation {
    #[serde(rename = "@type")]
    pub schema_type: String, // "Place"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostalAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostalAddress {
    #[serde(rename = "@type")]
    pub schema_type: String, // "PostalAddress"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_locality: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_region: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>,
}

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
    TaskBased,
    MicroTask,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobLocationType {
    Telecommute,
    OnSite,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseSalary {
    #[serde(rename = "@type")]
    pub schema_type: String,

    pub currency: String,
    pub value: MonetaryValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonetaryValue {
    Single(MonetaryAmountDistribution),
    Range(QuantitativeValue),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonetaryAmountDistribution {
    #[serde(rename = "@type")]
    pub schema_type: String,

    pub duration: String,
    pub median: Option<f64>,
    pub percentile10: Option<f64>,
    pub percentile25: Option<f64>,
    pub percentile75: Option<f64>,
    pub percentile90: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantitativeValue {
    #[serde(rename = "@type")]
    pub schema_type: String,

    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub unit_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicantLocationRequirement {
    #[serde(rename = "@type")]
    pub schema_type: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceRequirement {
    #[serde(rename = "@type")]
    pub schema_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub months_of_experience: Option<u32>,
}

// === Implementation ===
impl JobListing {
    pub fn new(
        identifier: String,
        title: String,
        description: String,
        company_name: String,
        date_posted: String,
        apply_url: Option<String>,   // UPDATED
        eligible_worker_type: Vec<EligibleWorkerType>,
    ) -> Self {
        Self {
            context: "https://schema.org".to_string(),
            schema_type: "JobPosting".to_string(),
            identifier,
            title,
            description,
            date_posted,
            valid_through: None,
            eligible_worker_type,
            hiring_organization: HiringOrganization {
                schema_type: "Organization".to_string(),
                name: company_name,
                url: None,
                logo: None,
                same_as: None,
            },
            job_location: vec![],
            employment_type: vec![],
            apply_url, // now Option<>
            base_salary: None,
            job_location_type: None,
            qualifications: None,
            responsibilities: None,
            skills: None,
            work_hours: None,
            applicant_location_requirements: None,
            job_benefits: None,
            experience_requirements: None,
            required_capabilities: None,
            response_time_max: None,
            accuracy_min: None,
            throughput_min: None,
            interface_type: None,
            protocol: None,
            human_oversight: None,
            nostr_employer_pubkey: None,
            apply_via_nostr: None,
            lightning_address: None,
            nip05_verified: None,
            extra: HashMap::new(),
            translations: None,
        }
    }

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
        if self.apply_url.is_none() {
            return Err(ValidationError::MissingField("apply_url"));
        }
        if self.eligible_worker_type.is_empty() {
            return Err(ValidationError::MissingField("eligible_worker_type"));
        }

        if !self.date_posted.contains('-') {
            return Err(ValidationError::InvalidDateFormat("date_posted"));
        }

        // Validate AI agent fields are only used with AIAgent worker type
        if !self.eligible_worker_type.contains(&EligibleWorkerType::AIAgent) {
            if self.response_time_max.is_some()
                || self.accuracy_min.is_some()
                || self.throughput_min.is_some()
                || self.interface_type.is_some()
                || self.protocol.is_some()
            {
                return Err(ValidationError::InvalidConfiguration(
                    "AI agent fields require EligibleWorkerType::AIAgent",
                ));
            }
        }

        Ok(())
    }

    pub fn builder(
        identifier: String,
        title: String,
        description: String,
        company: String,
        date: String,
        apply_url: String,
    ) -> JobListingBuilder {
        JobListingBuilder::new(identifier, title, description, company, date, apply_url)
    }
}

/// Builder pattern
pub struct JobListingBuilder {
    job: JobListing,
}

impl JobListingBuilder {
    pub fn new(
        identifier: String,
        title: String,
        description: String,
        company: String,
        date: String,
        apply_url: String,
    ) -> Self {
        Self {
            job: JobListing::new(
                identifier,
                title,
                description,
                company,
                date,
                Some(apply_url),
                vec![EligibleWorkerType::Human], // Default to human!
            ),
        }
    }

    pub fn valid_through(mut self, date: String) -> Self {
        self.job.valid_through = Some(date);
        self
    }

    pub fn eligible_worker_type(mut self, types: Vec<EligibleWorkerType>) -> Self {
        self.job.eligible_worker_type = types;
        self
    }

    pub fn for_humans(mut self) -> Self {
        self.job.eligible_worker_type = vec![EligibleWorkerType::Human];
        self
    }

    pub fn for_ai_agents(mut self) -> Self {
        self.job.eligible_worker_type = vec![EligibleWorkerType::AIAgent];
        self
    }

    pub fn for_hybrid(mut self) -> Self {
        self.job.eligible_worker_type =
            vec![EligibleWorkerType::Human, EligibleWorkerType::AIAgent];
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

    pub fn capabilities(mut self, capabilities: Vec<CapabilityRequirement>) -> Self {
        self.job.required_capabilities = Some(capabilities);
        self
    }

    pub fn remote(mut self) -> Self {
        self.job.job_location_type = Some(vec![JobLocationType::Telecommute]);
        self
    }

    // AI Agent-specific builders
    pub fn response_time(mut self, value: f64, unit: String) -> Self {
        self.job.response_time_max = Some(PerformanceRequirement { value, unit });
        self
    }

    pub fn accuracy(mut self, value: f64) -> Self {
        self.job.accuracy_min = Some(PerformanceRequirement {
            value,
            unit: "PERCENT".to_string(),
        });
        self
    }

    pub fn throughput(mut self, value: f64, unit: String) -> Self {
        self.job.throughput_min = Some(PerformanceRequirement { value, unit });
        self
    }

    pub fn interface_type(mut self, interface: InterfaceType) -> Self {
        self.job.interface_type = Some(interface);
        self
    }

    pub fn protocol(mut self, protocol: String) -> Self {
        self.job.protocol = Some(protocol);
        self
    }

    pub fn human_oversight(mut self, oversight: OversightRequirement) -> Self {
        self.job.human_oversight = Some(oversight);
        self
    }

    // Nostr-specific builders
    pub fn nostr_pubkey(mut self, pubkey: String) -> Self {
        self.job.nostr_employer_pubkey = Some(pubkey);
        self
    }

    pub fn lightning_address(mut self, address: String) -> Self {
        self.job.lightning_address = Some(address);
        self
    }

    pub fn translations(mut self, translations: HashMap<String, TranslatedJob>) -> Self {
        self.job.translations = Some(translations);
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
    InvalidConfiguration(&'static str),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidationError::MissingField(field) => write!(f, "Missing required field: {}", field),
            ValidationError::InvalidDateFormat(field) => {
                write!(f, "Invalid date format in: {}", field)
            }
            ValidationError::InvalidUrl(field) => write!(f, "Invalid URL in: {}", field),
            ValidationError::InvalidConfiguration(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_job_listing() {
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
        .build();

        assert!(job.is_ok());
        let job = job.unwrap();
        assert_eq!(job.title, "Senior Rust Developer");
        assert_eq!(job.eligible_worker_type, vec![EligibleWorkerType::Human]);
        assert_eq!(
            job.job_location_type,
            Some(vec![JobLocationType::Telecommute])
        );
    }

    #[test]
    fn test_ai_agent_job_listing() {
        let job = JobListing::builder(
            "task-001".to_string(),
            "Image Classification Task".to_string(),
            "Classify product images".to_string(),
            "TaskPlatform".to_string(),
            "2025-01-15".to_string(),
            "https://api.taskplatform.com/apply".to_string(),
        )
        .for_ai_agents()
        .employment_type(vec![EmploymentType::TaskBased])
        .salary(0.05, 0.10, "USD".to_string(), "TASK".to_string())
        .response_time(5.0, "SECOND".to_string())
        .accuracy(95.0)
        .interface_type(InterfaceType::Api)
        .protocol("REST".to_string())
        .capabilities(vec![
            CapabilityRequirement {
                name: "Image Classification".to_string(),
                level: CapabilityLevel::Advanced,
            },
        ])
        .build();

        assert!(job.is_ok());
        let job = job.unwrap();
        assert_eq!(job.eligible_worker_type, vec![EligibleWorkerType::AIAgent]);
        assert!(job.response_time_max.is_some());
        assert!(job.accuracy_min.is_some());
    }

    #[test]
    fn test_hybrid_job_listing() {
        let job = JobListing::builder(
            "hybrid-001".to_string(),
            "Data Annotation with Review".to_string(),
            "AI agents annotate, humans review".to_string(),
            "Research Lab".to_string(),
            "2025-01-15".to_string(),
            "https://researchlab.edu/apply".to_string(),
        )
        .for_hybrid()
        .employment_type(vec![EmploymentType::Contractor])
        .human_oversight(OversightRequirement::Required)
        .build();

        assert!(job.is_ok());
        let job = job.unwrap();
        assert_eq!(job.eligible_worker_type.len(), 2);
        assert!(job.eligible_worker_type.contains(&EligibleWorkerType::Human));
        assert!(job.eligible_worker_type.contains(&EligibleWorkerType::AIAgent));
    }

    #[test]
    fn test_validation_missing_worker_type() {
        let mut job = JobListing::new(
            "job-001".to_string(),
            "Test".to_string(),
            "Description".to_string(),
            "Company".to_string(),
            "2025-01-15".to_string(),
            Some("https://example.com".to_string()),
            vec![],
        );

        assert!(job.validate().is_err());
    }

    #[test]
    fn test_validation_ai_fields_without_ai_worker() {
        let job = JobListing::builder(
            "job-001".to_string(),
            "Test".to_string(),
            "Description".to_string(),
            "Company".to_string(),
            "2025-01-15".to_string(),
            "https://example.com".to_string(),
        )
        .for_humans()
        .response_time(5.0, "SECOND".to_string())
        .build();

        assert!(job.is_err());
    }

    #[test]
    fn test_serialization() {
        let job = JobListing::builder(
            "job-001".to_string(),
            "Test Job".to_string(),
            "Test Description".to_string(),
            "Test Corp".to_string(),
            "2025-01-15".to_string(),
            "https://example.com/apply".to_string(),
        )
        .for_humans()
        .build()
        .unwrap();

        let json = serde_json::to_string_pretty(&job).unwrap();
        assert!(json.contains("JobPosting"));
        assert!(json.contains("schema.org"));
        assert!(json.contains("Human"));
    }
}

