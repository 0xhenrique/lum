use serde::{Serialize, Deserialize};
// use std::io;
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

#[derive(Serialize, Deserialize)]
struct Bookmark {
    link: String,
    created_at: u64,
    last_updated: u64,
}

#[derive(Serialize, Deserialize)]
struct BookmarkData {
    owner: String,
    created_at: u64,
    bookmarks: Vec<Bookmark>,
}

fn generate_lum() -> std::io::Result<()> {
    let computer_name = env::var("OWNER").unwrap_or_else(|_| "admin".to_string());

    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let bookmark = Bookmark {
        link: "https://github.com/henrique-marques-vsoft/lum".to_string(),
	created_at,
	last_updated: created_at,
    };

    let bookmark_data = BookmarkData {
        owner: computer_name,
        created_at,
        bookmarks: vec![bookmark],
    };

    let json_data = serde_json::to_string_pretty(&bookmark_data)?;
    let mut file = File::create("assets/lum-marker.json")?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

fn parse_cli_args() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-l" | "--list" => {
		view_bookmarks();
            }
            "-h" | "--help" => {
		help();
            }
            "-g" | "--generate" => {
		let _ = generate_lum();
            }
            "-a" | "--add" => {
		add_new_bookmark(args[2].to_string());
            }
            "-d" | "--delete" => {
		delete_bookmark(args[2].to_string());
            }
            _ => {
                println!("Invalid option. Use -h to see available options.");
            }
        }
    } else {
        println!("Use -h to see available options.");
    }
}

fn help() {
    println!("
Lum - Linux Ubiquitous Marker\n
This project was created for two main reasons: I wanted to mess a little bit with Rust and I wanted to have bookmarks outside the Web Browser.
I plan to integrate Lum to other programs such as rofi/wofi, Emacs etc.\n
Usage: lum [OPTION] value \n
OPTIONS:
  -l, --list      -    Lists all available bookmarks
  -h, --help      -    Shows this help output
  -g, --generate  -    Generate the Bookmark file
  -a, --add       -    Add a new bookmark to an already existent bookmark file
  -d, --delete    -    Delete a bookmark by its index
    ")
}

fn add_new_bookmark(link: String) {
    let mut file = match File::open("assets/lum-marker.json") {
	Ok(file) => file,
	Err(error) => {
	    println!("Error due to: {}", error);
		return;
	}
    };

    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let last_updated = created_at;

    let mut contents = String::new();
    if let Err(error) = file.read_to_string(&mut contents) {
	println!("Failed due to: {}", error);
	return;
    };

    let mut bookmark_data: BookmarkData = match serde_json::from_str(&contents) {
        Ok(data) => data,
        Err(error) => {
            println!("Failed to parse bookmark data due to: {}", error);
            return;
        }
    };
    
    let new_bookmark = Bookmark { link, created_at, last_updated};
    
    bookmark_data.bookmarks.push(new_bookmark);
    
    let new_contents = match serde_json::to_string(&bookmark_data) {
        Ok(data) => data,
        Err(error) => {
            println!("Failed to serialize bookmark data due to: {}", error);
            return;
        }
    };
    
    let mut file = match OpenOptions::new().write(true).truncate(true).open("assets/lum-marker.json") {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open bookmark file for writing due to: {}", error);
            return;
        }
    };
    
    if let Err(error) = file.write_all(new_contents.as_bytes()) {
        println!("Failed to write updated bookmark data to file due to: {}", error);
        return;
    }
    
    println!("New bookmark added successfully.");
}

fn view_bookmarks() {
    let mut file = match File::open("assets/lum-marker.json") {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open bookmark file due to: {}", error);
            return;
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            let bookmark_data: BookmarkData = match serde_json::from_str(&contents) {
                Ok(data) => data,
                Err(error) => {
                    println!("Failed to parse bookmark data due to: {}", error);
                    return;
                }
            };

            for (index, bookmark) in bookmark_data.bookmarks.iter().enumerate() {
                println!("{}. - {}\n", index, bookmark.link);
            }
        }
        Err(error) => {
            println!("Failed to read bookmark file due to: {}", error);
            return;
        }
    }
}

fn delete_bookmark(index: String) {
    let selected_index: usize = index.parse().expect("You need the specify the index of the bookmark.");

    let mut file = match File::open("assets/lum-marker.json") {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open bookmark file: {}", error);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(error) = file.read_to_string(&mut contents) {
        println!("Failed to read bookmark file: {}", error);
        return;
    }

    let mut bookmark_data: BookmarkData = match serde_json::from_str(&contents) {
        Ok(data) => data,
        Err(error) => {
            println!("Failed to parse bookmark data: {}", error);
            return;
        }
    };

    if selected_index >= bookmark_data.bookmarks.len() {
        println!("Index out of bounds. This bookmark probably DO NOT exist.");
        return;
    }

    bookmark_data.bookmarks.remove(selected_index);

    let new_contents = match serde_json::to_string(&bookmark_data) {
        Ok(data) => data,
        Err(error) => {
            println!("Failed to serialize bookmark data: {}", error);
            return;
        }
    };

    let mut file = match OpenOptions::new().write(true).truncate(true).open("assets/lum-marker.json") {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open bookmark file for writing: {}", error);
            return;
        }
    };

    if let Err(error) = file.write_all(new_contents.as_bytes()) {
        println!("Failed to write updated bookmark data to file: {}", error);
        return;
    }

    println!("Bookmark deleted successfully.");
}

fn main() {
    parse_cli_args();
}
