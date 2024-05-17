use crate::module_name::{Skill, Note, Individual, Organization, Project};

enum EmploymentType {
    FullTime,
    PartTime,
    Temporary,
    Contract,
    Internship,
    Apprenticeship,
    Freelance,
    SelfEmployed,
    Other,
}

struct Experience {
    //Internal Use
    uuid: String,
    name: String,
    notes: Vec<Note>,
    // Components
    organizations: Vec<Organization>,
    description: String,
    start_date: Date,
    end_date: Date,
    titles: Vec<String>,
    employment_type: EmploymentType,
    responsibilities: Vec<String>,
    achievements: Vec<String>,
    projects: Vec<Project>,
    skills: Vec<Skill>,
    reason_left: String,
    contacts: Vec<Individual>,
    supervisor: Individual,
}