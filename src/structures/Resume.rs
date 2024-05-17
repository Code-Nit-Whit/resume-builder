use crate::module_name::{Experience, Project, Skill, Interest, Education, Note, PersonalData};

struct Resume {
    //Internal Use
    uuid: String,
    name: String,
    job_position: String,
    employer: String,
    notes: Vec<Note>,
    // Componets
    personal_data: PersonalData,
    work_experiences: Vec<Experience>,
    projects: Vec<Project>,
    education: Vec<Education>,
    skills: Vec<Skill>,
    interests: Vec<Interest>,
}