// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


// 3: use Rc & RefCell ... actually use clone(), just show Rc usage.
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let vec0 = Vec::new();

    let v0 = Rc::new(RefCell::new(vec0.clone()));

    let mut vec1 = fill_vec(v0.clone().borrow().to_vec());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}


// 1: use mutable
// fn main() {
//     let mut vec0 = Vec::new();
// 
//     let mut vec1 = fill_vec(&mut vec0);
// 
//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
// 
//     vec1.push(88);
// 
//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }
// 
// fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
// 
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);
// 
//     vec.to_vec()
// }

// 2: use clone
// fn main() {
//     let vec0 = Vec::new();
// 
//     let mut vec1 = fill_vec(vec0.clone());
// 
//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
// 
//     vec1.push(88);
// 
//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }
// 
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;
// 
//     vec.push(22);
//     vec.push(44);
//     vec.push(66);
// 
//     vec
// }


