use crate::module_name::Note;

struct PersonalData {
    // Internal Use
    uuid: String,
    name: String,
    notes: Vec<Note>,
    // Componets
    user_name: String,
    preferred_titles: Vec<String>,
    bios : Vec<String>,
    birthdate: Date,
    addresses: Vec<String>,
    phone_numbers: Vec<String>,
    email_addresses: Vec<String>,
    website_urls: Vec<String>,
    social_handles: Vec<String>,
}