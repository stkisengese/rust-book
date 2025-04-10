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

/// Ensures there is at least 1 guard for every 200 square meters of floor size
/// If not, adds guards from the provided vector to meet the requirement
pub fn check_for_securities(mall: &mut Mall, guards: Vec<Guard>) {
    let mut g = guards.clone();
    let mut count = 0_usize;
    let s = mall
        .floors
        .iter()
        .flat_map(|floor| &floor.stores)
        .fold(0_u64, |acc, x| acc + x.square_meters);
    while count * 200 < s as usize && g.len() > 0 {
        count += 1;
        mall.guards.push(g[0].clone());
        g.remove(0);
    }
}


/// Adjusts employee salaries: +10% if they work more than 10 hours, -10% otherwise
pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.iter_mut() {
        for store in floor.stores.iter_mut() {
            for employee in store.employees.iter_mut() {
                let w_h = employee.working_hours.1 - employee.working_hours.0;
                let percentage = employee.salary * 0.1;
                if w_h > 10 {
                    employee.salary += percentage
                } else {
                    employee.salary -= percentage
                }
            }
        }
    }
}