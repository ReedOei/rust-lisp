fn tokenize_split(s: &str) -> Vec<String> {
    return s.replace("(", " ( ").replace(")", " ) ").replace(";", " ; ")
            .split_whitespace().map(String::from).collect();
}

#[derive(Clone)]
enum LexemeType {
    Int,
    Op,
    Identifier,
    ParenStart,
    ParenEnd
}

fn lex_type_str(lex_type: &LexemeType) -> String {
    String::from(match lex_type {
        LexemeType::Int => "Int",
        LexemeType::Op => "Op",
        LexemeType::Identifier => "Identifier",
        LexemeType::ParenStart => "ParenStart",
        LexemeType::ParenEnd => "ParenEnd"
    })
}

impl std::fmt::Display for LexemeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", lex_type_str(self))
    }
}

#[derive(Clone)]
struct Lexeme {
    s: String,
    lex_type: LexemeType
}

fn is_int(s: &String) -> bool {
    return s.parse::<i32>().is_ok()
}

fn lex(tokens: Vec<String>) -> Vec<Lexeme> {
    let mut lexs: Vec<Lexeme> = Vec::new();

    for token in tokens {
        if token == "+" {
            lexs.push(Lexeme{s: token, lex_type: LexemeType::Op});
        } else if token == "(" {
            lexs.push(Lexeme{s: token, lex_type: LexemeType::ParenStart});
        } else if token == ")" {
            lexs.push(Lexeme{s: token, lex_type: LexemeType::ParenEnd});
        } else if is_int(&token) {
            lexs.push(Lexeme{s: token, lex_type: LexemeType::Int});
        } else {
            lexs.push(Lexeme{s: token, lex_type: LexemeType::Identifier});
        }
    }

    return lexs
}

fn main() {
    let code = "print (+ 10 (+ 30 (+ (+ 7 7) 7)))";
    let v: Vec<String> = tokenize_split(code);

    for i in v {
        println!("Token: {}", i);
    }

    let mut lexs: Vec<Lexeme> = lex(tokenize_split(code));

    for lex in lexs.clone() {
        println!("Lexeme: {}, {}", lex.s, lex.lex_type);
    }
}

