use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub movement: i32,
    pub toughness: i32,
    pub save: i32,
    pub invulnerable: i32,
    pub wounds: i32,
    pub leadership: i32,
    pub objective_control: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weapons {
    pub name: String,
    pub range: i32,
    pub attacks: i32,
    pub attack_dice: Option<String>,
    pub hit: i32,
    pub strength: i32,
    pub armour_pen: i32,
    pub damage: i32,
    pub damage_dice: Option<String>,
    pub tags: Option<Vec<String>>,
    pub ranged: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeaponSet {
    pub weapons: Vec<Weapons>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
    pub points: Vec<i32>,
    pub stats: Stats,
    pub weapons: Option<Vec<Weapons>>,
    pub abilities: HashMap<String, serde_json::Value>,
    pub tags: Option<Vec<String>>,
    pub models: HashMap<String, Vec<i32>>,
    pub equipment: Option<Vec<String>>,
}
