fn premier_mot(s: &String) -> usize {
    let octets = s.as_bytes();

    for (i, &element) in octets.iter().enumerate() {
        if element == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

}
