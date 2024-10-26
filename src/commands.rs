use crate::utils::capitalize_word;

pub enum Opt {
    Add { name: String, dpt: String },
    Remove { name: String, dpt: String },
    Count { dpt: String },
    ListDepartments,
    Update { old_name: String, new_name: String, new_dpt: String },
    Department(String),
    All,
    Clear { dpt: String },
    Help,
    Quit,
    ExportToFile(String),
}

impl Opt {
    pub fn from_input(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        
        match parts.as_slice() {
            ["Add", name, "to", dpt] => parse_add_remove(name, dpt, |name, dpt| Opt::Add { name, dpt }),
            ["Remove", name, "from", dpt] => parse_add_remove(name, dpt, |name, dpt| Opt::Remove { name, dpt }),
            ["Count", dpt] => Ok(Opt::Count { dpt: capitalize_word(dpt) }),
            ["ListDepartments"] => Ok(Opt::ListDepartments),
            ["Update", old_name, "to", new_name, "in", new_dpt] => {
                if old_name.is_empty() || new_name.is_empty() || new_dpt.is_empty() {
                    return Err(format_error("Old name, new name, and new department cannot be empty."));
                }
                Ok(Opt::Update {
                    old_name: capitalize_word(old_name),
                    new_name: capitalize_word(new_name),
                    new_dpt: capitalize_word(new_dpt),
                })
            }
            ["Quit"] => Ok(Opt::Quit),
            ["All"] => Ok(Opt::All),
            ["Clear", dpt] => {
                if dpt.is_empty() {
                    return Err(format_error("Department cannot be empty"));
                }
                Ok(Opt::Clear { dpt: capitalize_word(dpt) })
            }
            ["Help"] | ["?"] => Ok(Opt::Help),
            [dpt] => Ok(Opt::Department(dpt.to_string())),
            ["Export", file_path] => Ok(Opt::ExportToFile(file_path.to_string())),
            _ => Err(format_error("Invalid command format.")),
        }
    }
}

fn parse_add_remove<F>(name: &str, dpt: &str, constructor: F) -> Result<Opt, String>
where
    F: Fn(String, String) -> Opt,
{
    if name.is_empty() || dpt.is_empty() {
        return Err(format_error("Name and department cannot be empty."));
    }
    Ok(constructor(
        capitalize_word(name),
        capitalize_word(dpt),
    ))
}

fn format_error(msg: &str) -> String {
    format!("[ERROR]: {}", msg)
}
