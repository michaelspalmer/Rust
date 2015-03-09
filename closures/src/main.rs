fn main()
{
    let x: i32 = 5;
    
    let add_one = |x| { 1 + x };
    let printer = || { println!("x is: {}", x) };
    let address_printer = |x: i32| { println!("address of x is: {:p}", &x) };
    
    printer();
    
    address_printer(x);
    
    println!("the sum of x and one is: {}", add_one(x));
    
    println!("The value of x squared twice: {}", twice(x, |x: i32| { x * x }));
    
    println!("x composed: {}", compose(x, |n: i32| { n * 10 },
                                          |n: i32| { n / 10 }));
}

fn twice<F> (x: i32, f: F) -> i32 
    where F: Fn(i32) -> i32
{
    f(x) + f(x)
}

fn compose<F, G> (x: i32, f: F, g: G) -> i32
    where F: Fn(i32) -> i32, G: Fn(i32) -> i32
{
    g(f(x))
}
