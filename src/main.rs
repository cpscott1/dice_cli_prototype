use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Crest {
    Movement,
    Attack,
    Defense,
    Magic,
    Trap,
}

#[derive(Debug)]
enum DiceFace {
    Summon(u8),
    Crest(Crest),
}

#[derive(Debug)]
struct Monster {
    name: String,
    level: u8,
    attack: u8,
    defense: u8,
    hp: u8,
}

#[derive(Debug)]
struct Player {
    crests: HashMap<Crest, u32>,
    monsters: Vec<Monster>,
    life_points: i32,
}

fn main() {
    println!("Welcome to Dice Battle!");
}