#![feature(stmt_expr_attributes)]

use std::os::raw::c_int;

extern "C" {
    // random number generator from C standard library.
    fn rand() -> c_int;
}

/// Chip structure
///
/// tuple.0: chip's number
/// tuple.1: is it good/bad?
#[derive(Debug)]
struct Chip(usize, bool);

impl Chip {
    fn test(&self, other: &Chip) -> bool {
        if self.1 {
            other.1
        } else {
            // bad isn't reliable
            unsafe { rand() % 2 == 1 }
        }
    }
}

fn main() {
    #[rustfmt::skip]
    // 17 chips
    let mut chips = [
        true, false, false, true, true,
        true, false, false, true, true,
        true, true, false, true, false,
        false, false,
    ]
    .into_iter()
    .enumerate()
    .map(|(index, state)| Chip(index, state))
    .collect::<Vec<_>>();
    while chips.len() > 3 {
        let mut index = 0;
        while index + 1 < chips.len() {
            #[rustfmt::skip]
            if chips[index].test(&chips[index + 1])
                && chips[index + 1].test(&chips[index]) {
                chips.swap_remove(index + 1);
            } else {
                chips.swap_remove(index + 1);
                chips.swap_remove(index);
            }
            index += 2;
        }
    }
    if chips.len() == 3 {
        #[rustfmt::skip]
        if chips[0].test(&chips[1])
            && chips[1].test(&chips[0]) {
            chips.swap_remove(2);
        } else {
            chips.swap_remove(1);
            chips.swap_remove(0);
        }
    } else {
        chips.swap_remove(0);
    }
    println!("{chips:?}");
}
