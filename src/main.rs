use std::{io::Write, usize};

mod macros;

trait Tab {
    fn tab(&self, index: usize) -> &str;
}

impl Tab for &str {
    fn tab(&self, index: usize) -> &str {
        let list: Vec<&str> = self.split('\t').collect();
        if list.len() < index {
            return "";
        }
        list[index]
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in args.iter() {
        let arg = arg.as_str().to_lowercase();
        if arg == "--help" || arg == "-h" {
            println!("Usage: {} <program> [flags]", args[0]);
            std::process::exit(0);
        }
    }
    if args.len() < 2 {
        println!("Usage: {} <program> [flags]", args[0]);
        exit!(1, "No program specified");
    }

    let output = command!("which", "flatpak");
    if !output.status.success() {
        exit!(1, "Flatpak is not installed");
    }

    if args.len() == 3 && (args[2].to_lowercase() == "--update" || args[2].to_lowercase() == "-u") {
        check_for_updates();
    }

    println!("Searching for program -> {}", args[1]);
    let output = command!("flatpak", "search", &args[1]);
    let loos = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = loos.lines().collect();

    for (index, line) in lines.iter().enumerate() {
        println!("{:3} {:20} {}", index + 1, line.tab(0), line.tab(2));
    }

    let input = read_input(lines.len());
    let program = lines[input - 1];

    println!("Installing: {}", program.tab(0));
    println!("{}\n", program);
    install_program(program);
}

fn check_for_updates() {
    println!("Checking for updates...");
    let output = command!("flatpak", "update");
    let loos = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = loos.lines().collect();

    if lines.last().unwrap() == &"Nothing to do." {
        println!("No updates available");
        return;
    }

    if lines.len() > 1 {
        println!("Updates available:");
        for line in lines.iter().skip(1) {
            println!("{}", line);
        }
        println!("Updating...");
        command!("flatpak", "update", "-y");
    }

    println!("Updates checked successfully");
}

fn install_program(program: &str) {
    let app_id = program.tab(0);
    let app_name = program.tab(2);
    println!("Installing {}...", app_name);
    command!("flatpak", "install", "-y", app_id);
    println!("\n{} installed successfully", app_name);
}

fn read_input(max: usize) -> usize {
    if max == 0 {
        exit!(1, "No program found");
    }

    if max == 1 {
        let input = ask_to_user("Do you want to install this program? (y/n): ");
        if my_matches!(input, "y", "yes", "") {
            return 1;
        }
        exit!(0, "Goodbye!");
    }

    loop {
        // print and prompt on the same line
        let input = ask_to_user("Enter the index of the program to install (q to quit): ");
        if my_matches!(input, "q", "quit", "esc") {
            exit!(0, "Goodbye!");
        }
        // check if the input is a number
        if !input.chars().all(char::is_numeric) {
            println!("Invalid input. Please try again.");
            continue;
        }

        let index: usize = input.parse().expect("Invalid input");
        if 1 <= index && index <= max {
            return index;
        }
        println!("Invalid index. Please try again.");
    }
}

fn ask_to_user(msg: &str) -> String {
    let mut input = String::new();
    print!("\n{}", msg);
    std::io::stdout().flush().expect("Failed to flush stdout");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!();
    input.trim().to_string()
}
