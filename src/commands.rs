use crate::contact::Contact;
use crate::storage::{load_contacts, save_contacts};
use prettytable::{Table, row};
use colored::*;

pub fn add_contact(name:String, phone:String, email:Option<String>){
    let mut contacts = load_contacts();
    let new_id = contacts.last().map(|c| c.id + 1).unwrap_or(1);

    let contact = Contact {
        id:new_id,
        name,
        phone,
        email,
    };

    contacts.push(contact);
    save_contacts(&contacts);

    println!("{}", "Contact added successfully!".green());
}

pub fn list_contacts(){
    let contacts = load_contacts();

    if contacts.is_empty(){
        println!("{}", "No contacts found.".red());
        return;
    }

    let mut table = Table::new();
    table.add_row(row!["ID", "Name", "Phone", "Email"]);

    for c in contacts{
        table.add_row(row![
            c.id,
            c.name,
            c.phone,
            c.email.unwrap_or("-".to_string())
        ]);
    }

    table.printstd();
}

pub fn search_contacts(search_name: String){
    let contacts = load_contacts();

    let results: Vec<Contact> = contacts
        .into_iter()
        .filter(|c| c.name.to_lowercase().contains(&search_name.to_lowercase()))
        .collect();

    if results.is_empty(){
        println!("{}", "No matiching contacts found.".red());
        return;
    }

    let mut table = Table::new();
    table.add_row(row!["ID", "Name", "Phone", "Email"]);

    for c in results{
        table.add_row(row![
            c.id,
            c.name,
            c.phone,
            c.email.unwrap_or("-".to_string())
        ]);
    }

    table.printstd();
}

pub fn delete_contact(id: u32){
    let mut contacts = load_contacts();
    let initial_len = contacts.len();

    contacts.retain(|c| c.id != id);

    if contacts.len() == initial_len{
        println!("{}", format!("No contact found with the ID {}", id).red());
        return;
    }

    save_contacts(&contacts);
    println!("{}", "Contact deleted successfully!".green())
}