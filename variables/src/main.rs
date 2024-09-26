fn main() {
    let x = (1, 0, [(true,), (false,)]);
    println!("{}", if x.2[x.1].0 { "Yes "} else {"No"});
}