#![feature(test)]
extern crate test;

use itertools::{self, Itertools};

use std::collections::{hash_map::RandomState, BTreeSet, HashSet};

/*
test tests::bench_p1_btree     ... bench:     146,111 ns/iter
test tests::bench_p1_hash      ... bench:     161,523 ns/iter
test tests::bench_p1_itertools ... bench:     169,472 ns/iter
test tests::bench_p1_skipping  ... bench:       1,152 ns/iter

test tests::bench_p2_btree     ... bench:     861,028 ns/iter
test tests::bench_p2_hash      ... bench:     692,197 ns/iter
test tests::bench_p2_itertools ... bench:     319,863 ns/iter
test tests::bench_p2_skipping  ... bench:       1,147 ns/iter
*/
fn main() {
    let p1 = p_skipping(REAL, 4);
    let p2 = p_skipping(REAL, 14);
    println!("{} {}", p1, p2);
}

fn p_btree(s: &str, n: usize) -> usize {
    s.as_bytes()
        .windows(n)
        .enumerate()
        .find(|(_, window)| BTreeSet::from_iter(window.iter()).len() == window.len())
        .unwrap()
        .0
        + n
}

fn p_hash(s: &str, n: usize) -> usize {
    s.as_bytes()
        .windows(n)
        .enumerate()
        .find(|(_, window)| {
            HashSet::<_, RandomState>::from_iter(window.iter()).len() == window.len()
        })
        .unwrap()
        .0
        + n
}

fn p_itertools(s: &str, n: usize) -> usize {
    s.as_bytes()
        .windows(n)
        .enumerate()
        .find(|(_, window)| window.iter().all_unique())
        .unwrap()
        .0
        + n
}

// in theory to make this nice we'd what need 1) an iterator that does combinations
// that the j/k loops are doing, and 2) some kind of iterator adapter that lets the inner
// fn choose how many elements to skip, a la the outer loop
//
// todo: also instead of just doing n^2 searches, should still use a hash map here
fn p_skipping(s: &str, n: usize) -> usize {
    let s = s.as_bytes();
    let mut i = 0;
    'outer: while i + n < s.len() {
        let window = &s[i..i + n];
        // we want to find the furthest most back dupe
        // I really tried to find something better than just a double for-loop
        // but everything I tried was 100x slower, wtf
        for j in (1..n).rev() {
            let wj = window[j];
            for k in (0..j).rev() {
                if wj == window[k] {
                    // we found a dupe at indexes (j,k) so we know that
                    // the next k windows must also have a dupe, so we skip them
                    i += k + 1;
                    continue 'outer;
                }
            }
        }
        // we found no dupe, so we're done
        return i + n;
    }
    unreachable!()
}

fn p_skipping_a(s: &str, n: usize) -> usize {
    let s = s.as_bytes();
    let mut i = 0;
    'outer: while i + n < s.len() {
        // we want to find the furthest most back dupe
        // I really tried to find something better than just a double for-loop
        // but everything I tried was 100x slower, wtf
        for j in (1..n).rev() {
            let wj = s[i + j];
            for k in (0..j).rev() {
                if wj == s[i + k] {
                    // we found a dupe at indexes (j,k) so we know that
                    // the next k windows must also have a dupe, so we skip them
                    i += k + 1;
                    continue 'outer;
                }
            }
        }
        // we found no dupe, so we're done
        return i + n;
    }
    unreachable!()
}
// test tests::bench_p2_skipping_2 ... bench:      18,444 ns/iter
// hashset even worse
fn p_skipping_2(s: &str, n: usize) -> usize {
    let s = s.as_bytes();
    let mut i = 0;
    'outer: while i + n < s.len() {
        let window = &s[i..i + n];
        // we want to find the furthest most back dupe
        let mut hs = BTreeSet::new();
        hs.insert(window[n - 1]);
        for j in (0..n - 1).rev() {
            if !hs.insert(window[j]) {
                i += j + 1;
                continue 'outer;
            }
        }
        // we found no dupe, so we're done
        return i + n;
    }
    unreachable!()
}

