fn main() {

    let mut vec: Vec<i32> = vec![0; 10];
    
    loop_through(&mut vec);

}

fn add_ten(x: usize) {
    x + 10;
}

fn loop_through(vec: &mut Vec<i32>) {

    let mut range = 0..vec.len();
    
    for i in range {
    
        add_ten(i);
        vec.push(i as i32);
        println!("{}", vec.len());
    }
}