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
}

fn main() {

    let WIDTH: i32 = 100;
    let HEIGHT: i32 = 30;
    let JUNGLE: [i32; 4] = [45, 10, 10, 10];
    let PLANT_ENERGY = 80;
    let mut ANIMALS: [&mut Animal; 1] = [&mut Animal::new(WIDTH / 2 as i32,
                                                HEIGHT / 2 as i32,
                                                1000,
                                                0,
                                                [1, 1, 10, 1, 1, 1, 1, 1])];
    
    let mut plants: &mut HashMap<(i32, i32), bool> = &mut HashMap::new();
    
    for i in 1..10 {
        add_plants(plants, [0, 0, WIDTH, HEIGHT]);
        add_plants(plants, JUNGLE);
    }
    
    for (k, v) in plants.iter() {
        println!("{:?}, {:?}", k, v);
    }
}

fn random_plant(list: [i32; 4]) -> (i32, i32) {

    let mut rng = rand::thread_rng();
    let width_between = Range::new(0, list[2]);
    let height_between = Range::new(0, list[3]);
    
    let r_num_left = list[0] + width_between.ind_sample(&mut rng);
    let r_num_top = list[1] + height_between.ind_sample(&mut rng);
    
    let pos = (r_num_left, r_num_top);
    
    pos
}

fn add_plants(plants: &mut HashMap<(i32, i32), bool>, list: [i32; 4]) {
    plants.insert(random_plant(list), true);
}

fn move(animal: &mut Animal) {
    let dir = animal.dir;
    let x = animal.x;
    let y = animal.y;
    
    if dir >= 2 && dir < 5 { animal.x = x + 1 };
    else if dir == 1 || dir == 5 { animal.x = x + 0 };
    else { animal.x = x - 1);
    
    