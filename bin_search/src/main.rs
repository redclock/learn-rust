fn search(array: &mut Vec<i32>, target: i32)->i32{
    array.sort();
    println!("{:?}, {}", *array, target);
    let mut start = 0;
    let mut end = array.len();
    while start < end {
        let mid = (start + end) / 2;
        //println!("{}", mid);
        let x = array[mid];
        if target < x {
            end = mid;
        }
        else if target > x {
            start = mid + 1;
        }
        else {
            return mid as i32;
        }
    }
    return -1;
}

fn main() {
    println!("{}", search(&mut vec![1, 2, 3, 4, 5, 7, 10, 20], 0));
    println!("{}", search(&mut vec![1, 2, 3, 4, 5, 7, 10, 20], 1));
    println!("{}", search(&mut vec![1, 2, 3, 4, 5, 7, 10, 20], 2));
    println!("{}", search(&mut vec![1, 2, 3, 4, 5, 7, 10, 20], 3));
    println!("{}", search(&mut vec![1, 2, 3, 4, 5, 7, 10, 20], 4));
    println!("{}", search(&mut vec![1, 2, 3, 4, 5, 7, 10, 20], 5));
    println!("{}", search(&mut vec![1, 2, 3, 4, 5, 7, 10, 20], 6));
    println!("{}", search(&mut vec![1, 2, 3, 4, 5, 7, 10, 20], 7));
    println!("{}", search(&mut vec![1, 2, 3, 4, 5, 7, 10, 20], 10));
    println!("{}", search(&mut vec![1, 2, 3, 4, 5, 7, 10, 20], 20));
    println!("{}", search(&mut vec![1, 2, 3, 4, 5, 7, 10, 20], 30));
}
