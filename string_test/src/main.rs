use std::str;


fn take_slice (slice: &str) {
    println!("Got {}", slice);
}

fn main () {

    let s = "Hello".to_string();
    let x: &[u8] = &[b'a', b'b'];
    let stack_string: &str = str::from_utf8(x).unwrap();

    println!("x: {:?}", x);
    println!("stack_string: {}", stack_string);
    println!("String version: {}", s);
    take_slice (&s);

    for elem in s.graphemes(true) {
        println!("{}", elem);
    }

    for elem in s.chars() {
        println!("{}", elem);
    }

    for elem in s.bytes() {
        println!("{}", elem);
    }
}
