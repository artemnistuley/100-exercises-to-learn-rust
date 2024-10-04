// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let (v1, v2) = v.split_at(v.len() / 2);

    let result = thread::scope(|scope| {
        let s1 = scope.spawn(|| {
            v1.iter().sum::<i32>()
        });
        let s2 = scope.spawn(|| {
            v2.iter().sum::<i32>()
        });

        s1.join().unwrap() + s2.join().unwrap()
    });

    result
}

// let v = vec![1, 2, 3];
// let midpoint = v.len() / 2;

// std::thread::scope(|scope| {
//     scope.spawn(|| {
//         let first = &v[..midpoint];
//         println!("Here's the first half of v: {first:?}");
//     });
//     scope.spawn(|| {
//         let second = &v[midpoint..];
//         println!("Here's the second half of v: {second:?}");
//     });
// });

// println!("Here's v: {v:?}");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
