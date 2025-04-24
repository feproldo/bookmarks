pub fn one(args: Vec<String>) {
    if let None = args.get(1) {
        eprintln!("Invalid argument: <index or name>");
        std::process::exit(1);
    }
    if let Ok(index) = args.get(1).expect("Invalid argument: <index or name>").parse::<usize>() {
        let mut bookmarks = super::get_bookmarks();
        if bookmarks.bookmarks.len() <= index {
            eprintln!("Note inst fined");
            std::process::exit(1);
        }
        bookmarks.bookmarks.remove(index);
        super::write::write(bookmarks);
        println!("Successfully deleted.");
        std::process::exit(0);
    }
    else {
        let mut bookmarks = super::get_bookmarks();
        let mut index: usize = 0;
        for bookmark in &bookmarks.bookmarks {
            if bookmark.name == args.get(1).expect("Invalid argument: <index or name>").clone() {
                bookmarks.bookmarks.remove(index);
                super::write::write(bookmarks);
                println!("Successfully deleted.");
                std::process::exit(0);
            }
            index += 1;
        };
    }
    println!("Note isnt fined");
}