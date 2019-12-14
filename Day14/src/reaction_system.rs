use crate::reaction::{Chemical, Reaction};
use std::collections::HashMap;

pub struct ReactionSystem {
    reactions: HashMap<Chemical, Reaction>,
}

impl ReactionSystem {
    pub fn new(reactions: Vec<Reaction>) -> ReactionSystem {
        let mut m = HashMap::new();
        for reaction in reactions {
            m.insert(reaction.output_chemical().clone(), reaction);
        }
        ReactionSystem { reactions: m }
    }

    pub fn get_ore_for_fuel(
        &self,
        fuel: usize,
        mut excess_list: &mut HashMap<Chemical, usize>,
    ) -> Option<usize> {
        let mut ore: usize = 0;
        let mut reactives_to_resolve: HashMap<Chemical, usize> = HashMap::new();
        reactives_to_resolve.insert(Chemical::Fuel, fuel);
        while let Some((reactive, mut quantity)) = reactives_to_resolve
            .iter()
            .map(|(r, q)| (r.clone(), *q))
            .next()
        {
            let excess = *excess_list.get(&reactive).unwrap_or(&0);
            if quantity < excess {
                *excess_list.get_mut(&reactive).unwrap() -= quantity;
                quantity = 0;
            } else {
                excess_list.remove(&reactive);
                quantity -= excess;
            }
            let reaction = self.reactions.get(&reactive)?;
            let output_quantity = reaction.output().coefficient();
            let needed_reactions = (quantity as f64 / output_quantity as f64).ceil() as usize;
            for input in reaction.inputs() {
                let quantity_needed = input.coefficient() as usize * needed_reactions;
                match input.chemical() {
                    Chemical::Ore => ore += quantity_needed,
                    Chemical::Intermediary(_) => {
                        let entry = reactives_to_resolve
                            .entry(input.chemical().clone())
                            .or_insert(0);
                        *entry += quantity_needed;
                    }
                    Chemical::Fuel => return None,
                }
            }
            let total_output = output_quantity as usize * needed_reactions;
            let excess = total_output - quantity;
            let excess_entry = excess_list.entry(reactive.clone()).or_insert(0);
            *excess_entry += excess;
            reactives_to_resolve.remove(&reactive);
        }
        Some(ore)
    }

    pub fn get_minimum_ore_for_one_fuel(&self) -> Option<usize> {
        self.get_ore_for_fuel(1, &mut HashMap::new())
    }

    pub fn get_maximum_fuel_synthetizable(&self, mut ore: usize) -> Option<usize> {
        let mut excess_list: HashMap<Chemical, usize> = HashMap::new();
        let mut fuel = 0;
        let mut fuel_increment = 2usize.pow(16);
        while fuel_increment != 0 {
            let mut new_excess_list = excess_list.clone();
            let ore_needed = self.get_ore_for_fuel(fuel_increment, &mut new_excess_list)?;
            if ore_needed < ore {
                fuel += fuel_increment;
                ore -= ore_needed;
                excess_list = new_excess_list;
            } else {
                fuel_increment >>= 1;
            }
        }
        Some(fuel)
    }
}
