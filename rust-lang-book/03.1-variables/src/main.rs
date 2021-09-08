fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{}, {}, {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);

    // array with 5 i32 elements
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0]; // 1
    let second = a[1]; // 2

    // 5 elements array initialized with 3 for every element
    let five_threes = [3; 5]; // [3, 3, 3, 3, 3]
}
