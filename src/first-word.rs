fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn main() {
    let s1 = "apple";
    let s2 = String::from("banana tree");
    println!("The first word of '{}' is '{}'.", s1, first_word(s1));
    println!("The first word of '{}' is '{}'.", s2, first_word(&s2));
    println!("The first word of '{}' is '{}'.", &s2[2..], first_word(&s2[2..]));
}
