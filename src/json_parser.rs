// src/json_parser.rs
// Enhanced SimpleJobJson with full AI agent support
use serde::{Deserialize, Serialize};
use anyhow::Result; // Change this import
use crate::types::{
    CapabilityLevel, CapabilityRequirement, EligibleWorkerType, 
    EmploymentType, InterfaceType, JobListing, JobLocationType, 
    OversightRequirement,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedJobJson {
    #[serde(rename = "type")]
    pub job_type: String,
    pub version: u32,
    pub title: String,
    pub company: String,
    pub location: String,
    pub employment_type: String,
    pub description: String,
    
    // Core fields
    pub skills: Vec<String>,
    pub salary: SalaryJson,
    
    // AI Agent specific
    #[serde(default)]
    pub eligible_worker_type: Option<Vec<String>>,
    
    #[serde(default)]
    pub required_capabilities: Option<Vec<CapabilityJson>>,
    
    #[serde(default)]
    pub performance_requirements: Option<PerformanceRequirementsJson>,
    
    #[serde(default)]
    pub interface_requirements: Option<InterfaceRequirementsJson>,
    
    #[serde(default)]
    pub human_oversight: Option<String>,
    
    #[serde(default)]
    pub quality_assurance: Option<QualityAssuranceJson>,
    
    #[serde(default)]
    pub compliance: Option<ComplianceJson>,
    
    // Optional fields
    #[serde(default)]
    pub identifier: Option<String>,
    
    #[serde(default)]
    pub valid_through: Option<String>,
    
    #[serde(default)]
    pub lightning_address: Option<String>,
    
    #[serde(default)]
    pub apply_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleJobJson {
    #[serde(rename = "type")]
    pub job_type: String,
    pub version: u32,
    pub title: String,
    pub company: String,
    pub location: String,
    pub employment_type: String,
    pub description: String,
    pub skills: Vec<String>,
    pub salary: SalaryJson,
    #[serde(default)]
    pub identifier: Option<String>,
    #[serde(default)]
    pub valid_through: Option<String>,
    #[serde(default)]
    pub lightning_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryJson {
    pub currency: String,
    pub min: f64,
    pub max: f64,
    #[serde(default)]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityJson {
    pub name: String,
    pub level: String, // "basic", "intermediate", "advanced", "expert"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirementsJson {
    #[serde(default)]
    pub response_time_max: Option<MetricJson>,
    
    #[serde(default)]
    pub accuracy_min: Option<MetricJson>,
    
    #[serde(default)]
    pub throughput_min: Option<MetricJson>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricJson {
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceRequirementsJson {
    pub interface_type: String, // "API", "RPC", "WEBHOOK", "WEB_PORTAL"
    pub protocol: String,
    #[serde(default)]
    pub authentication: Option<String>,
    #[serde(default)]
    pub input_format: Option<String>,
    #[serde(default)]
    pub output_format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAssuranceJson {
    #[serde(default)]
    pub sample_review_rate: Option<f64>,
    #[serde(default)]
    pub appeal_process: Option<bool>,
    #[serde(default)]
    pub feedback_loop: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceJson {
    #[serde(default)]
    pub gdpr_compliant: Option<bool>,
    #[serde(default)]
    pub data_retention_days: Option<u32>,
    #[serde(default)]
    pub audit_logging: Option<bool>,
}

// Conversion function - Changed return type to anyhow::Result
pub fn convert_enhanced_to_job_listing(
    enhanced: &EnhancedJobJson,
    pubkey: &str,
) -> Result<JobListing> {  // Changed from Result<JobListing, Box<dyn std::error::Error>>
    use uuid::Uuid;
    
    // Generate UUID for uniqueness
    let uuid = Uuid::new_v4();
    let uuid_short = uuid.to_string().split('-').next().unwrap().to_string();
    
    let identifier = enhanced.identifier.clone().unwrap_or_else(|| {
        format!(
            "{}-{}-{}",
            enhanced.company.to_lowercase().replace(' ', "-"),
            enhanced
                .title
                .to_lowercase()
                .replace(' ', "-")
                .chars()
                .take(20)
                .collect::<String>(),
            uuid_short
        )
    });

    let date_posted = chrono::Utc::now().format("%Y-%m-%d").to_string();
    
    // Parse worker types
    let worker_types = if let Some(types) = &enhanced.eligible_worker_type {
        types
            .iter()
            .filter_map(|t| match t.to_uppercase().as_str() {
                "HUMAN" => Some(EligibleWorkerType::Human),
                "AIAGENT" | "AI_AGENT" | "AI" => Some(EligibleWorkerType::AIAgent),
                _ => None,
            })
            .collect()
    } else {
        vec![EligibleWorkerType::AIAgent] // Default for AI jobs
    };
    
    // Start building
    let mut builder = JobListing::builder(
        identifier,
        enhanced.title.clone(),
        enhanced.description.clone(),
        enhanced.company.clone(),
        date_posted,
        enhanced.apply_url.clone().unwrap_or_default(),
    )
    .eligible_worker_type(worker_types)
    .employment_type(vec![parse_employment_type(&enhanced.employment_type)])
    .location_type(vec![parse_location_type(&enhanced.location)])
    .skills(enhanced.skills.clone())
    .nostr_pubkey(pubkey.to_string());
    
    // Salary
    let salary_unit = enhanced
        .salary
        .unit
        .clone()
        .unwrap_or_else(|| "YEAR".to_string());
    builder = builder.salary(
        enhanced.salary.min,
        enhanced.salary.max,
        enhanced.salary.currency.clone(),
        salary_unit,
    );
    
    // Capabilities
    if let Some(caps) = &enhanced.required_capabilities {
        let parsed_caps: Vec<CapabilityRequirement> = caps
            .iter()
            .map(|c| CapabilityRequirement {
                name: c.name.clone(),
                level: parse_capability_level(&c.level),
            })
            .collect();
        builder = builder.capabilities(parsed_caps);
    }
    
    // Performance requirements
    if let Some(perf) = &enhanced.performance_requirements {
        if let Some(response_time) = &perf.response_time_max {
            builder = builder.response_time(response_time.value, response_time.unit.clone());
        }
        if let Some(accuracy) = &perf.accuracy_min {
            builder = builder.accuracy(accuracy.value);
        }
        if let Some(throughput) = &perf.throughput_min {
            builder = builder.throughput(throughput.value, throughput.unit.clone());
        }
    }
    
    // Interface requirements
    if let Some(interface) = &enhanced.interface_requirements {
        let interface_type = match interface.interface_type.to_uppercase().as_str() {
            "API" => InterfaceType::Api,
            "RPC" => InterfaceType::Rpc,
            "WEBHOOK" => InterfaceType::Webhook,
            "WEB_PORTAL" | "WEBPORTAL" => InterfaceType::WebPortal,
            _ => InterfaceType::Api,
        };
        builder = builder.interface_type(interface_type);
        builder = builder.protocol(interface.protocol.clone());
    }
    
    // Human oversight
    if let Some(oversight_str) = &enhanced.human_oversight {
        let oversight = match oversight_str.to_lowercase().as_str() {
            "required" => OversightRequirement::Required,
            "optional" => OversightRequirement::Optional,
            "none" => OversightRequirement::None,
            _ => OversightRequirement::Optional,
        };
        builder = builder.human_oversight(oversight);
    }
    
    // Optional fields
    if let Some(valid_through) = &enhanced.valid_through {
        builder = builder.valid_through(valid_through.clone());
    }
    
    if let Some(lightning) = &enhanced.lightning_address {
        builder = builder.lightning_address(lightning.clone());
    }
    
    Ok(builder.build()?)
}

pub fn parse_capability_level(s: &str) -> CapabilityLevel {
    match s.to_lowercase().as_str() {
        "basic" => CapabilityLevel::Basic,
        "intermediate" => CapabilityLevel::Intermediate,
        "advanced" => CapabilityLevel::Advanced,
        "expert" => CapabilityLevel::Expert,
        _ => CapabilityLevel::Intermediate,
    }
}

pub fn parse_employment_type(s: &str) -> EmploymentType {
    match s.to_lowercase().as_str() {
        "full-time" | "fulltime" => EmploymentType::FullTime,
        "part-time" | "parttime" => EmploymentType::PartTime,
        "contractor" | "contract" => EmploymentType::Contractor,
        "intern" | "internship" => EmploymentType::Intern,
        "temporary" | "temp" => EmploymentType::Temporary,
        "volunteer" => EmploymentType::Volunteer,
        "task-based" | "taskbased" => EmploymentType::TaskBased,
        "micro-task" | "microtask" => EmploymentType::MicroTask,
        _ => EmploymentType::Other,
    }
}

pub fn parse_location_type(s: &str) -> JobLocationType {
    match s.to_lowercase().as_str() {
        "remote" | "telecommute" => JobLocationType::Telecommute,
        "onsite" | "on-site" | "office" => JobLocationType::OnSite,
        "hybrid" => JobLocationType::Hybrid,
        _ => JobLocationType::Telecommute,
    }
}
