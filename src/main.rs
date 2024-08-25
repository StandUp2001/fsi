use std::{io::Write, usize};

macro_rules! my_matches {
    ($value:expr, $($arg:expr),*) => {
        {
            let lower = $value.to_lowercase();
            let a = lower.as_str();
            $(a == $arg.to_lowercase().as_str())||*
        }
    };
}

macro_rules! exit {
    ($code:expr, $( $msg:expr ),*) => {{
        println!($($msg),*);
        std::process::exit($code);
    }};
}

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
    if args.len() < 2 {
        exit!(1, "Usage: {} <program>", args[0]);
    }

    // TODO
    // check_for_updates();

    let output = search_program(&args[1]);
    let loos = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = loos.lines().collect();
    show_list(&lines);
    let input = read_input(lines.len());
    let program = lines[input - 1];
    println!("Installing: {}", program.tab(0));
    println!("{}\n", program);
    install_program(program);
}

#[allow(dead_code, unreachable_code)]
fn check_for_updates() {
    todo!("Not implemented yet");
    println!("Checking for updates...");
    // Show the list of updates
    let output = std::process::Command::new("flatpak")
        .arg("update")
        .output()
        .expect("Failed to execute command");
    let loos = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = loos.lines().collect();
    // if no updates available, return
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
        std::process::Command::new("flatpak")
            .arg("update")
            .arg("-y")
            .output()
            .expect("Failed to execute command");
    }

    println!("Updates checked successfully");
}

fn install_program(program: &str) {
    let app_id = program.tab(0);
    let app_name = program.tab(2);
    println!("Installing {}...", app_name);
    std::process::Command::new("flatpak")
        .arg("install")
        .arg("-y")
        .arg(app_id)
        .output()
        .expect("Failed to execute command");
    println!();
    println!("{} installed successfully", app_name);
}

fn show_list(lines: &Vec<&str>) {
    for (index, line) in lines.iter().enumerate() {
        println!("{:3} {:20} {}", index + 1, line.tab(0), line.tab(2));
    }
}

fn search_program(program: &str) -> std::process::Output {
    std::process::Command::new("flatpak")
        .arg("search")
        .arg(program)
        .output()
        .expect("Failed to execute command")
}

fn read_input(max: usize) -> usize {
    if max == 0 {
        exit!(1, "No program found");
    }

    if max == 1 {
        let input = ask_to_user("Do you want to install this program? (y/n): ");
        if my_matches!(input.trim(), "y", "yes", "") {
            return 1;
        }
        exit!(0, "Goodbye!");
    }

    loop {
        // print and prompt on the same line
        let input = ask_to_user("Enter the index of the program to install (q to quit): ");
        let trimmed = input.trim();
        if my_matches!(trimmed, "q", "quit") {
            exit!(0, "Goodbye!");
        }
        // check if the input is a number
        if !trimmed.chars().all(char::is_numeric) {
            println!("Invalid input. Please try again.");
            continue;
        }

        let index: usize = trimmed.parse().expect("Invalid input");
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
    input
}
