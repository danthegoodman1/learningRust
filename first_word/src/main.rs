use std::io;

fn main() {
    println!("Give me a string, and I will tell you the first word!");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("invalid input!");

    let fw = first_word(&input);

    println!("The first word was '{fw}'")
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i] // when we find a space, return the first word
        }
    }
    &s[..] // otherwise it's one word
}
