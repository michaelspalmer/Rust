#![warn(deprecated)]
#![feature(rand)]

use std::collections::HashMap;
use std::rand;
use std::rand::distributions::{IndependentSample, Range};

struct Animal {
    x:      i32,
    y:      i32,
    energy: i32,
    dir:    i32,
    genes: [i32; 8],
}

impl Animal {

    fn new(x: i32, y: i32, energy: i32, dir: i32, genes: [i32; 8],) -> Animal {
        Animal { x: x, y: y, energy: energy, dir: dir, genes: genes }
    }

    fn show(&self) {
        println!("x: {}, y: {}\nenergy: {}\ndir: {}",
                 self.x, self.y, self.energy, self.dir)
    }
}

fn main() {

    let width: i32 = 100;
    let height: i32 = 30;
    let jungle: [i32; 4] = [45, 10, 10, 10];
    let plant_energy = 80;
    let reproduction_energy = 200;

    let mut first_animal: Animal = Animal::new(width  / 2 as i32,
                                               height / 2 as i32,
                                               1000,
                                               0,
                                               [1, 1, 10, 1, 1, 1, 1, 1]);
    let mut animals: Vec<&mut Animal> = vec![&mut first_animal];

    let mut plants: &mut HashMap<(i32, i32), bool> = &mut HashMap::new();

    // testing loops - looking for mutable values
    for i in 1..3 {
        add_plants(plants, [0, 0, width, height]);
        add_plants(plants, jungle);
    }

    for (k, v) in plants.iter() {
        println!("{:?}, {:?}", k, v);
    }

    for i in animals.iter_mut() {
        i.show();
        move_animal(i);
        animal_eat(i, plants, plant_energy);
        i.show();
    }

}

fn random_plant(list: [i32; 4]) -> (i32, i32) {

    let mut rng = rand::thread_rng();
    let width_between  = Range::new(0, list[2]);
    let height_between = Range::new(0, list[3]);

    let r_num_left = list[0] + width_between.ind_sample(&mut rng);
    let r_num_top  = list[1] + height_between.ind_sample(&mut rng);

    let pos = (r_num_left, r_num_top);

    pos
}

fn add_plants(plants: &mut HashMap<(i32, i32), bool>, list: [i32; 4]) {
    plants.insert(random_plant(list), true);
}

fn move_animal(animal: &mut Animal) {

    let dir = animal.dir;

    animal.x = if dir >= 2 && dir < 5  { animal.x + 1 }
          else if dir == 1 || dir == 5 { animal.x }
          else { animal.x - 1};

    animal.y = if dir >= 0 && dir < 3 { animal.y - 1 }
          else if dir >= 4 && dir < 7 { animal.y + 1 }
          else { animal.y };

    animal.energy -= 1;
}

fn animal_eat(animal: &mut Animal, plants: &mut HashMap<(i32, i32), bool>, plant_energy: i32) {

    let pos = (animal.x, animal.y);

    if plants.contains_key(&pos) {
        animal.energy += plant_energy;
        plants.remove(&pos);
    };
}

fn reproduce_animal(animal: &mut Animal, rep_energy: i32, animals: &mut Vec<&mut Animal>) {

    if animal.energy >= rep_energy {
        animal.energy = animal.energy / 2 as i32;
        let mut new_animal = Animal::new(animal.x,
                                         animal.y,
                                         animal.energy,
                                         animal.dir,
                                         animal.genes);
        animals.push(&mut new_animal);
    }
    //mess with a random gene to get a variation
}

// fn turn_animal(animal: &mut animal) {
//
// }
