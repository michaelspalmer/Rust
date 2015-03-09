fn main()
{

    let sum = (1..4).fold(0, |sum, x| sum + x);
    let new_sum = (0..5).fold(0, |sum, x| sum + add_ten(x));
    let map_one = (1..10).map(|x| x + 1);
    let filter_one = (1..100).filter(|&x| x % 5 == 0);
    let filter_chain = (1..1000)
                        .filter(|&x| x % 2 == 0)
                        .filter(|&x| (x / 3) % 12 == 0)
                        .take(20)
                        .collect::<Vec<i32>>();
    
    for i in 0..sum {
        println!("{}", i);
    }
    for i in 0..new_sum {
        println!("{}", i);
    }
    for i in map_one {
        println!("{}", i);
    }
    for i in filter_one {
        println!("{}", i);
    }
    for i in &filter_chain {
        println!("{}", i);
    }
}

fn add_ten(x: i32) -> i32
{
    x + 10;
    
    x
}