// test tests::bench_p2_skipping_3 ... bench:      23,349 ns/iter
fn p_skipping_3(s: &str, n: usize) -> usize {
    SkippingIter::new(s.as_bytes().windows(n).enumerate().map(|(i, window)| {
        for j in (1..n).rev() {
            let wj = window[j];
            for k in (0..j).rev() {
                if wj == window[k] {
                    // we found a dupe at indexes (j,k) so we know that
                    // the next k windows must also have a dupe, so we skip them
                    return Err(k);
                }
            }
        }
        // we found no dupe, so we're done
        return Ok(i + n);
    }))
    .next()
    .unwrap()
}
struct SkippingIter<I>(I, usize);
impl<I> SkippingIter<I> {
    fn new(i: I) -> Self {
        SkippingIter(i, 0)
    }
}
impl<N, I: Iterator<Item = Result<N, usize>>> Iterator for SkippingIter<I> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = if self.1 > 0 {
                self.0.nth(std::mem::take(&mut self.1) - 1)
            } else {
                self.0.next()
            };
            match next {
                Some(Ok(x)) => return Some(x),
                Some(Err(skip)) => {
                    self.1 = skip;
                    continue;
                }
                None => return None,
            }
        }
    }
}

const TEST1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
const TEST2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
const TEST3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
const TEST4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
const TEST5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
const REAL: &str = include_str!("real.txt");


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn p1() {
        assert_eq!(p_skipping(TEST1, 4), 7);
        assert_eq!(p_skipping(TEST2, 4), 5);
        assert_eq!(p_skipping(TEST3, 4), 6);
        assert_eq!(p_skipping(TEST4, 4), 10);
        assert_eq!(p_skipping(TEST5, 4), 11);

        assert_eq!(p_skipping_2(TEST1, 4), 7);
        assert_eq!(p_skipping_2(TEST2, 4), 5);
        assert_eq!(p_skipping_2(TEST3, 4), 6);
        assert_eq!(p_skipping_2(TEST4, 4), 10);
        assert_eq!(p_skipping_2(TEST5, 4), 11);

        assert_eq!(p_skipping_3(TEST1, 4), 7);
        assert_eq!(p_skipping_3(TEST2, 4), 5);
        assert_eq!(p_skipping_3(TEST3, 4), 6);
        assert_eq!(p_skipping_3(TEST4, 4), 10);
        assert_eq!(p_skipping_3(TEST5, 4), 11);
    }

    // #[bench]
    // fn bench_p1_btree(b: &mut Bencher) {
    //     b.iter(|| p_btree(REAL, 4));
    // }

    // #[bench]
    // fn bench_p2_btree(b: &mut Bencher) {
    //     b.iter(|| p_btree(REAL, 14));
    // }

    // #[bench]
    // fn bench_p1_hash(b: &mut Bencher) {
    //     b.iter(|| p_hash(REAL, 4));
    // }

    // #[bench]
    // fn bench_p2_hash(b: &mut Bencher) {
    //     b.iter(|| p_hash(REAL, 14));
    // }

    // #[bench]
    // fn bench_p1_itertools(b: &mut Bencher) {
    //     b.iter(|| p_itertools(REAL, 4));
    // }

    // #[bench]
    // fn bench_p2_itertools(b: &mut Bencher) {
    //     b.iter(|| p_itertools(REAL, 14));
    // }

    #[bench]
    fn bench_p1_skipping(b: &mut Bencher) {
        b.iter(|| p_skipping(REAL, 4));
    }

    #[bench]
    fn bench_p2_skipping(b: &mut Bencher) {
        b.iter(|| p_skipping(REAL, 14));
    }

    #[bench]
    fn bench_p1_skipping_2(b: &mut Bencher) {
        b.iter(|| p_skipping_2(REAL, 4));
    }

    #[bench]
    fn bench_p2_skipping_2(b: &mut Bencher) {
        b.iter(|| p_skipping_2(REAL, 14));
    }

    #[bench]
    fn bench_p1_skipping_3(b: &mut Bencher) {
        b.iter(|| p_skipping_3(REAL, 4));
    }

    #[bench]
    fn bench_p2_skipping_3(b: &mut Bencher) {
        b.iter(|| p_skipping_3(REAL, 14));
    }
    #[bench]

    fn bench_p2_skipping_a(b: &mut Bencher) {
        b.iter(|| p_skipping_a(REAL, 14));
    }
}
