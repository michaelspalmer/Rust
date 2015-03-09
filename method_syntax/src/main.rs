extern crate method_syntax;

use method_syntax::shapes::shapes;

fn main() {

    let c = shapes::Circle { x: 0.0, y: 0.0, radius: 2.0 };
    let d = c.grow().area();
    let e = shapes::Circle::new(0.0, 0.0, 5.0);
    let f = shapes::CircleBuilder::new()
                                    .coordinate(0.0, 0.0)
                                    .radius(5.0)
                                    .finalize();
    
    println!("{}", c.area());
    println!("{}", d);
    println!("{}", e.area());
    println!("{}", f.area());
}