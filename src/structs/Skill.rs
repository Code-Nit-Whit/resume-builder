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
    uuid: String,
    name: String,
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