use std::collections::HashMap;

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

pub fn animal_move(animal: &mut Animal) {

    let dir = animal.dir;

    animal.x = if dir >= 2 && dir < 5  { animal.x + 1 }
          else if dir == 1 || dir == 5 { animal.x }
          else { animal.x - 1};

    animal.y = if dir >= 0 && dir < 3 { animal.y - 1 }
          else if dir >= 4 && dir < 7 { animal.y + 1 }
          else { animal.y };

    animal.energy -= 1;
}

pub fun animal_turn(animal: &mut Animal) {
    // they need to turn because right now they all move in the same direction!
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

    let new_animal = Animal::new(animal.x,
                                 animal.y,
                                 animal.energy,
                                 animal.dir,
                                 animal.genes, //add a function that returns a [i32] to modify this
                                 animal.alive);
    new_animal
}


pub fn add_animal(animal: Animal, animals: &mut Vec<Animal>) {

    animals.push(animal);
}

pub fn is_alive(animal: &mut Animal) {

    if animal.energy < 1 { animal.alive = false };
}
