mod core;
mod error;
use crate::core::*;
use crate::error::*;

fn main() {
    match engine() {
        Err(error) => exit(error),
        Ok((head, tail, result)) => {
            let (h, t) = solution(head, tail, result);
            for i in 0..h.len() {
                println!("{}{}", h[i], t[i]);
            }
        }
    }
}
