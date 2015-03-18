extern crate rand;
extern crate evolve;

use std::collections::HashMap;
use evolve::animal::animal;
use evolve::functions::functions;

static WIDTH: i32 = 100;
static HEIGHT: i32 = 30;

fn main() {
    let mut time_count: i32 = 0;
    let mut plants: HashMap<(i32, i32), bool> = HashMap::new();
    let mut animals: Vec<animal::Animal> = vec![animal::Animal::new(WIDTH / 2,
                                                                    HEIGHT / 2,
                                                                    1000,
                                                                    0,
                                                                    functions::gen_genes(),
                                                                    true)];
    loop {
        let user_input: i32 = functions::ask_for_input();
        time_count += user_input;

        for _ in 0..user_input {
            functions::simulate_day(&mut animals,
                                    &mut plants);
        }

        print!("{} days passed\t", time_count);
        print!(" {} years passed\t",time_count / 365 as i32);
        print!(" Total Animals: {}\t", animals.len());
        print!(" Total Plants: {}\t", plants.len());

    functions::draw_world(&animals, &plants);
    // println!("{:?}", animals[0].show());
    }
}
