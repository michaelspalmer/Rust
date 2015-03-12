#![feature(rand)]

extern crate evolve;

use std::collections::HashMap;
use evolve::animal::animal;
use evolve::functions::functions;

fn main() {

    let width: i32 = 100;
    let height: i32 = 30;
    let jungle: [i32; 4] = [45, 10, 10, 10];
    let plant_energy: i32 = 80;
    let reproduction_energy: i32 = 200;
    let mut plants: &mut HashMap<(i32, i32), bool> = &mut HashMap::new();
    let mut animals: Vec<animal::Animal> = vec![animal::Animal::new(width  / 2,
                                                                    height / 2,
                                                                    1000,
                                                                    0,
                                                                   [1, 1, 10, 1, 1, 1, 1, 1],
                                                                    true)];
     for i in 0..1000 {
        functions::simulate_day(&mut animals,
                                &mut plants,
                                plant_energy,
                                reproduction_energy,
                                jungle,
                                width,
                                height);
    }

    println!("Number of animals: {}", animals.len());

    let range = 0..animals.len();
    let mut count = 0;

    for i in range {
        if animals[i].alive {
            count += 1;
        }
    }
    println!("{}", plants.len());
    println!("Count: {}", count);
}
