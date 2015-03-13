use std::rand;
use std::collections::HashMap;
use std::rand::distributions::{IndependentSample, Range};
use std::num::SignedInt;
use std::num;

// pub fn random_plant(list: [i32; 4]) -> (i32, i32) {
//
//     // [0, 0, width, height]
//     // [45, 10, 10, 10] (jungle)
//
//     let mut rng = rand::thread_rng();
//     let width_between  = Range::new(list[0], list[2]);
//     let height_between = Range::new(list[1], list[3]);
//
//     let y = (list[0] + width_between.ind_sample(&mut rng) % list[2]);
//     let x = (list[1] + height_between.ind_sample(&mut rng) % list[3]);
//
//     let pos = (x, y);
//
//     pos
//
//     // animal.dir = SignedInt::abs(rand::random::<i32>() % 8);
//
//     //The problem here is that there are size issues - this random number creates
//     // some big numbers.  Above, you tried to restrict it to a range - maybe see if that
//     // will work again?  Re running with the above is closer -
//
//     // let pos: (i32, i32) = ((SignedInt::abs(rand::random::<i8>()) + list[0] as i8) % list[2] as i8,
//     //           (SignedInt::abs(rand::random::<i8>()) + list[1] as i8) % list[3] as i8);
//
//     // pos
// }

pub fn random_plant(w: i32, h: i32, jungle: bool) -> (i32, i32) {

    let mut rng = rand::thread_rng();
    let width;
    let height;

    if jungle {
        width = Range::new(45, 55);
        height = Range::new(10, 20);
    } else {
        width = Range::new(0, w);
        height = Range::new(0, h);
    }

    let x = width.ind_sample(&mut rng);
    let y = height.ind_sample(&mut rng);

    let pos = (x, y);

    pos
}


pub fn add_plants(plants: &mut HashMap<(i32, i32), bool>) {
    plants.insert(random_plant(100, 30, false), true);
    plants.insert(random_plant(0, 0, true), true);
}
