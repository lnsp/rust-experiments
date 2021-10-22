use std::io;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Token {
    Add,
    Subtract,
    Multiple,
    Divide,
    BracketOpen,
    BracketClose,
    Whitespace,
    Number,
}

type Value = (Token, f64);

fn precedence(t: &Token) -> u8 {
    match t {
        Token::Add | Token::Subtract => 1,
        Token::Multiple | Token::Divide => 2,
        _ => 0,
    }
}

fn tokenize(s: &str) -> Vec<Value> {
    let mut items: Vec<Value> = Vec::new();
    let mut last_token = Token::Whitespace;
    let mut j = 0;

    for (i, c) in s.as_bytes().iter().enumerate() {
        let token = match c {
            b'0'..=b'9' => Token::Number,
            b'+' => Token::Add,
            b'-' => Token::Subtract,
            b'*' => Token::Multiple,
            b'/' => Token::Divide,
            b'(' => Token::BracketOpen,
            b')' => Token::BracketClose,
            _ => Token::Whitespace,
        };
        if last_token != token {
            if j <= i && last_token != Token::Whitespace {
                let f = match last_token {
                    Token::Number => s[j..i].parse().expect("parse number"),
                    _ => 0.0,
                };
                items.push((last_token, f));
            }
            j = i;
            last_token = token;
        }
    }

    if last_token != Token::Whitespace {
        let f = match last_token {
            Token::Number => s[j..].parse().expect("parse number"),
            _ => 0.0,
        };
        items.push((last_token, f));
    }

    return items;
}

fn compute(tokens: &[Value]) -> Option<f64> {
    let mut stack: Vec<Value> = Vec::new();
    let mut output: Vec<Value> = Vec::new();

    for t in tokens {
        match t.0 {
            Token::Number => output.push(t.clone()),
            Token::Add | Token::Subtract | Token::Multiple | Token::Divide => {
                while !stack.is_empty() {
                    let s = stack.last()?;
                    if s.0 != Token::BracketOpen && precedence(&s.0) >= precedence(&t.0) {
                        output.push(stack.pop()?.clone())
                    } else {
                        break;
                    }
                }
                stack.push(t.clone())
            }
            Token::BracketOpen => stack.push(t.clone()),
            Token::BracketClose => {
                while stack.last()?.0 != Token::BracketOpen {
                    output.push(stack.pop()?.clone())
                }
                stack.pop();
            }
            Token::Whitespace => (),
        }
    }

    while !stack.is_empty() {
        output.push(stack.pop()?)
    }

    for t in output {
        match t.0 {
            Token::Number => stack.push(t),
            Token::Add | Token::Subtract | Token::Multiple | Token::Divide => {
                let s1 = stack.pop()?;
                let s2 = stack.pop()?;

                if s1.0 != Token::Number || s2.0 != Token::Number {
                    panic!("expected number")
                }

                match t.0 {
                    Token::Add => stack.push((Token::Number, s2.1 + s1.1)),
                    Token::Subtract => stack.push((Token::Number, s2.1 - s1.1)),
                    Token::Multiple => stack.push((Token::Number, s2.1 * s1.1)),
                    Token::Divide => stack.push((Token::Number, s2.1 / s1.1)),
                    _ => panic!("unexpected operator {:?}", t.0),
                }
            }
            _ => panic!("unexpected token {:?}", t.0),
        }
    }

    Some(stack.last()?.1)
}

fn main() {
    let mut buffer = String::new();
    loop {
        buffer.clear();

        let n = io::stdin()
            .read_line(&mut buffer)
            .expect("could not read line");
        match n {
            0 => break,
            _ => (),
        }

        let tokens = tokenize(buffer.as_str().trim());
        let result = compute(tokens.as_slice());
        println!("{}", result.expect("computes result"))
    }
}
