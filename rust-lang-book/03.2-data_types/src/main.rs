fn main() {
    println!("Hello, data_types!");

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //println!("a: {}", a);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {}", x);
}
