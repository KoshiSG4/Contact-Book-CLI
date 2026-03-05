use std::fs;
use std::path::Path;
use crate::contact::Contact;

const FILE_PATH: &str = "contacts.json";

pub fn load_contacts() -> Vec<Contact> {
    if !Path::new(FILE_PATH).exists(){
        return Vec::new();
    }

    let data = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())

}

pub fn save_contacts(contacts: &Vec<Contact>){
    let json = serde_json::to_string_pretty(contacts).unwrap();
    fs::write(FILE_PATH, json).expect("Unable to write file");
}