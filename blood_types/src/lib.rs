#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(format!("Invalid antigen: {}", s)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(format!("Invalid Rh factor: {}", s)),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            ord => ord,
        }
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(format!("Invalid blood type: {}", s));
        }

        let (antigen_str, rh_str) = s.split_at(s.len() - 1);
        let antigen = Antigen::from_str(antigen_str)?;
        let rh_factor = RhFactor::from_str(rh_str)?;

        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug, Formatter};

impl Debug for BloodType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match self.antigen {
                Antigen::A => "A",
                Antigen::AB => "AB",
                Antigen::B => "B",
                Antigen::O => "O",
            },
            match self.rh_factor {
                RhFactor::Positive => "+",
                RhFactor::Negative => "-",
            }
        )
    }
}

impl BloodType {

	pub fn can_receive_from(&self, other: &Self) -> bool {
        // Check antigen compatibility
        let antigen_compatible = match (&self.antigen, &other.antigen) {
            (_, Antigen::O) => true,                      // Anyone can receive from O
            (Antigen::AB, _) => true,                     // AB can receive from anyone
            (Antigen::A, Antigen::A) => true,             // A can receive from A
            (Antigen::B, Antigen::B) => true,             // B can receive from B
            _ => false,                                   // Other combinations not compatible
        };

        // Check Rh factor compatibility
        let rh_compatible = match (&self.rh_factor, &other.rh_factor) {
            (_, RhFactor::Negative) => true,              // Anyone can receive from negative
            (RhFactor::Positive, RhFactor::Positive) => true, // Positive can receive from positive
            _ => false,                                   // Negative cannot receive from positive
        };

        antigen_compatible && rh_compatible
	}

    fn all_types() -> Vec<BloodType> {
        use Antigen::*;
        use RhFactor::*;

        vec![
            BloodType { antigen: A, rh_factor: Positive },
            BloodType { antigen: A, rh_factor: Negative },
            BloodType { antigen: B, rh_factor: Positive },
            BloodType { antigen: B, rh_factor: Negative },
            BloodType { antigen: AB, rh_factor: Positive },
            BloodType { antigen: AB, rh_factor: Negative },
            BloodType { antigen: O, rh_factor: Positive },
            BloodType { antigen: O, rh_factor: Negative },
        ]
    }

    fn filter_blood_types<F>(&self, predicate: F) -> Vec<BloodType>
    where
        F: Fn(&BloodType) -> bool,
    {
        Self::all_types()
            .into_iter()
            .filter(predicate)
            .collect()
    }

    pub fn donors(&self) -> Vec<BloodType> {
        self.filter_blood_types(|bt| self.can_receive_from(bt))
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        self.filter_blood_types(|bt| bt.can_receive_from(self))
    }

	// pub fn donors(&self) -> Vec<Self> {
	// }

	// pub fn recipients(&self) -> Vec<BloodType> {
    // }
}