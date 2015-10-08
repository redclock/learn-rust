fn test_int_inmutable() {
    let x = 10;
    let y = x;
    let z = &y;
    println!("x = {}, y = {}, z = {}", x, y, z);
}

fn test_int_mutable() {
    let mut x = 10;
    let mut y = 20;
    println!("x = {}, y = {}", x, y);
    let mut z = &mut x;
    println!("z = {}", z);
    z = &mut y;
    println!("z = {}", z);
}

fn test_int_mutable_borrow() {
    let mut x = 10;
    {
        let y = &mut x;
        *y = 20;
    }
    println!("x = {}", x);
}

fn main() {
    test_int_inmutable();
    test_int_mutable();
    test_int_mutable_borrow();
}
