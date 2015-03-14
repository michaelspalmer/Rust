use std::rand;
use std::rand::Rng;
use std::num::SignedInt;
use std::collections::HashMap;
use std::rand::distributions::{IndependentSample, Range};


pub struct Animal {
    pub x:      i32,
    pub y:      i32,
    pub energy: i32,
    pub dir:    i32,
    pub genes: Vec<i32>,
    pub alive: bool,
}

impl Animal {

    pub fn new(x: i32, y: i32, energy: i32, dir: i32, genes: Vec<i32>, alive: bool) -> Animal {
        Animal { x: x, y: y, energy: energy, dir: dir, genes: genes, alive: alive }
    }

    pub fn show(&self) {
        println!("x: {}, y: {}\nenergy: {}\ndir: {}\nalive: {}\ngenes: {:?}",
                 self.x, self.y, self.energy, self.dir, self.alive, self.genes)
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

    // this needs work - add in something similar to what he has.
    if animal.genes[((animal.x + animal.y) % 8) as usize] > 5 {
        animal.dir += 1;
    } else {
        animal.dir -= 1;
    }
    
    animal.dir = SignedInt::abs(animal.dir % 8);
}

pub fn animal_eat(animal: &mut Animal, plants: &mut HashMap<(i32, i32), bool>, plant_energy: i32) {

    let pos = (animal.x, animal.y);

    if plants.contains_key(&pos) {
        animal.energy += plant_energy;
        plants.remove(&pos);
    } else {
        animal.energy -= 1;
    }
}

pub fn animal_reproduce(animal: &mut Animal, rep_energy: i32) -> bool {

    if animal.energy >= rep_energy {
        animal.energy /= 2;
        return true
    }
    false
}

pub fn copy_animal(animal: &mut Animal) -> Animal {

    let mut new_animal = Animal::new(animal.x + 1,
                                 animal.y - 1,
                                 animal.energy,
                                 (animal.dir + 1) % 8,
                                 animal.genes.clone(),
                                 animal.alive);
    mut_gene(&mut new_animal);
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
    let mut count: i32 = 0;
    let mut hold: Animal;
    let length = animals.len();
    
    for i in range {
        if !animals[i].alive { 
           hold = animals.remove(i as usize);
           animals.push(hold);
           count += 1;
        }
    }
    animals.truncate(length - count as usize);
}

pub fn gen_genes() -> Vec<i32> {

    let mut genes: Vec<i32> = vec![];

    let between = Range::new(0, 10);
    let mut rng = rand::thread_rng();
    let range   = 0..8;
    
    for _ in range {
        genes.push(between.ind_sample(&mut rng));
    }   
    genes
}

pub fn mut_gene(animal: &mut Animal) {
    
    let mutation_val = Range::new(0, 3);
    let index = Range::new(0, animal.genes.len());
    let mut rng = rand::thread_rng();
    
    let mut mutation: i32 = mutation_val.ind_sample(&mut rng);
    let index = index.ind_sample(&mut rng);
    
    animal.genes[index] += mutation;
}