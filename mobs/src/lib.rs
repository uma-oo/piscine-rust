

pub use mobs::boss::{Boss};
pub use mobs::member::{Member};

use std::collections::{ HashMap, HashSet };

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&self, members_info: (&str, u32)) {}

    pub fn attack(&self, another_mob: Mob) {}

    pub fn steal(&self, another_mob: Mob, value_to_steal: u64) {}

    pub fn conquer_city(&self, slice_of_mobs: &[Mob], city_name: String) {}
}
