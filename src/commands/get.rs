pub fn all() {
    let bookmark = super::get_bookmarks();
    if bookmark.bookmarks.is_empty() {
        eprintln!("bookmarks is empty!");
        std::process::exit(1);
    }
    else {
        let mut index: usize = 0;
        println!("{}", bookmark.bookmarks.iter().map(|el| {
            let mut output = format!("{}. {} - {}", index, el.name, el.content);
            if let Some(action) = &el.action {
                output = format!("{}. {} - {} {}", index, el.name, action, el.content);
            }
            index += 1;
            return format!("{}", output);
        }).collect::<Vec<String>>().join("\n"));
    }
}

pub fn by_index(index: usize) {
    let bookmark = super::get_bookmarks();
    if bookmark.bookmarks.is_empty() {
        eprintln!("bookmarks is empty!");
        std::process::exit(1);
    }
    else {
        if let Some(content) = bookmark.bookmarks.get(index) {
            println!("{}", content.content);
        }
        else {
            eprintln!("bookmark {} isn't fined", index);
            std::process::exit(1);
        }
    }
}

pub fn by_name(args: Vec<String>) {
    let name = args.get(1).expect("Invalid argument: <name>").clone();
    let bookmarks = super::get_bookmarks();
    let mut i: usize = 0;
    for bookmark in bookmarks.bookmarks {
        if bookmark.name == name {
            println!("{}", bookmark.content);
            break;
        }
        i += 1;
    }
}

pub fn choose_method(args: Vec<String>) {
    if let Some(first) = args.get(1) {
        if let Ok(index) = first.parse::<usize>() {
            by_index(index);
        }
        else {
            by_name(vec!["get".to_string(), first.clone()]);
        }
    }
    else {
        all();
    }
}