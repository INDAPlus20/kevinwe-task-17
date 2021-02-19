/***
 *  Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */

use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();
    let mut firstline = true;
    let mut shows = 0;
    let mut recorders = 0;
    let mut showlist = vec![];

    // or loop all input lines,
    for _line in input.lock().lines().map(|_line| _line.unwrap()) {
        if firstline == true {
            shows = _line.chars().nth(0).unwrap() as usize;
            recorders = _line.chars().nth(2).unwrap() as usize;
            firstline = false;
        }
        if firstline == false {
            let start = _line.chars().nth(0).unwrap() as usize;
            let end = _line.chars().nth(2).unwrap() as usize;
            showlist.push((start, end))
        }
    }

}