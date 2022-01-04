use std::collections::HashMap;

#[allow(dead_code)]

/// Add an employee to a department
pub fn add_to_department(name: &str, department: &str, database: &mut HashMap<String, Vec<String>>) -> Result<bool, ()> {
    // add the name as first input for the department
    let names_vec = database.entry(String::from(department)).or_insert(vec![String::from(name)]);
    // if the department already has names in it, add the new name
    if !names_vec.contains(&String::from(name)) {
        names_vec.append(&mut vec![String::from(name)]);
    }
    Ok(true)
}

/// Remove an employee from a department
pub fn remove_from_department(name: &str, department: &str, database: &mut HashMap<String, Vec<String>>) -> Result<bool, ()> {
    if let Some(department_names) = database.get_mut(department) {
        department_names.retain(|n| !n.eq_ignore_ascii_case(name));
        Ok(true)
    } else {
        println!("Error: Department not found");
        Err(())
    }
}

/// Find an employee in department
pub fn search_employee(name: &str, department: &str, database: &HashMap<String, Vec<String>>) -> Result<bool, ()> {
    if database.contains_key(department) {
        if let Some(employees) = database.get(department) {
            for empl in employees.iter() {
                if empl.eq_ignore_ascii_case(name) {
                    return Ok(true);
                }
            };
            Ok(false)
        } else {
            Ok(false)
        }
    } else {
        println!("Department not found");
        Err(())
    }
}

/// Show a list of the employees in a department
pub fn get_employees(department: &str, database: &HashMap<String, Vec<String>>) -> Result<Vec<String>, ()> {
    if database.contains_key(department) {
        if let Some(employees) = database.get(department) {
            Ok(employees.clone())
        } else {
            Ok(Vec::new())
        }
    } else {
        println!("Department not found");
        Err(())
    }
}

/// Show a list of the departments
pub fn get_departments(database: &HashMap<String, Vec<String>>) -> Result<Vec<String>, ()> {
    Ok(database.clone().into_keys().collect())
}