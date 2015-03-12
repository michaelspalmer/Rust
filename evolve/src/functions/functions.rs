
use animal::animal;
use plant::plant;
use std::collections::HashMap;

pub fn simulate_day(animals: &mut Vec<animal::Animal>,
                    plants: &mut HashMap<(i32, i32), bool>,
                    plant_energy: i32,
                    reproduction_energy: i32,
                    jungle: [i32; 4],
                    width: i32,
                    height: i32) {

    let range = 0..animals.len();

    for i in range {
        plant::add_plants(plants, [0, 0, width, height]);
        plant::add_plants(plants, jungle);

        if animals[i].alive {
            // animal::animal_turn(&mut animals[i]);
            animal::animal_move(&mut animals[i]);
            animal::animal_eat(&mut animals[i], plants, plant_energy);
            if animal::animal_reproduce(&mut animals[i], reproduction_energy) {
                animal::add_animal(animal::copy_animal(&mut animals[i]), animals);
            }
            animal::is_alive(&mut animals[i]);
        }
    }
}
