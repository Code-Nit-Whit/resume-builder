use crate::module_name::Note;

struct Interest {
    // Internal Use
    uuid: String,
    name: String,
    notes: Vec<Note>,
    // Components
    interest_name: String,
    interest_category: String,
    longevity: String,
    description: String,
}