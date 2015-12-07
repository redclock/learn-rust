//use std::io;
use std::str::Chars;
use std::iter::Peekable;
use std::iter::Iterator;

trait UniOp {
    fn calc(&self, a: f32) -> f32; 
}

trait BinOp {
    fn calc(&self, a: f32, b: f32) -> f32; 
}

struct AddOp;
impl BinOp for AddOp { fn calc(&self, a: f32, b: f32) -> f32 { a + b } }

struct SubOp;
impl BinOp for SubOp { fn calc(&self, a: f32, b: f32) -> f32 { a - b } }

struct MulOp;
impl BinOp for MulOp { fn calc(&self, a: f32, b: f32) -> f32 { a * b } }

struct DivOp;
impl BinOp for DivOp { fn calc(&self, a: f32, b: f32) -> f32 { a / b } }

struct PosOp;
impl UniOp for PosOp { fn calc(&self, a: f32) -> f32 { a } }

struct NegOp;
impl UniOp for NegOp { fn calc(&self, a: f32) -> f32 { -a } }

enum OpTypes {
    ForOne(Box<UniOp>),
    ForTwo(Box<BinOp>),
}

struct OpRec {
    op: OpTypes,
    pre: i32,
}

impl OpRec {
    fn new_uni<T>(op: T, pre: i32) -> OpRec where T: UniOp + 'static{
       OpRec { op: OpTypes::ForOne(Box::new(op)), pre: pre }    
    }

    fn new_bin<T>(op: T, pre: i32) -> OpRec where T: BinOp + 'static{
       OpRec { op: OpTypes::ForTwo(Box::new(op)), pre: pre }    
    }
}

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

fn get_uni_op(c: char, cur_pre: i32) -> Option<OpRec> {
    match c {
        '+' => Some(OpRec::new_uni(PosOp, cur_pre + 10)),
        '-' => Some(OpRec::new_uni(NegOp, cur_pre + 10)),
        _ => None
    }
}

fn get_bin_op(c: char, cur_pre: i32) -> Option<OpRec> {
    match c {
        '+' => Some(OpRec::new_bin(AddOp, cur_pre + 1)),
        '-' => Some(OpRec::new_bin(SubOp, cur_pre + 1)),
        '*' => Some(OpRec::new_bin(MulOp, cur_pre + 2)),
        '/' => Some(OpRec::new_bin(DivOp, cur_pre + 2)),
        _ => None
    }
}

struct ExprParser {
    op_stack: Vec<OpRec>,
    num_stack: Vec<f32>,
}

impl ExprParser {
    fn new() -> ExprParser {
        ExprParser {
            op_stack: Vec::new(), 
            num_stack: Vec::new(),
        }
    }
    
    fn pop_op(&mut self, pre: i32, left_to_right: bool) {
        while self.op_stack.len() > 0 {
            {
                let op_rec = self.op_stack.last().unwrap();
                let should_return = 
                    if left_to_right { op_rec.pre < pre } 
                    else { op_rec.pre <= pre };
                if should_return {
                    return;
                }
            }
            let op = self.op_stack.pop().unwrap().op;
            println!("{:?}", self.num_stack);

            match op {
                OpTypes::ForOne(ref op) => {
                    let a = self.num_stack.pop().unwrap();
                    self.num_stack.push(op.calc(a));
                }
                OpTypes::ForTwo(ref op) => {
                    let b = self.num_stack.pop().unwrap();
                    let a = self.num_stack.pop().unwrap();
                    self.num_stack.push(op.calc(a, b));
                }
            }
        }
    } 
    
    fn parse(&mut self, expr: &str) {
        let mut cur_pre = 0;
        let mut chars = expr.chars().peekable();
        let mut is_expect_bin_op = false;
        
        loop {
            let c;
            {
                let option = chars.peek();
                if option.is_none() {
                    break;
                }
                c = *option.unwrap();
            }
    
            if c.is_whitespace() {
                chars.next();
                continue;
            }
    
            if is_expect_bin_op {
                if c == ')' {
                    println!("right");
                    cur_pre -= 100;
                }
                else {
                    match get_bin_op(c, cur_pre) {
                        Some(op) => {
                            println!("BIN OP: {}, {}", c, op.pre);
                            self.pop_op(op.pre, true);
                            self.op_stack.push(op);
                        }
                        None => {
                            println!("expect binary op but got: {}", c);
                        }
                    }
                    is_expect_bin_op = false;
                }
                chars.next();
            }
            else {
                match c {
                    '0' ... '9' | '.' => {
                        let num = get_num(&mut chars);
                        self.num_stack.push(num);
                        println!("NUM: {}", num);
                        is_expect_bin_op = true;
                    }
                    '(' => {
                        println!("left");
                        cur_pre += 100;
                        chars.next();
                    }
                    _ => {
                        match get_uni_op(c, cur_pre) {
                            Some(op) => {
                                println!("UNI OP: {}, {}", c, op.pre);
                                self.pop_op(op.pre, false);
                                self.op_stack.push(op);
                                chars.next();
                                continue;
                            }
                            None => { println!("unexpect {}", c); }
                        }
                    }
                }
            }
        }
        self.pop_op(0, true);
        println!("{:?}", self.num_stack);
    }
}

fn main() {
    println!("Please input your guess.");
    
//    let mut expr = String::new();
//    io::stdin().read_line(&mut expr)
//        .ok()
//        .expect("failed to read line");
       
    let mut parser = ExprParser::new();
    parser.parse("1+2*--3.67 * (5.34 - 2)/+.5");
}