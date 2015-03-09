struct Point{
    x: i32,
    y: i32,
}



fn main() {

    let (x, y) = countEm(12, 16);

    println!("The answer is {} and {}!", x, y);

    let mut point = Point { x: 1, y: 2 };

    println!("Point is now: {}. {}", point.x, point.y);

    let (n, m) = change_the_loc(point.x, point.y, 5);

    println!("Point has moved to: {}. {}", n, m);

}

fn countEm (x: i32, y: i32) -> (i32, i32) {

        (x + y, x * y)
}

fn change_the_loc(x: i32, y: i32, factor: i32) -> (i32, i32) {

    (x * factor, y * factor)

}
