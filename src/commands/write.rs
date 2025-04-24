use super::BookmarkJSON;

pub fn add(mut args: Vec<String>, flags: Vec<String>) {
    args.remove(0);
    let name = args.get(0).expect("Invalid argument: <name>").clone();
    if let Ok(_) = name.parse::<usize>() {
        eprintln!("<name> must contain at least one char");
        std::process::exit(1);
    }
    let action = if args.len() > 2 { Some(args.get(1).expect("Invalid argument: [action]").clone()) } else { None };
    let content = if args.len() > 2 { args.get(2).expect("Invalid argument: <content>").clone() } else { args.get(1).expect("Invalid argument: <content>").clone() };
    let bookmark = super::Bookmark {
        name: name.clone(), action: action.clone(), content: content.clone()
    };
    let mut bookmark_json = super::get_bookmarks();
    let mut index = 0;
    let mut is_override = false;
    for bookmark in &bookmark_json.bookmarks {
        if bookmark.name == name {
            if !flags.contains(&"-o".to_string()) {
                eprintln!("Bookmark {} is already exists! Use -o flag to override bookmark.", name);
                std::process::exit(1);
            }
            else {
                is_override = true;
                break;
            }
        }
        index += 1;
    }
    if is_override {
        bookmark_json.bookmarks.remove(index);
    }
    let mut output = bookmark_json;
    output.bookmarks.push(bookmark);
    println!("writed {} - {} {}", name, action.unwrap_or("".to_string()), content);
    write(output);
}

pub fn write(bookmarks: BookmarkJSON) {
    std::fs::write(super::get_bookmark_file_path(), serde_json::to_string(&bookmarks).expect("error parsing to json")).expect("Error writing to file");
}