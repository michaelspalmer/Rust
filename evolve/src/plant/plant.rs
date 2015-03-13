use std::rand;
use std::collections::HashMap;
use std::rand::distributions::{IndependentSample, Range};
use std::num::SignedInt;
use std::num;

pub fn random_plant(list: [i32; 4]) -> (i32, i32) {

    // [0, 0, width, height]
    // [45, 10, 10, 10] (jungle)

    let mut rng = rand::thread_rng();
    let width_between  = Range::new(0, list[2]);
    let height_between = Range::new(0, list[3]);

    let r_num_left = (list[0] + width_between.ind_sample(&mut rng) % list[2]);
    let r_num_top  = (list[1] + height_between.ind_sample(&mut rng) % list[3]);

    let pos = (r_num_top, r_num_left);

    pos
    
    // animal.dir = SignedInt::abs(rand::random::<i32>() % 8);
    
    //The problem here is that there are size issues - this random number creates
    // some big numbers.  Above, you tried to restrict it to a range - maybe see if that
    // will work again?  Re running with the above is closer - 
    
    // let pos: (i32, i32) = ((SignedInt::abs(rand::random::<i8>()) + list[0] as i8) % list[2] as i8, 
    //           (SignedInt::abs(rand::random::<i8>()) + list[1] as i8) % list[3] as i8);
    
    // pos 
}

pub fn add_plants(plants: &mut HashMap<(i32, i32), bool>, list: [i32; 4]) {
    plants.insert(random_plant(list), true);
}
