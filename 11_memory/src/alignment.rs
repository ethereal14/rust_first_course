use std::mem::{align_of, size_of};

struct s1 {
    a: u8,
    b: u16,
    c: u8,
}

struct s2 {
    a: u8,
    b: u8,
    c: u16,
}

fn main() {
    println!("sizeof s1: {}, s2 {}", size_of::<s1>(), size_of::<s2>());
    println!("alignof s1: {}, s2 {}", align_of::<s1>(), align_of::<s2>());
}
