fn main() {
    let s = String::from("hello world");

    println!("前5个字母是 {}", &s[..5]);

    println!("第1个单词是 {}", first_word(&s));

    println!("第1个单词是 {}", first_word_slice(&s));

    let _sl: &str = "Hello, world!";
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
