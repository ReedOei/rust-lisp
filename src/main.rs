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

#[derive(Clone)]
enum Expr {
    IntExpr(i32),
    VarExpr(String),
    Add(Box<Expr>, Box<Expr>)
}

#[derive(Clone)]
struct Stmt {
    name: String,
    arg: Expr
}

fn parse_expr(lexs: &mut Vec<Lexeme>) -> Option<Expr> {
    let head = lexs.remove(0);

    match head.lex_type {
        // Safe because we know that it parsed before
        LexemeType::Int => Some(Expr::IntExpr(head.s.parse::<i32>().unwrap())),
        LexemeType::Identifier => Some(Expr::VarExpr(head.s)),
        LexemeType::Op => {
            if head.s == "+" {
                return parse_expr(lexs)
                    .and_then(|e1| parse_expr(lexs).and_then(|e2| Some(Expr::Add(Box::new(e1), Box::new(e2)))));
            } else {
                return None;
            }
        }
        LexemeType::ParenStart =>
            parse_expr(lexs).and_then(|expr| {
                let end_paren = lexs.remove(0);
                return match end_paren.lex_type {
                    LexemeType::ParenEnd => Some(expr),
                    _ => None
                }
            }),
        _ => None
    }
}

fn parse(lexs: &mut Vec<Lexeme>) -> Option<Vec<Stmt>> {
    if lexs.is_empty() {
        return Some(Vec::new());
    }

    let head = lexs.remove(0);

    match head.lex_type {
        LexemeType::Identifier => parse_expr(lexs).and_then(|expr| {
            let stmt = Stmt{name: head.s, arg: expr};

            return parse(lexs).and_then(|stmts| {
                let mut res = stmts.clone();
                res.insert(0, stmt);
                return Some(res);
            });
        }),
        _ => None
    }
}

fn expr_str(e: &Expr) -> String {
    match e {
        Expr::VarExpr(s) => s.clone(),
        Expr::IntExpr(i) => i.to_string(),
        Expr::Add(e1, e2) => "(+ ".to_string() + &expr_str(e1) + " " + &expr_str(e2) + ")"
    }
}

fn eval(expr: Expr) -> i32 {
    match expr {
        Expr::IntExpr(i) => i,
        Expr::Add(e1, e2) => eval(*e1) + eval(*e2),
        Expr::VarExpr(_) => 0
    }
}

fn exec(stmts: Vec<Stmt>) {
    for stmt in stmts {
        if stmt.name == "print" {
            println!("{}", eval(stmt.arg));
        }
    }
}

fn main() {
    let code = "print (+ 10 (+ 30 (+ (+ 7 7) 7))) print (+ -10 40)";
    let v: Vec<String> = tokenize_split(code);

    for i in v {
        println!("Token: {}", i);
    }

    let mut lexs: Vec<Lexeme> = lex(tokenize_split(code));

    for lex in lexs.clone() {
        println!("Lexeme: {}, {}", lex.s, lex.lex_type);
    }

    let mut expr_lexs = lex(tokenize_split("(+ (+ 1 4) 2)"));
    let expr: Expr = parse_expr(&mut expr_lexs).unwrap();
    println!("Expr: {}", expr_str(&expr));

    let parsed: Vec<Stmt> = parse(&mut lexs).unwrap();

    for stmt in parsed.clone() {
        println!("Stmt: {} {}", stmt.name, expr_str(&stmt.arg));
    }

    exec(parsed);
}

