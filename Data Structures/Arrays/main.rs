use std::mem;

fn main() {
    let mut a: [i32; 5] = [1,2,3,4,5]; 

    println!("a has {} elements first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a has {} elements first is {}", a.len(), a[0]);

    println!("{:?}", a);

    let b = [1; 10]; // to create an array fulled with the same value, x number of times
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32; 3]; 2] = 
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];

    println!("{:?}", mtx)
}