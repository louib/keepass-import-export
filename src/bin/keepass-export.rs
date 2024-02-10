/// utility to dump keepass database internal XML data.
use std::fs::File;
use std::io::Write;

use anyhow::Result;
use clap::Parser;
use keepass::{db::NodeRef, Database, DatabaseKey};

#[derive(Parser)]
#[derive(Debug)]
#[command(version, about)]
struct Args {
    /// Provide a .kdbx database
    in_kdbx: String,

    /// Provide a keyfile
    #[arg(short = 'k', long)]
    keyfile: Option<String>,

    /// Format to export the database to.
    #[arg(short = 'f', long)]
    format: String,

    /// A comma-separated list of tags to exclude from the export
    #[arg(long)]
    exclude_tags: String,

    /// Provide an output file
    out_file: String,
}

fn main() -> Result<std::process::ExitCode> {
    let args = Args::parse();

    if args.format != "aegis" {
        panic!("This tool only supports the aegis format for the moment.");
    }

    let mut source = File::open(args.in_kdbx.clone())?;
    let mut key = DatabaseKey::new();

    if let Some(f) = args.keyfile {
        key = key.with_keyfile(&mut File::open(f)?)?;
    }

    let password =
        rpassword::prompt_password("Password (or blank for none): ").expect("Could not read password from TTY");

    key = key.with_password(&password);

    let mut db = Database::open(&mut source, key)?;
    let exclude_tags = args.exclude_tags.split(',').collect::<Vec<&str>>();

    let password = rpassword::prompt_password("Entry password for the exported database (or blank for none): ")
        .expect("Could not read password from TTY");

    let out_db = keepass_import_export::aegis::export_database(&db, &password, exclude_tags);

    let mut out_file = File::options().write(true).create(true).open(args.out_file)?;
    out_file.write_all(out_db.as_bytes())?;
    println!("Database was exported successfully.");

    Ok(std::process::ExitCode::SUCCESS)
}
