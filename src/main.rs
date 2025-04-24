mod commands;
fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    commands::check_file();
    args.remove(0);
    let mut positional_args = Vec::new();
    let mut flags = Vec::new();
    for arg in &args {
        if arg.starts_with("-") {
            flags.push(arg.clone());
        }
        else {
            positional_args.push(arg.clone());
        }
    }
    commands::match_args(positional_args, flags);
}