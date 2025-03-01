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

impl Player {
    fn new() -> Self {
        Self { 
            crests: HashMap::new(),
            monsters: Vec::new(),
            life_points: 3,
        }
    }

    fn add_crest(&mut self, crest: Crest) {
        *self.crests.entry(crest).or_insert(0) += 1;
    }

    fn use_crest(&mut self, crest: Crest, amount: u32) -> bool {
        if let Some(count) = self.crests.get_mut(&crest) {
            if *count >= amount {
                *count -= amount;
                return true;
            }
        }
        false
    }
}

fn main() {
    println!("Welcome to Dice Battle!");
}

