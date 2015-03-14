use std::rand;
use std::old_io;
use plant::plant;
use animal::animal;
use std::collections::HashMap;
use std::rand::distributions::{IndependentSample, Range};

pub fn simulate_day(animals: &mut Vec<animal::Animal>,
                    plants: &mut HashMap<(i32, i32), bool>,
                    plant_energy: i32,
                    reproduction_energy: i32,
                    width: i32,
                    height: i32)
{
    let range = 0..animals.len();

    plant::add_plants(plants, width, height);

    for i in range {
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

pub fn draw_world(animals: &Vec<animal::Animal>,
                  plants: &HashMap<(i32, i32), bool>,
                  width: i32,
                  height: i32)
{
    let mut has_animal: bool;

    for y in 0..height {

        print!("\n");
        print!("|");

        for x in 0..width {

            let pos = (x as i32, y as i32);
            has_animal = false;

            for animal in animals.iter() {

                if animal.x == x && animal.y == y && animal.alive {

                    print!("\x1b[31mM\x1b[0m");
                    has_animal = true;
                }
            }
            if has_animal {

                continue;

            } else if plants.contains_key(&pos) {

                let flower_num = gen_random_nbr(1,3);

                if flower_num == 1 {

                    print!("\x1b[32m*\x1b[0m");

                } else if flower_num == 2 {

                    print!("\x1b[33m*\x1b[0m");
                    
                } else {

                    print!("\x1b[34m*\x1b[0m");
                }
            } else {

                print!(" ");
            }
        }
        print!("|");
    }
    print!("\n");
}

pub fn ask_for_input() -> i32 {

    let mut input = old_io::stdin().read_line().ok().expect("I/O Failure!");

    let shorten_by = input.len() - 1;
    input.truncate(shorten_by);

    match input.parse() {
        Ok(n)    => n,
        Err(msg) => 0,
    }
}

pub fn gen_random_nbr(low: i32, high: i32) -> i32 {

    let mut rng = rand::thread_rng();
    let bound = Range::new(low, high);
    let return_nbr = bound.ind_sample(&mut rng);

    return_nbr
}
