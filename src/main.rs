mod prelude;
mod employee;
mod commands;
mod utils;

use prelude::*;
use commands::Opt;
use employee::*;
use utils::{capitalize_word, help_prompt};
use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut employees_list: HashMap<String, Vec<String>> = HashMap::new();
    help_prompt();

    loop {
        let input = get_input();
        if input.is_empty() {
            println!("{}", "[ERROR]: Input cannot be empty.".red().bold());
            continue;
        }

        match Opt::from_input(&capitalize_word(&input)) {
            Ok(command) => handle_command(command, &mut employees_list),
            Err(err) => print_error(err),
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    let arrow_emoji: Emoji = Emoji::new("\n▶ ", ">");
    print!("{} ", arrow_emoji.0);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn handle_command(command: Opt, employees: &mut HashMap<String, Vec<String>>) {
    match command {
        Opt::Add { name, dpt } => add_employee(employees, name, dpt),
        Opt::Remove { name, dpt } => remove_employee(employees, &name, &dpt),
        Opt::Count { dpt } => println!("{}", count_employees(employees, &dpt)),
        Opt::ListDepartments => available_departments(employees),
        Opt::Update { old_name, new_name, new_dpt } => update_employee(employees, old_name, new_name, new_dpt),
        Opt::Department(dpt) => list_department(employees, &capitalize_word(&dpt)),
        Opt::All => list_all(employees),
        Opt::Clear { dpt } => clear_department(employees, &dpt),
        Opt::Help => help_prompt(),
        Opt::Quit => {
            println!("Exiting...");
            std::process::exit(0);
        }
        Opt::ExportToFile(file_path) => {
            if let Err(e) = export_to_csv(employees, &file_path) {
                eprintln!("Error exporting data: {}", e);
            }
        }
    }
}

fn print_error(err: String) {
    let error_emoji: Emoji = Emoji::new("🚫", "[ERROR]");
    println!("\n{} {}: {}\n", error_emoji.0, "[ERROR]".red().bold(), err);
}
