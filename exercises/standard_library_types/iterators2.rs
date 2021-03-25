// iterators2.rs
// In this module, you'll learn some of the unique advantages that iterators can offer.
// Step 1. Complete the `capitalize_first` function to pass the first two cases.
// Step 2. Apply the `capitalize_first` function to a vector of strings.
//         Ensure that it returns a vector of strings as well.
// Step 3. Apply the `capitalize_first` function again to a list.
//         Try to ensure it returns a single string.
// As always, there are hints if you execute `rustlings hint iterators2`!


pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

pub fn capitalize_first_vec(mut vec: Vec<&str>) -> Vec<String> {
    let mut res = Vec::new();
    for i in vec.iter_mut() {
        res.push(capitalize_first(*i))
    }
    res
}

pub fn capitalize_first_vec_into_string(vec: Vec<&str>) -> String {
    let mut res = String::new();
    for i in capitalize_first_vec(vec).iter_mut() {
        res = res + i;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = capitalize_first_vec(words); // TODO
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let capitalized_words = capitalize_first_vec_into_string(words); // TODO
        assert_eq!(capitalized_words, "Hello World");
    }
}
