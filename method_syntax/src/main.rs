extern crate method_syntax;

use method_syntax::shapes::shapes;

fn main() {

    let c = shapes::Circle { x: 0.0, y: 0.0, radius: 2.0 };
    let d = c.grow(5.0);
    let e = shapes::Circle::new(0.0, 0.0, 5.0);
    let f = shapes::CircleBuilder::new()
                                    .coordinate(0.0, 0.0)
                                    .radius(5.0)
                                    .finalize();
                                    
    let g = shapes::Square::new(2.0, 5.0, 10.0);
    let h = shapes::SquareBuilder::new()
                                    .coordinate(3.0, 5.0)
                                    .side(20.0)
                                    .finalize();
    

    println!("{}", c.area());
    println!("{}", d.area());
    println!("{}", e.area());
    println!("{}", f.area());
    println!("{}", g.area());
    println!("{}", h.area());
    println!("{}", h.x);
}