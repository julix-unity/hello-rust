use std::collections::HashMap;
use std::io;
use dialoguer::Select;

fn prompt_and_read(input_prompt: &str) -> String {
    println!("{}", input_prompt);
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    input_string.trim().to_string()
}

fn add_employee_to_department(employees_by_department_map: &mut HashMap<String, Vec<String>>) {
    let employee_name = prompt_and_read("What's the name of the employee you want to add?");
    let department_prompt = format!("Great! And what's the name of the department you want to add {} to?", employee_name);
    let department_name = prompt_and_read(&department_prompt);

    employees_by_department_map.entry(department_name.clone())
        .and_modify(|e| {
            match e.binary_search(&employee_name) {
                Ok(_) => println!("{} is already in the {} department.", employee_name, department_name),
                Err(pos) => e.insert(pos, employee_name.clone()),
            }
        })
        .or_insert_with(|| vec![employee_name.clone()]);
    
    println!("{} has been added to the {} department.", employee_name, department_name);

    if let Some(employees) = employees_by_department_map.get(&department_name) {
        println!("{} is now in the list: {:?}", &employee_name, employees);
    }
}

/*
* Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
* For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
* 
* Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/


pub fn department_employees() {

    // Setup
    let mut employees_by_department_map = HashMap::from([
        (String::from("Engineering"), vec![
            String::from("Emily"), 
            String::from("Frank"),
            String::from("Sarah"),
        ]),
        (String::from("Marketing"), vec![
            String::from("Boo"),
            String::from("Marleen"),
        ]),
    ]);


    let options: Vec<&str> = vec!["1) Display employees of a particular department", "2) Display all employees", "3) Add an employee to a department", /*"4) Remove an employee"*/];

    let selection = Select::new()
        .with_prompt("What do you choose?")
        .items(&options)
        .interact()
        .unwrap() + 1;

    let choose_department = || -> String {
        let departments: Vec<&String> = Vec::from_iter(employees_by_department_map.keys());
        let selection = Select::new()
        .with_prompt("Display which department?")
        .items(&departments)
        .interact()
        .unwrap();
        departments[selection].clone()
    };


    // println!("{:?}", Vec::from_iter(employees_by_department_map.keys()));
    
    match selection {
        1 => {
            let department = choose_department();
            if let Some(department_employees) = employees_by_department_map.get(&department) {
                println!("The employees in {} are {:?}", department, department_employees);
            } else {
                println!("The department '{}' does not exist.", department);
            }
        },
        2 => println!("{:?}", employees_by_department_map),
        3 => add_employee_to_department(&mut employees_by_department_map),
        _ => {}
    }


    
}
