use snafu::Snafu;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum OrbitObject {
    CenterOfMass,
    Object(String),
}

impl From<&str> for OrbitObject {
    fn from(s: &str) -> OrbitObject {
        match s {
            "COM" => OrbitObject::CenterOfMass,
            _ => OrbitObject::Object(s.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct OrbitsMap {
    orbits: HashMap<OrbitObject, OrbitObject>,
    orbits_count: RefCell<HashMap<OrbitObject, usize>>,
}

#[derive(Debug, Snafu)]
pub enum ParseOrbitsMapError {
    #[snafu(display("Invalid line in orbits map: {}", line))]
    InvalidSyntax { line: String },
    #[snafu(display("Object has unresolved orbits: {:?}", unresolved_orbits))]
    UnresolvedOrbit {
        unresolved_orbits: HashSet<OrbitObject>,
    },
}

impl FromStr for OrbitsMap {
    type Err = ParseOrbitsMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut orbits: HashMap<OrbitObject, OrbitObject> = HashMap::new();
        let mut unresolved_orbits: HashSet<OrbitObject> = HashSet::new();
        for line in s.lines() {
            // Split the line : OBJECT)OBJECT
            let objects = line.trim().split(")").collect::<Vec<_>>();
            // Verify that there is two objects
            if objects.len() != 2 {
                return Err(ParseOrbitsMapError::InvalidSyntax {
                    line: line.to_string(),
                });
            }
            // get the two ojects
            let center = OrbitObject::from(objects[0]);
            let satellite = OrbitObject::from(objects[1]);
            // If the satellite was on an unresolved orbit, resolve it
            unresolved_orbits.remove(&satellite);
            // If the center has no known orbits
            // add it to the list of unresolved orbits
            if center != OrbitObject::CenterOfMass && !orbits.contains_key(&center) {
                unresolved_orbits.insert(center.clone());
            }
            // Add the orbit
            orbits.insert(satellite, center);
        }
        if !unresolved_orbits.is_empty() {
            Err(ParseOrbitsMapError::UnresolvedOrbit { unresolved_orbits })
        } else {
            Ok(OrbitsMap {
                orbits,
                orbits_count: RefCell::new(HashMap::new()),
            })
        }
    }
}

impl OrbitsMap {
    fn orbit_center(&self, object: &OrbitObject) -> Option<&OrbitObject> {
        self.orbits.get(object)
    }

    fn count_orbits_of(&self, object: &OrbitObject) -> usize {
        if *object == OrbitObject::CenterOfMass {
            0
        } else {
            let count_option = self.orbits_count.borrow().get(object).map(|o| o.clone());
            if let Some(count) = count_option {
                count
            } else {
                let center = self.orbit_center(object).unwrap();
                let orbits_count = self.count_orbits_of(center) + 1;
                self.orbits_count
                    .borrow_mut()
                    .insert(object.clone(), orbits_count);
                orbits_count
            }
        }
    }

    pub fn orbit_count_checksum(&self) -> usize {
        self.orbits.keys().map(|o| self.count_orbits_of(o)).sum()
    }

    pub fn minimal_orbital_transfers(
        &self,
        obj1: &OrbitObject,
        obj2: &OrbitObject,
    ) -> Option<usize> {
        let mut obj1_orbits = Vec::new();

        let mut pos = obj1;
        loop {
            let center = self.orbit_center(pos)?;
            obj1_orbits.push(center);
            pos = center;
            if *center == OrbitObject::CenterOfMass {
                break;
            }
        }

        let mut pos = obj2;
        let mut distance2 = 0;
        loop {
            let center = self.orbit_center(pos)?;
            if let Some(distance1) = obj1_orbits.iter().position(|&o| o == center) {
                return Some(distance1 + distance2);
            }
            pos = center;
            distance2 += 1;
            if *center == OrbitObject::CenterOfMass {
                break;
            }
        }
        None
    }
}
