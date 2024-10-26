use crate::prelude::*;

pub fn export_to_csv(employees: &HashMap<String, Vec<String>>, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = Writer::from_writer(File::create(file_path)?);
    
    wtr.write_record(&["Department", "Employee"])?;

    for (department, employee_list) in employees {
        for employee in employee_list {
            wtr.write_record(&[department, employee])?;
        }
    }

    wtr.flush()?;
    println!("Data exported successfully to {}.", file_path);
    Ok(())
}



pub fn add_employee(employees: &mut HashMap<String, Vec<String>>, name: String, dpt: String) {
    employees.entry(dpt.clone()).or_insert_with(Vec::new).push(name);
    let check_emoji: Emoji = Emoji::new("✅", "v");
    println!("{} {}\n", check_emoji.0, "Successfully added!".green().bold());
}

pub fn remove_employee(employees: &mut HashMap<String, Vec<String>>, name: &str, dpt: &str) {
    if let Some(names) = employees.get_mut(dpt) {
        if names.iter().position(|n| n == name).is_some() {
            names.retain(|n| n != name);
            let check_emoji: Emoji = Emoji::new("✅", "v");
            println!("{}", format!("{} {} removed from {}!\n", check_emoji.0, name, dpt).green().bold());
        } else {
            let error_emoji: Emoji = Emoji::new("🚫", "e");
            println!("{} {} not found in {}.\n", error_emoji.0, name, dpt);
        }
    } else {
        let error_emoji: Emoji = Emoji::new("🚫", "e");
        println!("{} No employees found in department: {}\n", error_emoji.0, dpt);
    }
}

pub fn count_employees(employees: &HashMap<String, Vec<String>>, dpt: &str) -> String {
    let count = employees.get(dpt).map_or(0, |names| names.len());
    if count > 0 {
        format!("{} {} employees in department. '{}'.\n", count.to_string().green().bold(), dpt, dpt)
    } else {
        let error_emoji: Emoji = Emoji::new("🚫", "e");
        format!("{} No employees found in department: {}\n", error_emoji.0, dpt)
    }
}

pub fn list_all(employees: &HashMap<String, Vec<String>>) {
    if employees.is_empty() {
        let error_emoji: Emoji = Emoji::new("🚫", "e");
        println!("{} No employees found.", error_emoji.0);
    } else {
        for (dpt, names) in employees {
            println!("{}", format_department_header(dpt));
            if names.is_empty() {
                println!("No employees found.");
            } else {
                for name in names {
                    println!(" {}", name);
                }
            }
            println!();
        }
    }
}

pub fn list_department(employees: &HashMap<String, Vec<String>>, dpt: &str) {
    if let Some(names) = employees.get(dpt) {
        println!("{}", format_department_header(dpt));
        if names.is_empty() {
            println!("No employees found.");
        } else {
            for name in names {
                println!(" {}", name);
            }
        }
    } else {
        let error_emoji: Emoji = Emoji::new("🚫", "e");
        println!("{} No such department: {}", error_emoji.0, dpt);
    }
}

pub fn available_departments(employees: &HashMap<String, Vec<String>>) {
    if employees.is_empty() {
        let error_emoji: Emoji = Emoji::new("🚫", "e");
        println!("{} No departments found.\n", error_emoji.0.red().bold());
    } else {
        println!("Available Departments:");
        for dpt in employees.keys() {
            println!("- {}", dpt);
        }
    }
}

pub fn update_employee(employees: &mut HashMap<String, Vec<String>>, old_name: String, new_name: String, new_dpt: String) {
    let mut found = false;
    for (_dpt, names) in employees.iter_mut() {
        if let Some(pos) = names.iter().position(|n| n == &old_name) {
            names.remove(pos);
            found = true;

            employees.entry(new_dpt.clone()).or_insert_with(Vec::new).push(new_name.clone());
            let check_emoji: Emoji = Emoji::new("✅", "v");
            println!("{} {} updated to {} in {}.\n", check_emoji.0, old_name, new_name, new_dpt);
            break;
        }
    }

    if !found {
        let error_emoji: Emoji = Emoji::new("🚫", "e");
        println!("{} {} not found.\n", error_emoji.0, old_name);
    }
}

pub fn clear_department(employees: &mut HashMap<String, Vec<String>>, dpt: &str) {
    if employees.remove(dpt).is_some() {
        let check_emoji: Emoji = Emoji::new("✅", "v");
        println!("{} All employees removed from {}!\n", check_emoji.0, dpt);
    } else {
        let error_emoji: Emoji = Emoji::new("🚫", "e");
        println!("{} No such department: {}\n", error_emoji.0, dpt);
    }
}


fn format_department_header(dpt: &str) -> String {
    format!("----- {} -----", dpt.to_uppercase())
}
