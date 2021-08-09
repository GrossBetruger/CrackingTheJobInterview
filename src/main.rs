mod linked_lists;
mod trees_and_graphs;
mod graphs;
mod naive_bayes;

use itertools::Itertools;
use std::collections::HashMap;

// Chapter 1 | Arrays and Strings
// 1.1 Is Unique

fn is_unique(s: String) -> bool {
    let s = s.chars().sorted().collect::<String>();
    let mut last = s.chars().next().unwrap();
    for c in s.chars().skip(1).into_iter() {
        match last == c {
            true => {return false;}
            false => {}
        }
        last = c;
    }
    true
}


// Chapter 1 | Arrays and Strings
// 1.2 Check Permutation

fn check_permutation(s1: &str, s2: &str) -> bool {
    let s1: String = s1.chars().sorted().collect();
    let s2: String = s2.chars().sorted().collect();
    s1 == s2
}

// Chapter 1 | Arrays and Strings
// 1.3 Urlify

// fn urlify(s: String) -> String {
//     let mut char_vector: Vec<char> = s.chars().collect();
//     for (i, c) in s.clone().trim().to_vec().into_iter().enumerate() {
//         if c == ' ' {
//             for (j, sub) in char_vector[i+1..].to_vec().into_iter().enumerate() {
//                 println!("{}, {}", j, sub);
//                 char_vector[i+3+j] = sub;
//             }
//             char_vector[i] = '%';
//             char_vector[i+1] = '2';
//             char_vector[i+2] = '0';
//         }
//
//         // char_vector[i+3] = original;
//
//     }
//     char_vector.into_iter().collect()
// }

// Chapter 1 | Arrays and Strings
// 1.4 Palindrome Permutation

fn palindrome_permutation(s: &str) -> bool {
    let mut map = HashMap::<char, usize>::new();
    for c in  s.chars().into_iter() {
        *map.entry(c).or_insert(0) += 1;
     }


    match s.len() % 2 == 1 {
        true => {
            map.values().filter(|x| *x % 2 == 1).count() == 1 && map.values().filter(|x| *x % 2 != 1).all(|x| x % 2 == 0)
        },
        false => {map.values().all(|x| x % 2 == 0)}
    }
}

// Chapter 1 | Arrays and Strings
// 1.5 One Away

fn one_away(s1: String, s2: String) -> bool {
    match s2.len() < s1.len() {
        true => {
            if compare_with_one_change(&s1, &s2) {
                return true;
            }
        }
        false => {
               if compare_with_one_change(&s2, &s1) {
                return true;
            }
        }
    }
    false
}

// Chapter 1 | Arrays and Strings
// 1.5 One Away
// Helper function

fn compare_with_one_change(s1: &str, s2: &str) -> bool {
    let mut mismatches: usize = 0;
    let mut idx: usize = 0;
    let s2_vec : Vec<char> = s2.chars().into_iter().collect_vec();
    let terms_equal = s1.len() == s2.len();

    for c in s1.chars().into_iter() {
        if idx == s2_vec.len() {
            return true;
        }

        if c != s2_vec[idx]{
            mismatches += 1;
            if terms_equal {
                idx += 1;
            }
        }

        else {
            idx += 1;
        }

        if mismatches > 1 {
            return false
        }
    }

    true
}

// Chapter 1 | Arrays and Strings
// 1.6 String Compression

fn string_compression(s: &str) -> String {
    let mut result: Vec<char> = vec![];
    let mut last: char = s.chars().next().unwrap();
    let mut counter: usize = 1;
    result.push(last);

    for c in s.chars().skip(1).into_iter() {

        match c == last {
            true => {
                counter += 1;
            },
            false => {
                push_counter(counter, &mut result);
                result.push(c);
                counter = 1;
                last = c;
            }
        }
    }

    push_counter(counter, &mut result);
    result.into_iter().collect()
}

// Chapter 1 | Arrays and Strings
// 1.6 String Compression
// Helper function

fn push_counter(counter: usize, char_vec: &mut Vec<char>) {
     for d in counter.to_string().chars().into_iter() {
                char_vec.push(d);
            };
}

// Chapter 1 | Arrays and Strings
// 1.7 Rotate Matrix (NXN)

fn rotate_matrix(matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>>{
    //  Rotate left

    let mut new_matrix = vec![];
    let dimension_size = matrix[0].len();

    for i in 0..dimension_size {
        let mut new_column = vec![];
        for v in matrix.clone().into_iter() {
            new_column.push(v[i])
        }
        new_matrix.push(new_column);
    }

    new_matrix.into_iter().rev().collect()
}


fn main() {
}

#[cfg(test)]
mod tests {
    use crate::{is_unique, check_permutation, palindrome_permutation, one_away, string_compression, rotate_matrix};

    #[test]
    fn test_string_uniqueness() {
        let not_unique = String::from("fcbssw");
        let unique = String::from("qwerty1234");
        assert_eq!(false, is_unique(not_unique));
        assert_eq!(true, is_unique(unique));
    }

    #[test]
    fn test_string_permutation() {
        let permut1 = "abcd";
        let permut2 = "acbd";
        let not_permut1 = "ffcc";
        let not_permut2 = "ffcd";
        assert_eq!(true, check_permutation(permut1, permut2));
        assert_eq!(false, check_permutation(not_permut1, not_permut2));
    }

    // #[test]
    // fn test_urlify() {
    //     assert_eq!(String::from("Mr%20John%20Smith"), urlify("Mr John Smith       ".into()));
    // }

    #[test]
    fn test_palindrome() {
        assert_eq!(true, palindrome_permutation("permmrep"));
        assert_eq!(true, palindrome_permutation("perm mrep"));
        assert_eq!(false, palindrome_permutation("perm xrep"));
        assert_eq!(true, palindrome_permutation("perm epmr"));
    }

    #[test]
    fn test_one_away() {
        assert_eq!(true, one_away("pale".into(), "ple".into()));
        assert_eq!(true, one_away("pales".into(), "pale".into()));
        assert_eq!(true, one_away("pale".into(), "bale".into()));
        assert_eq!(false, one_away("pale".into(), "bake".into()));
    }

    #[test]
    fn test_string_compression() {
        assert_eq!(String::from("a2b1c5a3"), string_compression("aabcccccaaa"));
    }

    #[test]
    fn test_rotate_matrix() {
        assert_eq!(vec![vec![2, 4], vec![1, 3]], rotate_matrix(vec![vec![1, 2], vec![3, 4]]));
        assert_eq!(vec![vec![3, 6, 9], vec![2, 5, 8], vec![1, 4, 7]],
                   rotate_matrix(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]))
    }
}