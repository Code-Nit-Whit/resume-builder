struct PersonalData {
    uuid: String,
    name: String,
    user_name: String,
    birthdate: Date,
    addresses: Vec<String>,
    phone_numbers: Vec<String>,
    email_addresses: Vec<String>,
    website_urls: Vec<String>,
    social_urls: Vec<String>,
    passions: Vec<String>,
    skills: Vec<Skill>,
    preferred_titles: Vec<String>,
}