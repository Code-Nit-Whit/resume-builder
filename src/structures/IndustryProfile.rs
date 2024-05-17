use crate::module_name::{PersonalData, Skill, Interest, Passion, Education, Experience, Resume, Note};

struct IndustryProfile {
    //Internal Use
    uuid: String,                     // String containing a unnique identifier for the IndustryProfile instance
    name: String,                     // String containing the name of the IndustryProfile instance
    industry_type: String,            // Single enum instance representing industry type
    description: String,              // String containing the description of the IndustryProfile instance
    notes: Vec<Note>,                 // Vector to hold multiple multipurpose notes
    // Resume Building Componets
    personal_data: Vec<PersonalData>, // Vector to hold multiple personal data entries
    education: Vec<Education>,        // Vector to hold multiple education entries
    experience: Vec<Experience>,      // Vector to hold multiple work experiences
    projects: Vec<Project>,           // Vector to hold multiple projects
    skills: Vec<Skill>,               // Vector to hold multiple skills
    interests: Vec<Interest>,         // Vector to hold multiple interests
    //Resume Storage
    resumes: Vec<Resume>,             // Vector to hold multiple tailored resumes
}
