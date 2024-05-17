use crate::module_name::{Skill, Note, Individual, Organization, Project};

enum EducationType {
    College,
    University,
    HighSchool,
    VocationalSchool,
    OnlineCourses,
    SelfTaught,
    Apprenticeship,
    MilitaryTraining,
    Other,
}

enum DegreeType {
    Certificate,
    Certification,
    Associates,
    Bachelors,
    Masters,
    Doctorate,
    ProfessionalDegree,
    Diploma,
    Other,
}

struct Education {
    //Internal Use
    uuid: String, 
    name: String,
    description: String,
    notes: Vec<Note>,
    // Components
    education_type: EducationType,
    institution: Organization,
    degree_type: DegreeType,
    degree_name: String,
    area: String,
    major: String,
    minor: String,
    start_date: Date,
    end_date: Date,
    achievements: Vec<String>,
    skills: Vec<Skill>,
    projects: Vec<Project>,
    gpa: Float,
    contacts: Vec<Individual>,
    main_contact: Individual,
}