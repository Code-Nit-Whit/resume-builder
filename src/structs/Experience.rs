struct Experience {
    uuid: String,
    organizations: Vec<Organization>,
    titles: Vec<String>,
    statuses: Enum<EmploymentStatus>,
    start_date: Date,
    end_date: Date,
    responsibilities: Vec<Responsibility>,
    achievements: Vec<Achievement>,
    description: String,
    reason_left: String,
    immediate_supervisors: Vec<Individual>,
    main_contact: Individual,
    personal_notes: Vec<String>,
}