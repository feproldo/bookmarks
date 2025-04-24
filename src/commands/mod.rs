use std::path::PathBuf;

use serde::{Deserialize, Serialize};
mod help;
mod delete;
mod get;
mod write;

#[derive(Serialize, Deserialize)]
pub struct Bookmark {
    pub name: String,
    pub content: String,
    pub action: Option<String>
}
#[derive(Serialize, Deserialize)]
pub struct BookmarkJSON {
    pub bookmarks: Vec<Bookmark>
}

pub fn match_args(args: Vec<String>, flags: Vec<String>) {
    if args.is_empty() {
        get::all();
    }
    else {
        if let Some(first) = args.get(0) {
            match first.as_str() {
                "set" => write::add(args.clone(), flags.clone()),
                "get" => get::choose_method(args.clone()),
                "delete" => delete::one(args.clone()),
                "help" => help::help(),
                "open" => {eprintln!("todo")}
                _ => {
                    get::choose_method(args);
                }
            }
        }
        else {
            get::choose_method(args);
        }
    }
}

pub fn get_bookmarks() -> BookmarkJSON {
    if let Ok(bookmarks) = std::fs::read_to_string(get_bookmark_file_path()) {
        if let Ok(bookmark_json) = serde_json::from_str::<BookmarkJSON>(bookmarks.as_str()) {
            return bookmark_json;
        }
        else {
            eprintln!("error parse bookmarks");
            std::fs::remove_file(get_bookmark_file_path()).expect("error deleting bookmarks.json");
            std::process::exit(1);
        }
    }
    else {
        eprintln!("error getting bookmarks");
        std::fs::remove_file(get_bookmark_file_path()).expect("error deleting bookmarks.json");
        std::process::exit(1);
    }
}

pub fn get_bookmark_file_path() -> PathBuf {
    let home = home::home_dir().expect("HOME environment variable not set");
    PathBuf::from(home).join(".local/share/bookmark/bookmarks.json")
}

pub fn get_bookmark_dir_path() -> PathBuf {
    let home = home::home_dir().expect("HOME environment variable not set");
    PathBuf::from(home).join(".local/share/bookmark")
}

pub fn check_file() {
    if let Ok(false) = std::fs::exists(get_bookmark_file_path()) {
        std::fs::create_dir_all(get_bookmark_dir_path()).expect(format!("Cant create bookmark directory in {}", get_bookmark_dir_path().to_str().unwrap_or("idk")).as_str());
        std::fs::write(get_bookmark_file_path(), "{\"bookmarks\": []}").expect(format!("Cant create bookmark file in {}", get_bookmark_dir_path().to_str().unwrap_or("idk")).as_str());
    }
}