use crate::module_name::Note;

struct Note {
    uuid: String,
    name: String,
    purpose: String,
    body: String,
    notes: Vec<Note>,
}