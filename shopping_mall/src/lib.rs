pub mod mall;
pub use guard::Guard;
pub use mall::floor::store::employee::Employee;
pub use mall::floor::store::*;
pub use mall::floor::*;
pub use mall::*;

/// Returns the store with the largest square meters in the mall
pub fn biggest_store(mall: Mall) -> Store {
    let mut biggest: Option<&Store> = None;
    let mut max_size = 0;

    // Iterate through all floors and their stores to find the biggest one
    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            if store.square_meters > max_size {
                max_size = store.square_meters;
                biggest = Some(store);
            }
        }
    }

    // Unwrap and clone the biggest store
    // This should never panic if there's at least one store in the mall
    biggest.unwrap().clone()
}

/// Returns a vector of employees with the highest salary
pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let mut highest_salary = 0.0;
    let mut highest_paid: Vec<Employee> = Vec::new();

    // Find the highest salary across all stores and employees
    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            for employee in store.employees.iter() {
                if employee.salary > highest_salary {
                    highest_salary = employee.salary;
                    highest_paid.clear();
                    highest_paid.push(employee.clone());
                } else if employee.salary == highest_salary {
                    highest_paid.push(employee.clone());
                }
            }
        }
    }

    highest_paid
}

/// Returns the total number of employees and guards in the mall
pub fn nbr_of_employees(mall: Mall) -> usize {
    let mut count = 0;

    // Count guards
    count += mall.guards.len();

    // Count all employees across all stores
    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            count += store.employees.len();
        }
    }

    count
}

/// Adds guards if there aren't enough (1 guard per 200 sq meters)
pub fn check_for_securities(mall: &mut Mall, guards: Vec<Guard>) {
    // Calculate total floor size
    let mut total_size = 0;
    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            total_size += store.square_meters;
        }
    }

    // Calculate required guards (1 per 200 sq meters)
    let required_guards = (total_size + 199) / 200; // Ceiling division
    let current_guards = mall.guards.len() as u64;

    // Add guards if needed
    if current_guards < required_guards {
        let guards_to_add = required_guards - current_guards;
        
        // Add guards from the provided vector
        let mut guard_index = 0;
        for _ in 0..guards_to_add {
            if guard_index < guards.len() {
                mall.hire_guard(guards[guard_index].clone());
                guard_index += 1;
            } else {
                break; // No more guards to add
            }
        }
    }
}

/// Adjusts employee salaries based on working hours
pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.iter_mut() {
        for store in floor.stores.iter_mut() {
            for employee in store.employees.iter_mut() {
                // Calculate working hours
                let hours = employee.working_hours.1 - employee.working_hours.0;
                
                if hours > 10 {
                    // Raise by 10%
                    employee.salary *= 1.1;
                } else {
                    // Cut by 10%
                    employee.salary *= 0.9;
                }
            }
        }
    }
}