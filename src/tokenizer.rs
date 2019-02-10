fn tokenize(s: &str) -> Vec<&str> {
    let mut v: Vec<&str> = Vec::new();

    let mut prev_token_start = 0;
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            v.push(&s[prev_token_start..i]);
            prev_token_start = i + 1;
        } else if c == ';' {
            v.push(&s[prev_token_start..i]);
            v.push(&s[i..=i]);
            prev_token_start = i + 2;
        }
    }

    if prev_token_start < s.len() {
        v.push(&s[prev_token_start..]);
    }

    return v
}

fn main() {
    let code = "print (+ 10 (+ 7 7))";
    let v: Vec<String> = tokenize(code);

    for i in v {
        println!("Token: {}", i);
    }
}

