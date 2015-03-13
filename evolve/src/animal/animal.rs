use std::rand;
use std::rand::Rng;
use std::collections::HashMap;
use std::rand::distributions::{IndependentSample, Range};


pub struct Animal {
    pub x:      i32,
    pub y:      i32,
    pub energy: i32,
    pub dir:    i32,
    pub genes: [i32; 8],
    pub alive: bool,
}

impl Animal {

    pub fn new(x: i32, y: i32, energy: i32, dir: i32, genes: [i32; 8], alive: bool) -> Animal {
        Animal { x: x, y: y, energy: energy, dir: dir, genes: genes, alive: alive }
    }

    pub fn show(&self) {
        println!("x: {}, y: {}\nenergy: {}\ndir: {}\nalive: {}",
                 self.x, self.y, self.energy, self.dir, self.alive)
    }
}

pub fn animal_move(animal: &mut Animal, width: i32, height: i32) {

    let dir = animal.dir;
    
    animal.x = ( animal.x + (if dir >= 2 && dir < 5  { 1 }
           else if dir == 1 || dir == 5 { 0 }
           else { -1 }) + width) % width;
           
    animal.y = (animal.y + (if dir >= 0 && dir < 3 { -1 }
           else if dir >= 4 && dir < 7 { 1  }
           else { 0 }) + height) % height;

    animal.energy -= 1;
}

pub fn animal_turn(animal: &mut Animal) {
    
    // animal.dir = SignedInt::abs(rand::random::<i32>() % 8);
    
    if animal.genes[((animal.x + animal.y) % 8) as usize] > 5 {
        animal.dir += 1;
    } else {
        animal.dir -= 1;
    }
}

pub fn animal_eat(animal: &mut Animal, plants: &mut HashMap<(i32, i32), bool>, plant_energy: i32) {

    let pos = (animal.x, animal.y);

    if plants.contains_key(&pos) {
        animal.energy += plant_energy;
        plants.remove(&pos);
    };
}

pub fn animal_reproduce(animal: &mut Animal, rep_energy: i32) -> bool {

    if animal.energy >= rep_energy {
        animal.energy /= 2;
        return true
    }
    false
}

pub fn copy_animal(animal: &mut Animal) -> Animal {

    let mut genes = animal.genes;
    
    genes[((animal.x + animal.y) % 8) as usize] += 8; 

    let new_animal = Animal::new(animal.x + 1,
                                 animal.y - 1,
                                 animal.energy,
                                 animal.dir,
                                 genes, //add a function that returns a [i32] to modify this
                                 animal.alive);
    new_animal
}


pub fn add_animal(animal: Animal, animals: &mut Vec<Animal>) {

    animals.push(animal);
}

pub fn is_alive(animal: &mut Animal) {

    if animal.energy < 1 { animal.alive = false };
}

pub fn remove_dead(animals: &mut Vec<Animal>) {
    
    let range = 0..animals.len();
    let mut hold_animal: Animal;
    
    for i in range {
        if !animals[i].alive { 
            hold_animal = animals.remove(i as usize);
            animals.push(hold_animal);
        };
    }
    
    let range_two = 0..animals.len();
    
    for i in range_two {
        if i != (animals.len() - 1) {
            if !animals[i].alive {
                animals.pop();
            }
        }
    }
}
        
