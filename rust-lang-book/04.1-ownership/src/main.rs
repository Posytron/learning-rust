fn main() {
    let mut s = String::from("hello");

    

    change(&mut s);

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
*/
