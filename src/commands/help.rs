pub fn help() {
    let program_name: String = String::from("bookmarks");
    println!("Hello! This is what you can use:
    {program_name} - displays all notes
    {program_name} get <index or name> - displays note by its name or index
    {program_name} [index or name] - displays note by its name or index (short alias)
    {program_name} set <name> [action] <\"content\"> - add a note. (You can use -o to override existing note)
    {program_name} delete <index or name> - delete note by its name or index
    {program_name} open - execute the \"action\" row. WIP!!!
    {program_name} help - shows this text")
}