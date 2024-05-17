enum IndustryType {
    
}

struct IndustryProfile {
    uuid: String,                     // String containing a unnique identifier for the IndustryProfile instance
    name: String,                     // String containing the name of the IndustryProfile instance
    industry_type: Enum<IndustryType>,// Single enum instance representing industry type
    personal_data: Vec<PersonalData>, // Vector to hold multiple personal data entries
    skills: Vec<Skill>,               // Vector to hold multiple skills
    education: Vec<Education>,        // Vector to hold multiple education entries
    experience: Vec<Experience>,      // Vector to hold multiple work experiences
    resumes: Vec<Resume>,             // Vector to hold multiple tailored resumes
}
