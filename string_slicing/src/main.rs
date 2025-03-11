fn main() {
    let inp_str = String::from("hello world");

    let word = first_word(&inp_str);
    let len = word.len();
    println!("inp_str: '{inp_str}' | word: '{word}' | len '{len}'");

    let word = first_word(&inp_str[len + 1..]);
    println!("word: {word}");

    let str_lit = "goodbye rust";

    let word = first_word(str_lit);
    let len = word.len();
    println!("str_lit: '{str_lit}' | word: '{word}' | len '{len}'");

    let word = first_word(&str_lit[len + 1..]);
    println!("word: {word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}
