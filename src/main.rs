#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("v {:?}", v)
}

fn pop_items_from_vec(num_items: i32, mut vector: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    while i < num_items {
        vector.pop();
        i = i + 1;
    }
    vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_pop_two_items() {
        let v = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(pop_items_from_vec(2, v).len(), 4);
    }

    #[test]
    fn case_pop_three_items() {
        let v = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(pop_items_from_vec(3, v).len(), 3);
    }
}

// assert_eq!(a, b)?
// does a == b

// use std::collections::VecDeque;

// fn main() {
//     let v2 = vec![1, 2, 3];
//     let v3: Vec<&str> = vec!["a", "b", "c"];

// }

// fn create_new_vec() {
//     let v1: Vec<i32> = Vec::new();
//     // OR
//     let v2: Vec<i32> = vec![];
// }

// // create a function to print the items of i32 in a vector
// fn print_vec(items: Vec<i32>) {
//     for i in items {
//         println!("{}", i)
//     }
// }

// fn push_to_vec(mut initial_items: Vec<i32>, items_to_add: Vec<i32>) -> Vec<i32> {
//     for i in items_to_add {
//         initial_items.push(i)
//     }
//     initial_items
// }

// // pop_from_vec
// // Step 1. pops all items from vector
// // Step 2. pops num_remove from vector

// fn pop_from_vec(num_remove: i32, mut items: Vec<i32>) {
//     let mut len = items.len();

//     while len > 0 {
//         let p = items.pop();
//         len = len - 1;
//     }
// }

// // iterate over a vector
// fn iterate_vec() {
//     let vec = vec![1, 2, 3, 4];
//     for x in vec.iter() {
//         println!("vec contained {}", x);
//     }
// }

// fn iterate_mut_vec() {
//     let mut vec = vec![1, 2, 3, 4];
//     for x in vec.iter_mut() {
//         *x += 1;
//     }
// }

// fn extend_vec() {
//     let mut vec1 = vec![1, 2, 3, 4];
//     let vec2 = vec![10, 20, 30, 40];
//     println!("vec2 {:?}", vec2);
//     vec1.extend(vec2);
//     println!("vec1 {:?}", vec1);
// }

// // Don't understand this one yet
// fn what_is_deque() {
//     let vec = vec![1, 2, 3, 4];
//     let buf: VecDeque<_> = vec.into_iter().collect();
//     println!("buf {:?}", buf)
// }

// // Reverse a Vec
// fn reverse_vec() {
//     let vec = vec![1, 2, 3, 4];
//     for x in vec.iter().rev() {
//        println!("vec contained {}", x);
//     }
// }
