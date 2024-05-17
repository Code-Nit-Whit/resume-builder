use crate::module_name::{Note, Organization};

struct Individual {
    // Internal Use
    uuid: String,
    name: String,
    notes: Vec<Note>,
    // Components
    individual_name: String,
    title: String,
    bios : String,
    birthdate: Date,
    addresses: Vec<String>,
    phone_numbers: Vec<String>,
    email_addresses: Vec<String>,
    organizations: Vec<Organization>,
    relationships: Vec<String>,
}
