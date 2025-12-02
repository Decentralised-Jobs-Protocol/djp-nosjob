// src/lib.rs
#![allow(unused_mut)]

pub mod types;
pub mod events;

// Re-export commonly used types
pub use types::{
    JobListing, 
    EmploymentType, 
    JobLocationType,
    HiringOrganization,
    JobLocation,
    BaseSalary,
    ValidationError,
};

pub use events::JobsFilter;