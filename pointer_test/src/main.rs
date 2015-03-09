#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {

    let x: i32 = 5;
    let y: i32 = 8; 
    let z: &i32 = &y;
    
    println!("{}", x);
    println!("{:p}", &x);
    println!("{}", y);
    println!("{:p}", &y);
    println!("{}", z);
    println!("{:p}", &z);
    println!("\n");
    println!("{}", x + *z);
    println!("\n");
    
    println!("succ z: {}", succ(z));
    println!("succ &x: {}", succ(&x));
    
    println!("{}", z);
    println!("{:p}", &z);
    println!("{}", x);
    println!("{:p}", &x);
    
    println!("\n");
    let x = Box::new(5);
    println!("{}", x);
    println!("{}", add_one(&*x));
    println!("{}", add_one(&*x));
    println!("{}", add_one(&*x));
    println!("{}", x);
    
    println!("\n");
    
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    
    println!("{:?}", list);
}

fn succ(x: &i32) -> i32 { *x + 1 }

fn add_one(x: &i32) -> i32 { *x + 1 }