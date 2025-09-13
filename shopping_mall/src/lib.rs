mod mall;

pub use mall::{ Mall, Store, Employee, Guard, Floor };
use std::collections::HashMap;
//biggest_store: receives a Mall and returns the Store with the most square_meters.
pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut max_store: Option<Store> = None;
    let mut name: String = String::new();
    for (_, floor) in mall.floors.clone() {
        for (key, store) in floor.stores {
            if max_store.is_none() {
                max_store = Some(store);
                name = key;
                continue;
            }
            if store.square_meters > max_store.clone().unwrap().square_meters {
                name = key;
                max_store = Some(store);
            }
        }
    }
    (name, max_store.unwrap())
}

//highest_paid_employee: receives a Mall and returns a vector containing the Employee(s) with the highest salary.
pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let mut result: Vec<(&String, &Employee)> = Vec::new();
    let mut max_salary = 0.0;
    for (_, floor) in mall.floors.clone() {
        for (_, store) in floor.stores {
            for (_, employee) in store.employees {
                if max_salary < employee.salary {
                    max_salary = employee.salary;
                }
            }
        }
    }

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, employee) in &store.employees {
                if max_salary == employee.salary {
                    result.push((name, employee));
                }
            }
        }
    }

    result
}

//nbr_of_employees: receives a Mall and returns the number of employees and guards as a usize
pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut number_staff = mall.guards.len();
    for (_, floor) in mall.floors.clone() {
        for (_, store) in floor.stores {
            number_staff += store.employees.len();
        }
    }
    number_staff
}

// If there is not at least 1 guard for every 200 square meters of total floor size,
//  a guard should be added to the Mall.guards.
pub fn check_for_securities(mall: &mut Mall, mut guards: HashMap<String, Guard>) {
    let mut how_guards_should_be = 0.0;
    for (_, floor) in mall.floors.clone() {
        how_guards_should_be += floor.size_limit as f64;
    }

    how_guards_should_be = how_guards_should_be / 200.0;
    if (how_guards_should_be.ceil() as usize) > mall.guards.len() {
        for mut _i in 0..(how_guards_should_be.ceil() as usize) - mall.guards.len() {
            for (key, _) in &guards {
                let (new_guard_name, new_guard) = guards.remove_entry(&key.clone()).unwrap();
                mall.guards.insert(new_guard_name, new_guard);
                break;
            }
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                if employee.working_hours.1 - employee.working_hours.0 < 10 {
                    employee.salary = employee.salary - 0.1 * employee.salary;
                } else {
                    employee.salary = employee.salary + 0.1 * employee.salary;
                }
            }
        }
    }
}
