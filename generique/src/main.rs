fn tous_egaux<T: std::cmp::PartialEq>(x: T, y: T, z: T) -> bool {
    if x == y && y == z {true}
    else {false}
}

fn correct_str<'a,'b>(x: &'a str, y: &'b str) -> Option<&'a str> {
    if x == y {Some(x)}
    else {None}
}

fn main() {
    println!("Hello, world!");
}
