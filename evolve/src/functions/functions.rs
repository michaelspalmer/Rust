use std::rand;
use std::old_io;
use animal::animal;
use std::collections::HashMap;
use std::rand::distributions::{IndependentSample, Range};

static PLANT_ENERGY: i32 = 80;
static REPRODUCTION_ENERGY: i32 = 200;
// use 150 X 50 with jungle 40 X 20 for big screen
static WIDTH: i32 = 100;
static HEIGHT: i32 = 30;

pub fn simulate_day(animals: &mut Vec<animal::Animal>,
                    plants: &mut HashMap<(i32, i32), bool>,)
{
    let range = 0..animals.len();

   add_plants(plants, WIDTH, HEIGHT);

    for i in range {
        if animals[i].alive {
            animals[i].turn();
            animals[i].ani_move(WIDTH, HEIGHT);
            animals[i].eat(plants, PLANT_ENERGY);
            if animals[i].reproduce(REPRODUCTION_ENERGY) {
                add_animal(copy_animal(&mut animals[i]), animals);
            }
            animals[i].is_alive();
        }
    }
    remove_dead(animals);
}

pub fn copy_animal(animal: &mut animal::Animal) -> animal::Animal {

    let mut new_animal = animal::Animal::new(animal.x,
                                             animal.y,
                                             animal.energy,
                                            (animal.dir + 1) % 8,
                                             animal.genes.clone(),
                                             animal.alive);
    mut_gene(&mut new_animal);
    new_animal
}

pub fn add_animal(animal: animal::Animal, animals: &mut Vec<animal::Animal>) {

    animals.push(animal);
}

pub fn gen_genes() -> Vec<i32> {

    let mut genes: Vec<i32> = vec![];
    let range   = 0..8;

    for _ in range {
        genes.push(gen_random_nbr(0, 10));
    }
    
    genes
}

pub fn mut_gene(animal: &mut animal::Animal) {

    let mutation_val = Range::new(0, 3);
    let index = Range::new(0, animal.genes.len());
    let mut rng = rand::thread_rng();

    let mutation: i32 = mutation_val.ind_sample(&mut rng);
    let index = index.ind_sample(&mut rng);
    // let len = animal.genes.len() as i32;
    // let mut mutation: i32 = gen_random_nbr(0, 3);
    // let index = gen_random_nbr(0, len);

    animal.genes[index] += mutation;
}

pub fn remove_dead(animals: &mut Vec<animal::Animal>) {

    let range = 0..animals.len();
    let mut count: i32 = 0;
    let mut hold: animal::Animal;
    let length = animals.len();

    for i in range {
        if !animals[i].alive {
           hold = animals.remove(i as usize);
           animals.push(hold);
           count += 1;
        }
    }
    animals.truncate(length - count as usize);
}

pub fn draw_world(animals: &Vec<animal::Animal>,
                  plants: &HashMap<(i32, i32), bool>)
{
    let mut has_animal: bool;
    let mut pos: (i32, i32);

    for y in 0..HEIGHT {

        print!("\n");
        print!("|");

        for x in 0..WIDTH {

            pos = (x, y);
            has_animal = false;

            for animal in animals.iter() {
                if animal.x == x && animal.y == y && animal.alive {
                    print!("\x1b[31mM\x1b[0m");
                    has_animal = true;
                }
            }
            
            if has_animal { continue; }
            
            else if plants.contains_key(&pos) {
                let flower_num = gen_random_nbr(1,3);
                
                     if flower_num == 1 { print!("\x1b[32m*\x1b[0m"); } 
                else if flower_num == 2 { print!("\x1b[33m*\x1b[0m"); } 
                else                    { print!("\x1b[34m*\x1b[0m"); }
                   
            } else { print!(" "); }
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
        Ok(n)  => n,
        Err(_) => 0,
    }
}

pub fn gen_random_nbr(low: i32, high: i32) -> i32 {

    let mut rng = rand::thread_rng();
    let bound   = Range::new(low, high);
    let rtn_nbr = bound.ind_sample(&mut rng);

    rtn_nbr
}

pub fn random_plant(w: i32, h: i32, jungle: bool) -> (i32, i32) {

    let mut rng = rand::thread_rng();
    let width;
    let height;

    if jungle {
        width  = Range::new(w / 3, (w / 3) + 10);
        height = Range::new(h / 3, (h / 3) + 10);
    } else {
        width  = Range::new(0, w);
        height = Range::new(0, h);
    }

    let x =  width.ind_sample(&mut rng);
    let y = height.ind_sample(&mut rng);

    let pos = (x, y);

    pos
}

pub fn add_plants(plants: &mut HashMap<(i32, i32), bool>, width: i32, height: i32) {
    plants.insert(random_plant(width, height, false), true);
    plants.insert(random_plant(width, height, true), true);
}