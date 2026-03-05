# Contact Book CLI

A simple **Command Line Interface (CLI)** application written in Rust to manage contacts. It allows users to **add, list, search, and delete contacts** using commands and stores all data persistently in a local JSON file.

---

## Features

### 1. Add Contact

Add a new contact to the contact book.

**Command:**

```bash
rcontact add --name "Jane Marie" --phone "0771234567" --email "jane@gmail.com"
```

**Requirements:**

- name → Required
- phone → Required
- email → Optional
- Automatically assigns an incremental ID to each contact
- Stores data in contacts.json

### 2. List Contacts

Display all saved contacts in a clean table format.

**Command:**

```bash
rcontact list
```

**Example Output:**

```bash
ID | Name      | Phone       | Email
--------------------------------------------
1  | John Doe  | 0771234567  | john@gmail.com
```

Shows a 'No contacts found' message in red if no contacts exist

### 3. Search Contact

Search for contacts by partial or full name. The search is case-insensitive.

**Command:**

```bash
rcontact search --name "John"
```

Displays matching results in table format

Shows a 'No matiching contacts found' message if no matching contacts are found.

### 4. Delete Contact

Delete a contact by its ID.

**Command:**

```bash
rcontact delete --id 1
```

Deletes the contact with the specified ID
Shows a success message
Handles invalid IDs gracefully

## Technical Details

- Built with Rust
- Managed using Cargo
- Argument parsing using the clap crate
- Persistent storage using a local JSON file (contacts.json)
- Uses prettytable crate for displaying contacts in table format

## Installation

Clone the repository and build the binary:

```bash
git clone <repo_url>
cd rcontact
cargo build --release
```

Optionally, install globally:

```bash
cargo install --path . --force
```

Now you can use rcontact from anywhere:

```bash
rcontact add --name "John Doe" --phone "0771234567" --email "john@gmail.com"
```

**Usage**

**Add a Contact:**

```bash
rcontact add --name "Jane Doe" --phone "0777654321" --email "jane@gmail.com"
```

**List all Contacts:**

```bash
rcontact list
```

**Search Contacts by Name:**

```bash
rcontact search --name "Jane"
```

**Delete a Contact by ID:**

```bash
rcontact delete --id 1
```

**Example contacts.json**

```bash
[
    {
        "id": 1,
        "name": "John Doe",
        "phone": "0771234567",
        "email": "john@gmail.com"
    },
    {
        "id": 2,
        "name": "Jane Doe",
        "phone": "0777654321",
        "email": "jane@gmail.com"
    }
]
```

## Author

**Koshi**\n
**Contact Book CLI v1.0**\n
A simple Rust CLI project for managing contacts efficiently.


