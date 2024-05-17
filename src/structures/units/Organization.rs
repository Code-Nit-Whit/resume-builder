use crate::module_name::{Note, Individual};

enum OrganizationType {
    Business,
    Corporate,
    Nonprofit,
    Educational,
    Cooperative,
    Governmental,
    Foundation,
    Partnership,
    Consortium,
}

struct Organization{
    // Internal Use
    uuid: String,
    name: String,
    notes: Vec<Note>,
    // Components
    organization_name: String,
    organization_type: OrganizationType,
    description: String,
    address: String,
    phone_number: String,
    email_address: String,
    website_url: String,
    main_contact: Individual,
    contacts: Vec<Individual>,
}