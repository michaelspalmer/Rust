use std::num::SignedInt;
use std::collections::HashMap;

pub struct Animal {
    pub x:      i32,
    pub y:      i32,
    pub energy: i32,
    pub dir:    i32,
    pub genes:  Vec<i32>,
    pub alive:  bool,
}

impl Animal {

    pub fn new(x: i32, y: i32, e: i32, d: i32, g: Vec<i32>, a: bool) -> Animal {
        Animal { x: x, y: y, energy: e, dir: d, genes: g, alive: a }
    }

    pub fn show(&self) {
        println!("x: {}, y: {}\nenergy: {}\ndir: {}\nalive: {}\ngenes: {:?}",
                 self.x, self.y, self.energy, self.dir, self.alive, self.genes)
    }

    pub fn ani_move(&mut self, width: i32, height: i32) {

        self.x = ( self.x + (if self.dir >= 2 && self.dir < 5 { 1 }
                else if self.dir == 1 || self.dir == 5 { 0 }
                else { -1 }) + width) % width;

        self.y = (self.y + (if self.dir >= 0 && self.dir < 3 { -1 }
                else if self.dir >= 4 || self.dir < 7 { 1 }
                else { 0 }) + height) % height;

        self.energy -= 1;
    }

    pub fn turn(&mut self) {

        // this needs work - add in something similar to what he has.
        if self.genes[((self.x + self.y) % 8) as usize] > 5 {
            self.dir += 1;
        } else {
            self.dir -= 1;
        }

        self.dir = SignedInt::abs(self.dir % 8);
    }
    
    pub fn eat(&mut self, plants: &mut HashMap<(i32, i32), bool>, plant_energy: i32) {

        let pos = (self.x, self.y);

        if plants.contains_key(&pos) {
            self.energy += plant_energy;
            plants.remove(&pos);
        }
    }

    pub fn reproduce(&mut self, rep_energy: i32) -> bool {

        if self.energy >= rep_energy {
            self.energy /= 2;
            return true
        }
        false
    }

    pub fn is_alive(&mut self) {

        if self.energy < 1 { self.alive = false };
    }
}
