use serde::{Serialize, Deserialize};
use std::io;
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::num::ParseIntError;
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

#[derive(Serialize, Deserialize)]
struct Bookmark {
    title: String,
    link: String,
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
        title: "Hyouka - Saint Pepsi".to_string(),
        link: "https://www.youtube.com/watch?v=SpQCuPTWrrI".to_string(),
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
	// We don't actually need this 6th here - just a convenience
        6 => {
	    match generate_lum() {
		Ok(_) => (),
		Err(e) => println!("Error due to: {}", e),
	    }
	},
        _ => handle_options_from_user_input(),
    }
}

/*
 * @TODO: The whole "input" logic will soon be moved to a specific directory/file so I can have a better
 * controll on this section
 */
fn print_options() {
    println!("What is the operation?");
    println!("1 - Add new bookmark;");
    println!("2 - View bookmarks;");
    println!("3 - Delete a bookmark;");
    println!("4 - Purge bookmarks;");
    println!("5 - Quit;");
    println!("6 - Generate Lum;");
}

fn add_new_bookmark() {
    let mut file = match File::open("assets/lum-marker.json") {
	Ok(file) => file,
	Err(error) => {
	    println!("Error due to: {}", error);
		return;
	}
    };

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
    
    println!("Enter the title for the new bookmark:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read input.");
    let title = title.trim().to_string();
    
    println!("Enter the link for the new bookmark:");
    let mut link = String::new();
    io::stdin().read_line(&mut link).expect("Failed to read input.");
    let link = link.trim().to_string();
    
    let new_bookmark = Bookmark { title, link };
    
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

            println!("Bookmarks for {}: ", bookmark_data.owner);
            for (index, bookmark) in bookmark_data.bookmarks.iter().enumerate() {
                println!("{}. {}\n - {}\n", index + 1, bookmark.title, bookmark.link);
            }
        }
        Err(error) => {
            println!("Failed to read bookmark file due to: {}", error);
            return;
        }
    }
    handle_options_from_user_input();
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
