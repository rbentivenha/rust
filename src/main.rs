fn main () {

    let str_literal = "This is an string slice";

    let word = return_word(str_literal);

    println!("Word: {}", word);
}

fn return_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}