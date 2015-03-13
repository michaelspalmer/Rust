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
        plant::add_plants(plants);
        // plant::add_plants(plants, [0, 0, width, height]);
        // plant::add_plants(plants, jungle);

        if animals[i].alive {
            animal::animal_turn(&mut animals[i]);
            animal::animal_move(&mut animals[i], width, height);
            animal::animal_eat(&mut animals[i], plants, plant_energy);
            if animal::animal_reproduce(&mut animals[i], reproduction_energy) {
                animal::add_animal(animal::copy_animal(&mut animals[i]), animals);
            }
            animal::is_alive(&mut animals[i]);
        }
    }
    animal::remove_dead(animals);
}

pub fn draw_world(animals: Vec<animal::Animal>, plants: HashMap<(i32, i32), bool>, width: i32, height: i32) {

    for y in 0..height {
        print!("\n");
        print!("|");
        for x in 0..width {
            let pos = (x as i32, y as i32);
            for animal in animals.iter() {
                if animal.x == x && animal.y == y && animal.alive { print!("M"); }
            }
            if plants.contains_key(&pos) { print!("*"); } else { print!(" "); }
        }
    }
    print!("\n");
}
