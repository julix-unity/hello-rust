/*
* Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
* For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
* 
* Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/

use std::collections::HashMap;
use std::{cmp::Ordering, io};
use dialoguer::Select;

// TODO look into this:
// template< typename T >
// typename std::vector<T>::iterator 
//    insert_sorted( std::vector<T> & vec, T const& item )
// {
//     return vec.insert
//         ( 
//             std::upper_bound( vec.begin(), vec.end(), item ),
//             item 
//         );
// }

pub fn department_employees() {

    // Setup
    let mut employees_by_department_map = HashMap::from([
        ("Engineering", vec!("Emily", "Frank", "Sarah")),
        ("Marketing", vec!("Boo", "Marleen")),
    ]);


    let options: Vec<&str> = vec!["1) Display employees of a particualar department", "2) Display all employees", "3) Add an employee to a department", /*"4) Remove an employee"*/];

    let selection = Select::new()
        .with_prompt("What do you choose?")
        .items(&options)
        .interact()
        .unwrap() + 1;

    let choose_department = || {
        let departments = Vec::from_iter(employees_by_department_map.keys());
        let selection = Select::new()
        .with_prompt("Diplay which department?")
        .items(&departments)
        .interact()
        .unwrap();
        departments[selection]
    };

    // println!("{:?}", Vec::from_iter(employees_by_department_map.keys()));
    
    match selection {
        1 => {
            let department = choose_department();
            let department_employees = &employees_by_department_map[department];
            println!("The employees in {} are {:?}", department, department_employees);
        },
        2 => println!("{:?}", employees_by_department_map),
        3 => {
            println!("What's the name of the employee you want to add?");
            
            let mut employee_name_raw = String::new();
            let mut department_name_raw = String::new();
            io::stdin()
                .read_line(&mut employee_name_raw)
                .expect("Failed to read employee name");
            
            println!("Great! And what's the name of the department you want to add {} to?", employee_name_raw);
            
            io::stdin()
                .read_line(&mut department_name_raw)
                .expect("Failed to read department name");

            let employee_name = employee_name_raw.trim();
            let department_name = department_name_raw.trim();


            let mut deparment_list = employees_by_department_map.keys();
            let deparment_exists = deparment_list.find(|&&x| x == department_name);

            match deparment_exists {
                Some(_x) => {
                    let mut employees = employees_by_department_map[department_name].clone();
                    match employees.binary_search(&employee_name){
                        Ok(_pos) => {}, // element already in vector @ `pos` 
                        Err(pos) => {
                            employees.insert(pos, employee_name);
                            employees_by_department_map.insert(&department_name, employees);
                        }
                    }
                }
                None => {
                    employees_by_department_map.insert(&department_name, vec!(&employee_name));
                }
            }

            println!("Perfect, adding {} to {}!\n{} is now in the list: {:?}", employee_name, department_name, employee_name, employees_by_department_map[department_name]);
        }
        _ => {}
    }


    
}
