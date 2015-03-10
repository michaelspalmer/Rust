use std::num::Float;

enum What<T, E> {
    Good(T),
    Bad(E),
}

fn main() {

    let n: Result<f64, String> = inverse(12.0);
    let m = inverse(123.0);
    let t: What<f64, String> = do_stuff(12.0f64);
    let v: What<f64, String> = do_stuff(123.0);
    
    match n {
        Ok(x) => println!("The inverse of 12 is {}", x),
        Err(msg) => println!("Error: {}", msg),
    }
    
    match m {
        Ok(x) => println!("The inverse of 123 is {}", x),
        Err(msg) => println!("ErrorL {}", msg),
    }
    
    match t {
        What::Good(x) => println!("That number times 100 is {}", x),
        What::Bad(msg) => println!("Error: {}", msg),
    }
    
    match v {
        What::Good(x) => println!("That number times 100 is {}", x),
        What::Bad(msg) => println!("Error: {}", msg),
    }
    
}

fn inverse<T: Float>(x: T) -> Result<T, String> {

    if x == Float::zero() { return Err("x cannot be zero!".to_string()); }
    
    let one = Float::one();
    
    Ok(one / x)
}

fn do_stuff(x: f64) -> What<f64, String> {
    if x < 100.0 {return What::Good(x * 100.0); }
    
    What::Bad("Your number is too big!".to_string())
}