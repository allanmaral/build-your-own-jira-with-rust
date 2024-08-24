#![allow(dead_code)]
#![allow(unused_doc_comments)]
use directories::ProjectDirs;
use std::error::Error;
use std::path::PathBuf;

/// The `main` function is the entry point of your application.
///
/// It gets called when you invoke `cargo run --bin jira-wip` and
/// its executed when a user runs the binary you generated by compiling your project
/// (`cargo build` -> `/target/debug/jira-wip` / `cargo build --release` -> `/target/release/jira-wip`)
///
/// Over the course of this workshop we will modify this entry point to build a fully fledged
/// command line application.
///
/// Brace yourself!
fn main() -> Result<(), Box<dyn Error>> {
    // Uncomment these lines after 02_ticket_store/09_store_recap

    // Comment these line after 03_cli/01_persistence
    // use path_to_enlightenment::store_recap::TicketStore;
    // let mut ticket_store = TicketStore::new();

    use path_to_enlightenment::persistence::{load, save};
    // Load the store from disk. If missing, a brand new one will be created.
    let mut ticket_store = load(&data_store_filename());

    use path_to_enlightenment::cli::{handle_command, Command};
    // Parse the command-line arguments.
    let command = <Command as paw::ParseArgs>::parse_args()?;
    handle_command(&mut ticket_store, command)?;

    // Save the store state to disk after we have completed our action.
    save(&ticket_store, &data_store_filename());
    Ok(())
}

mod path_to_enlightenment;

// `PROJECT_NAME`, `ORGANISATION_NAME` and `QUALIFIER` are used to determine
// where to store configuration files and secrets for an application
// according to the convention of the underlying operating system.
//
// `qualifier_name` is only relevant for MacOS - we leave it blank.
const PROJECT_NAME: &str = "IronJIRAWip";
const ORGANISATION_NAME: &str = "RustLDNUserGroup";
const QUALIFIER: &str = "";

const TICKET_STORE: &str = "ticket_store.yaml";

/// Determine the right location to store data based on the user OS.
/// It relies on the `directories` crate - see https://crates.io/crates/directories for more information.
fn data_store_filename() -> PathBuf {
    // Get the directory where we are supposed to store data
    // according to the convention of the underlying operating system.
    //
    // The operation could fail if some OS environment variables are not set (e.g. $HOME)
    let project_dir = ProjectDirs::from(QUALIFIER, ORGANISATION_NAME, PROJECT_NAME)
        .expect("Failed to determine path of the configuration directory.");
    let data_dir = project_dir.data_dir();
    println!("Data storage directory: {:?}", data_dir);

    // Create the data directory, if missing.
    // It also takes care of creating intermediate sub-directory, if necessary.
    std::fs::create_dir_all(data_dir).expect("Failed to create data directory.");

    // Path to the file storing our tickets
    data_dir.join(TICKET_STORE)
}
