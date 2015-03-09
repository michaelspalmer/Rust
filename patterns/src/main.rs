//! broke off into another, getting into iterators...

fn main ()
{
    for num in 0..10 {
        print_them(num);
    }
    
    print_iter(10);
}

fn print_them (num: i32)
{
    match num {
        n @ 1 ... 5 => println!("{}", n),
        _ => println!("nada"),
    }
}

fn print_iter (num: i32)
{
    let mut range = 0..num;
    
    let first_10 = (0..11).collect::<Vec<i32>>();
    
    for num in &first_10 {
        println!("Vec: {}", num);
    }
    
    loop {
        match range.next() {
            Some(n) => println!("{}", n),
            None    => break
        }
    }
}