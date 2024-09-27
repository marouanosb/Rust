fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    let c = comparer_taille(&s1,&s2);

    if c {
        println!("Les tailles sont égales.");
    } else {
        println!("Les tailles sont différentes.")
    }
}

fn calculer_taille(s: String) -> (String, usize) {
    let taille = s.len();

    (s, taille)
}

fn comparer_taille(s1: &String, s2: &String) -> (bool) {
    let taille1 = s1.len();
    let taille2 = s2.len();

    (taille1 == taille2)
}