//use std::io;
use std::str::Chars;
use std::iter::Peekable;
use std::iter::Iterator;

trait UnaryOp {
    fn calc(&self, a: f64) -> f64; 
}

trait BinOp {
    fn calc(&self, a: f64, b: f64) -> f64; 
}

macro_rules! define_op{
    ($tr:ident, $a:ident, $e:expr) => {
        struct $tr;
        impl UnaryOp for $tr { fn calc(&self, $a: f64) -> f64 { $e } }
    };
    ($tr:ident, $a:ident, $b:ident, $e:expr) => {
        struct $tr;
        impl BinOp for $tr { fn calc(&self, $a: f64, $b: f64) -> f64 { $e } }
    }
}

enum OpTypes {
    ForUnary(Box<UnaryOp>),
    ForBin(Box<BinOp>),
}

struct OpRec {
    op: OpTypes,
    pre: i32,
}

impl OpRec {
    fn new_unary<T>(op: T, pre: i32) -> OpRec where T: UnaryOp + 'static{
       OpRec { op: OpTypes::ForUnary(Box::new(op)), pre: pre }    
    }

    fn new_bin<T>(op: T, pre: i32) -> OpRec where T: BinOp + 'static{
       OpRec { op: OpTypes::ForBin(Box::new(op)), pre: pre }    
    }
}

define_op!(PosOp, a, a);
define_op!(NegOp, a, -a);
define_op!(AddOp, a, b, a + b);
define_op!(SubOp, a, b, a - b);
define_op!(MulOp, a, b, a * b);
define_op!(DivOp, a, b, a / b);

fn get_num(chars: &mut Peekable<Chars>) -> f64 {
    let mut has_dot = false;
    let mut tens: f64 = 1.0;
    let mut num: f64 = 0.0;
    let zero_char = '0' as i32;
    loop { 
        match chars.peek() {
            Some(&c) => {
                match c {
                    '0' ... '9' => {
                        let dig = ((c as i32) - zero_char) as f64; 
                        if has_dot {
                            tens = tens * 10.0;
                            num = num + dig / tens; 
                        }
                        else {
                            num = num * 10.0 + dig;
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

fn get_unary_op(c: char, cur_pre: i32) -> Option<OpRec> {
    match c {
        '+' => Some(OpRec::new_unary(PosOp, cur_pre + 10)),
        '-' => Some(OpRec::new_unary(NegOp, cur_pre + 10)),
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
    num_stack: Vec<f64>,
}

impl ExprParser {
    fn new() -> ExprParser {
        ExprParser {
            op_stack: Vec::new(), 
            num_stack: Vec::new(),
        }
    }
    
    fn calc_top_op(&mut self, pre: i32) -> bool {
        let op_rec = self.op_stack.last().unwrap();
        let op_pre = op_rec.pre;
        match op_rec.op {
            OpTypes::ForUnary(ref op) => {
                if op_pre <= pre {
                    return false;
                }
                if self.num_stack.len() < 1 {
                    return false;
                }
                let a = self.num_stack.pop().unwrap();
                self.num_stack.push(op.calc(a));
            }
            OpTypes::ForBin(ref op) => {
                if op_pre < pre {
                    return false;
                }
                if self.num_stack.len() < 2 {
                    return false;
                }
                let b = self.num_stack.pop().unwrap();
                let a = self.num_stack.pop().unwrap();
                self.num_stack.push(op.calc(a, b));
            }
        }
        return true;
    }

    fn pop_op(&mut self, pre: i32) {
        while self.op_stack.len() > 0 {
            println!("{:?}", self.num_stack);
            if !self.calc_top_op(pre) {
                return;
            }
            self.op_stack.pop();
        }
    } 
    
    fn parse(&mut self, expr: &str) -> Result<f64, String>{
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
                            self.pop_op(op.pre);
                            self.op_stack.push(op);
                        }
                        None => {
                            return Err(format!("expect binary op but got: {}", c));
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
                        match get_unary_op(c, cur_pre) {
                            Some(op) => {
                                println!("UNI OP: {}, {}", c, op.pre);
                                self.pop_op(op.pre);
                                self.op_stack.push(op);
                                chars.next();
                                continue;
                            }
                            None => { return Err(format!("unexpect {}", c)); }
                        }
                    }
                }
            }
        }
        self.pop_op(0);
        println!("{:?}", self.num_stack);
        if self.op_stack.len() > 0 {
            return Err(format!("lack of number"));
        }
        if self.num_stack.len() != 1 {
            return Err(format!("lack of operator"));
        }
        return Ok(self.num_stack[0]);
    }
}

fn test(expr: &str) {
    let mut parser = ExprParser::new();
    print!("{}=", expr);
    match parser.parse(expr) {
        Ok(r) => {
            println!("{}", r);
        },
        Err(s) => {
            println!("{}", s);
        }
    }
}

fn main() {
    println!("Please input your guess.");
    
//    let mut expr = String::new();
//    io::stdin().read_line(&mut expr)
//        .ok()
//        .expect("failed to read line");
    test("((12-2)");
    test("23.4 + 32 * 5+");       
    test("1++++----212 67");  
//    test("2*(2-(9.3-2.3))");     
//    test("1+2*--3.67 * (5.34 - 2)/+.5");       
}