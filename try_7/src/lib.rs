pub mod catalan_number;

use catalan_number::CatalanNumber;

pub struct Solution;

impl Solution {
    ///
    ///
    ///
    pub fn test1() {
        todo!()
    }

    /// Catalan Number Problem
    ///
    /// C(n) = C(n-1) * (4 * n - 2) / (n + 1), C(0) = 1
    pub fn catalan_number(n: usize) -> CatalanNumber {
        let mut catalan_number = vec![1];
        for idx in 1..=n {
            let tmp = idx * 4 - 2;
            catalan_number.iter_mut().for_each(|bit| {
                *bit *= tmp;
            });
            let mut carry = 0;
            catalan_number.iter_mut().for_each(|bit| {
                carry += *bit;
                *bit = carry % 10;
                carry /= 10;
            });
            while carry != 0 {
                catalan_number.push(carry % 10);
                carry /= 10;
            }
            let tmp = idx + 1;
            catalan_number.iter_mut().rev().for_each(|bit| {
                carry *= 10;
                carry += *bit;
                *bit = carry / tmp;
                carry %= tmp;
            });
            let cnt = catalan_number
                .iter()
                .rev()
                .map_while(|bit| (*bit == 0).then(|| ()))
                .count();
            catalan_number.truncate(catalan_number.len() - cnt);
        }
        catalan_number.into()
    }

    ///
    ///
    ///
    pub fn test3() {
        todo!()
    }

    /// Bash Game Problem
    ///
    ///
    pub fn bash_game(n: usize, m: usize) -> String {
        if n % (m + 1) == 0 { "second" } else { "first" }.to_string()
    }
}

#[cfg(test)]
mod tests;
