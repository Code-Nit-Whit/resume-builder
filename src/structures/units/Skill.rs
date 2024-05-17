use crate::module_name::{Note, Education, Skill};

enum SkillHardness {
    Soft,
    Hard
}

enum SkillLevel {
    EntryLevel,
    Basic, 
    Intermediate,
    Proficient,
    Advanced,
    Expert,
    Foundational,
    Skilled,
    HighlySkilled,
    Seasoned,
    Specialist
}

struct Skill {
    //Internal Use
    uuid: String,
    name: String,
    notes: Vec<Note>,
    // Components
    industry: String,
    category: String,
    description: String,
    hardness: SkillHardness,
    skill_level: SkillLevel,
    tools: Vec<String>,
    obtained_from: String,
    years: Integer,
    education: Vec<Education>, 
    dependancy_skills: Vec<Skill>,
}