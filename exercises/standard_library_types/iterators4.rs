// iterators4.rs


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
// recursion
//     if num == 0 {
//         1
//     } else if num == 1 || num == 2 {
//         num
//     } else {
//         num * factorial(num-1) 
//     }
//
//  product
//  (1..num+1).product()
//  (1..=num).product()
//
//  fold
//  (1..=num).fold(1, |acc, n| acc * n) 
//
//  iter
//    (1..=num).collect::<Vec<u64>>()
//        .iter()
//        .fold(1, |acc, n| acc * n)
//  match
    match num {
        0 | 1 => 1,
        _ => num * factorial(num-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
