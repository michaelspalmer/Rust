use std::rand;
use std::collections::HashMap;
use std::rand::distributions::{IndependentSample, Range};

pub fn random_plant(list: [i32; 4]) -> (i32, i32) {

    let mut rng = rand::thread_rng();
    let width_between  = Range::new(0, list[2]);
    let height_between = Range::new(0, list[3]);

    let r_num_left = list[0] + width_between.ind_sample(&mut rng);
    let r_num_top  = list[1] + height_between.ind_sample(&mut rng);

    let pos = (r_num_left, r_num_top);

    pos
}

pub fn add_plants(plants: &mut HashMap<(i32, i32), bool>, list: [i32; 4]) {
    plants.insert(random_plant(list), true);
}
