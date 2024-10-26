use crate::prelude::*;

pub fn capitalize_word(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn help_prompt() {
    println!("\nType {} to add an employee", "'Add <name> to <department>'".blue().bold());
    println!("Type {} to remove an employee", "'Remove <name> from <department>'".blue().bold());
    println!("Type {} to count the number of employees in a department", "'Count <department>'".blue().bold());
    println!("Type {} to list all departments", "'ListDepartments'".blue().bold());
    println!("Type {} to update an employee or to move it to another department", "'Update <name> to <name> in <department>'".blue().bold());
    println!("Type {} to remove all employees from a department", "'Clear <department>'".blue().bold());
    println!("Type {} to see all employees and their departments", "'All'".blue().bold());
    println!("Type {} to list all employees of a department", "'<department>'".blue().bold());
    println!("Type {} to export the employee data to CSV", "'Export to <filename.csv>'".blue().bold());
    println!("Type {} or {} to show this prompt", "'Help'".green().bold(), "'?'".yellow().bold());
    println!("Type {} to quit\n", "'Quit'".red().bold());
}
