fn main() {
    let ref_to_nothing = dangle();

    println!("{}", ref_to_nothing);

    let s = String::from("hello world!");
    let word = first_word(&s);
    println!("{}", word);
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
