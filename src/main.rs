use serde::Serialize;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::num::ParseIntError;
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

#[derive(Serialize)]
struct Bookmark {
    title: String,
    link: String,
}

#[derive(Serialize)]
struct BookmarkData {
    owner: String,
    created_at: u64,
    bookmarks: Vec<Bookmark>,
}

fn create_json_file() -> std::io::Result<()> {
    let computer_name = env::var("COMPUTERNAME").unwrap_or_else(|_| "admin".to_string());

    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let bookmark = Bookmark {
        title: "Hyouka - Saint Pepsi".to_string(),
        link: "https://www.youtube.com/watch?v=SpQCuPTWrrI".to_string(),
    };

    let bookmark_data = BookmarkData {
        owner: computer_name,
        created_at,
        bookmarks: vec![bookmark],
    };

    let json_data = serde_json::to_string_pretty(&bookmark_data)?;
    let mut file = File::create("bookmark.json")?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

fn handle_options_from_user_input() {
    let mut parsed_input: Result<u32, ParseIntError>;
    print_options();

    while {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input. Failed to read line.");

        parsed_input = input.trim().parse();
        let parsed_input_clone = parsed_input.clone();
        parsed_input.is_err() || !(1..=10).contains(&parsed_input_clone.unwrap())
    } {
        println!("Invalid input. Please enter a valid option!");
        print_options();
    }

    match parsed_input.unwrap() {
        1 => add_new_bookmark(),
        2 => view_bookmarks(),
        3 => delete_bookmark(),
        4 => purge_bookmarks(),
        5 => quit(),
        _ => handle_options_from_user_input(),
    }
}

// The whole "input" logic will soon be moved to a specific directory/file so I can have a better
// controll on this section -> src/input.rs maybe
fn print_options() {
    println!("What is the operation?");
    println!("1 - Add new bookmark;");
    println!("2 - View bookmarks;");
    println!("3 - Delete a bookmark;");
    println!("4 - Purge bookmarks;");
    println!("5 - Quit;");
}

fn add_new_bookmark() {
    println!("Adding a new bookmark now...");
}

fn view_bookmarks() {
    println!("Viewing bookmarks now...");
}

fn delete_bookmark() {
    println!("Deleting bookmark now...");
}

fn purge_bookmarks() {
    println!("Purging bookmarks now...");
}

fn quit() {
    println!("Goodbye... For now...");
}

fn main() {
    handle_options_from_user_input();
}
