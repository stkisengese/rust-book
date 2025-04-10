pub mod boss;
pub mod member;

use boss::Boss;
use member::{Member, Role};

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members.push(Member::new(name, Role::Associate, age));
    }

    pub fn attack(&mut self, other: &mut Mob) {
        // Calculate power combat score for both mobs
        let self_score = self.members.iter().map(|m| match m.role {
            Role::Underboss => 4,
            Role::Caporegime => 3,
            Role::Soldier => 2,
            Role::Associate => 1,
        }).sum::<u8>();

        let other_score = other.members.iter().map(|m| match m.role {
            Role::Underboss => 4,
            Role::Caporegime => 3,
            Role::Soldier => 2,
            Role::Associate => 1,
        }).sum::<u8>();

        // Determine which mob loses a member
        if self_score > other_score {
            if !other.members.is_empty() {
                other.members.pop();
            }
            
            // Check if other mob has no more members
            if other.members.is_empty() {
                // Add cities and wealth from other mob
                self.cities.extend(other.cities.clone());
                self.wealth += other.wealth;
                other.wealth = 0;
                other.cities.clear();
            }
        } else {
            // Self loses (including draw)
            if !self.members.is_empty() {
                self.members.pop();
            }
            
            // Check if self mob has no more members
            if self.members.is_empty() {
                // Add cities and wealth from self mob
                other.cities.extend(self.cities.clone());
                other.wealth += self.wealth;
                self.wealth = 0;
                self.cities.clear();
            }
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u32) {
        let amount_to_steal = amount.min(target.wealth);
        target.wealth -= amount_to_steal;
        self.wealth += amount_to_steal;
    }

    pub fn conquer_city(&mut self, mobs: &Vec<Mob>, city_name: &str, value: u8) {
        // Check if any other mob has this city
        let city_exists = mobs.iter().any(|mob| {
            mob.cities.iter().any(|(name, _)| name == city_name)
        });

        if !city_exists {
            self.cities.push((city_name.to_string(), value));
        }
    }
}