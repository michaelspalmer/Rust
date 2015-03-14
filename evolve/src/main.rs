extern crate evolve;
// extern crate rand;

use std::collections::HashMap;
use evolve::animal::animal;
use evolve::functions::functions;

fn main() {
    // use 150 X 50 with jungle 40 X 20 for big screen
    let width: i32 = 100;
    let height: i32 = 30;
    let plant_energy: i32 = 80;
    let mut time_count: i32 = 0;
    let reproduction_energy: i32 = 200;
    let mut plants: HashMap<(i32, i32), bool> = HashMap::new();
    let mut animals: Vec<animal::Animal> = vec![animal::Animal::new(width / 2,
                                                                    height / 2,
                                                                    1000,
                                                                    0,
                                                                    animal::gen_genes(),
                                                                    true)];
    loop {
        let user_input: i32 = functions::ask_for_input();
        time_count += user_input;

        for _ in 0..user_input {
            functions::simulate_day(&mut animals,
                                    &mut plants,
                                    plant_energy,
                                    reproduction_energy,
                                    width,
                                    height);
        }

        print!("{} days passed\t", time_count);
        print!(" {} years passed\t",time_count / 365 as i32);
        print!(" Total Animals: {}\t", animals.len());
        print!(" Total Plants: {}\t", plants.len());

    functions::draw_world(&animals, &plants, width, height);
    // println!("{:?}", animals[0].show());
    }
}
