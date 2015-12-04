use std::io;
use std::str::Chars;
use std::iter::Peekable;
use std::iter::Iterator;

fn get_num(chars: &mut Peekable<Chars>) -> f32 {
    let mut has_dot = false;
    let mut tens: f32 = 1.0;
    let mut num: f32 = 0.0;
    loop { 
        match chars.peek() {
            Some(&c) => {
                match c {
                    '0' ... '9' => {
                        let dig = (c as i32) - ('0' as i32); 
                        if has_dot {
                            tens = tens * 10.0;
                            num = num + dig as f32 / tens; 
                        }
                        else {
                            num = num * 10.0 + (dig as f32);
                        }
                    }
                    '.' if !has_dot => {
                        has_dot = true;
                    }
                    _ => return num
               }
           }
           _ => return num
        }
        chars.next();
    }
}

fn parse_expr(expr: &str) {
    let mut chars = expr.chars().peekable();
    loop {
        let c;
        {
            let option = chars.peek();
            if option.is_none() {
                break;
            }
            c = *option.unwrap();
        }
        match c {
            '0' ... '9' | '.' => {
                let num = get_num(&mut chars);
                println!("NUM: {}", num);
            }
            '+'|'-'|'*'|'/' => {
                println!("op: {}", c);
                chars.next();
            }
            '(' => {
                println!("left");
                chars.next();
            }
            ')' => {
                println!("right");
                chars.next();
            }
            _ => {chars.next();}
        }
    }
}

fn main() {
    println!("Please input your guess.");
    let mut expr = String::new();

    io::stdin().read_line(&mut expr)
        .ok()
        .expect("failed to read line");
       
    parse_expr(&expr);
    println!("You guessed: {}", expr);
}