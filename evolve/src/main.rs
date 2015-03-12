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

    let first_animal: animal::Animal = animal::Animal::new(width  / 2 as i32,
                                               height / 2 as i32,
                                               1000,
                                               0,
                                               [1, 1, 10, 1, 1, 1, 1, 1],
                                               true);
                                               
    let mut animals: Vec<animal::Animal> = vec![first_animal];

    let mut plants: &mut HashMap<(i32, i32), bool> = &mut HashMap::new();
    
     while animals.len() < 500 {
        functions::simulate_day(&mut animals, &mut plants, 
                                plant_energy, reproduction_energy, 
                                jungle, width, height);
    }
    println!("Number of animals: {}", animals.len());
    
    let range = 0..animals.len();
    let mut count = 0;
    for i in range {
        
        if animals[i].alive {
            count += 1;
            // animals[i].show();
            // println!("Count: {}", count);
        }
    }
    
    animals[100].show();
    println!("Count: {}", count);
}