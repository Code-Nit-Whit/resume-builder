use crate::module_name::{Skill, Note, Individual};

enum ProjectType {
    Volunteer,
    OpenSource,
    PersonalPortfolio,
    CommunityService,
    AcademicResearch,
    ExtraCurricular
}

struct Project {
    // Internal Use
    uuid: String,
    name: String,
    notes: Vec<Note>,
    // Components
    project_type: ProjectType,
    project_name: String,
    description: String,
    start_date: Date,
    end_date: Date,
    responsibilities: Vec<String>,
    achievements: Vec<String>,
    skills: Vec<Skill>,
    reason_left: String,
    contacts: Vec<Individual>,
    supervisor: Individual,
}