use aegis_vault::{
    algorithm::Method,
    vault::{Aegis, Entry},
};
use keepass::{db::NodeRef, Database};

pub fn export_database(database: &Database, password: &str) -> String {
    let mut aegis_root = Aegis::default();

    for entry in export_group(&database.root) {
        aegis_root.add_entry(entry);
    }

    aegis_root.encrypt(password).unwrap();

    let raw_encrypted_vault = serde_json::ser::to_string_pretty(&aegis_root).unwrap();

    raw_encrypted_vault
}

fn export_group(group: &keepass::db::Group) -> Vec<Entry> {
    let mut entries: Vec<Entry> = vec![];
    for child in &group.children {
        match child {
            keepass::db::Node::Group(g) => {
                for entry in export_group(g) {
                    entries.push(entry);
                }
            }
            keepass::db::Node::Entry(e) => {
                if let Some(entry) = export_entry(e) {
                    entries.push(entry);
                }
            }
        }
    }
    entries
}

fn export_entry(entry: &keepass::db::Entry) -> Option<Entry> {
    if let Ok(totp) = entry.get_otp() {
        let entry_title = match entry.get_title() {
            Some(t) => t.to_string(),
            None => entry.get_uuid().to_string(),
        };
        println!("Exporting TOTP for entry {}", entry_title);

        let mut aegis_entry = Entry::default();
        aegis_entry.method = Method::TOTP;
        aegis_entry.label = entry_title.to_string();
        aegis_entry.issuer = Some(totp.issuer.clone());
        aegis_entry.info.secret = totp.get_secret();
        aegis_entry.info.period = Some(totp.period.try_into().unwrap());
        aegis_entry.info.digits = totp.digits;
        aegis_entry.info.counter = None;
        return Some(aegis_entry);
    }
    None
}
