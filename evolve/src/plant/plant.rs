use std::rand;
use std::collections::HashMap;
use std::rand::distributions::{IndependentSample, Range};

pub fn random_plant(w: i32, h: i32, jungle: bool) -> (i32, i32) {

    let mut rng = rand::thread_rng();
    let width;
    let height;

    if jungle {
        width = Range::new(w / 3, (w / 3) + 10);
        height = Range::new(h / 3, (h / 3) + 10);
    } else {
        width = Range::new(0, w);
        height = Range::new(0, h);
    }

    let x = width.ind_sample(&mut rng);
    let y = height.ind_sample(&mut rng);

    let pos = (x, y);

    pos
}

pub fn add_plants(plants: &mut HashMap<(i32, i32), bool>, width: i32, height: i32) {
    plants.insert(random_plant(width, height, false), true);
    plants.insert(random_plant(width, height, true), true);
}
