fn main() {
    let mut s = String::from("Hello");
    s.push_str(" world!");
    println!("{}", s);

    let t = String::from("12345");
    let p = f(t);
    println!("{}", p);
}

fn f(s: String) -> String {
    return s;
}
