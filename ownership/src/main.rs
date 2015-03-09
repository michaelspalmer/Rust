fn main()
{

    let n = Box::new(5);
    let mut x = 5;
    
    println!("Outside n is: {}", n);
    println!("Outside x is: {}", x);
    
    add_one_borrow(&mut x);
    
    let y = add_one_owner(n);
    
    println!("Outside x is now: {}", x);
    println!("Outside y is now: {}", y);
}

fn add_one_owner(mut num: Box<i32>) -> Box<i32>
{
    println!("Inside owner n (BEFORE): {}", num);
    *num += 1;
    println!("Inside owner n (AFTER): {}", num);
    num
}

fn add_one_borrow(num: &mut i32)
{
    println!("Inside borrow &mut x (BEFORE): {}", num);
    *num += 1;
    println!("Inside borrow &mut x (AFTER): {}", num);
}