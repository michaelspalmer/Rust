use std::collections::HashMap;
use std::rand;
use std::rand::distributions::{IndependentSample, Range};

fn main() {

    let mut plants: HashMap<(i32, i32), bool> = HashMap::new();
    
    plants.insert((12,2), true);
    plants.insert((12,34), true);
    plants.insert((88,44), false);
    
    println!("Size: {}", plants.len());
    
    for (k, v) in plants.iter() {
        println!("{:?}", k);
    }
    
    let WIDTH = 100;
    
    let between = Range::new(0, WIDTH);
    let mut rng = rand::thread_rng();
    let r_num = between.ind_sample(&mut rng);
    println!("random: {}", r_num);
    
    rand_stuff([0,0,10,10]);
    
}
fn rand_stuff(list: [i32; 4]) {
    let mut rng = rand::thread_rng();
    let width_between = Range::new(list[0], list[2]);
    let height_between = Range::new(list[1], list[3]);
    
    let r_num_left = width_between.ind_sample(&mut rng);
    let r_num_top = height_between.ind_sample(&mut rng);
    
    let pos = (r_num_left as i32, r_num_top as i32);
    
    println!("{:?}", pos);
}