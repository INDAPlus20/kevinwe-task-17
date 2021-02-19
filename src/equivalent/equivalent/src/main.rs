/***
 *  Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */

use std::io;
use std::io::prelude::*;
struct Statement{
    proven: bool,
    proves: bool,
}
/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();
    let mut caseamount = 0;
    let mut caselines = 0;
    let mut cases = vec![];
    let mut curvec = vec![];
    // or loop all input lines,
    for _line in input.lock().lines().map(|_line| _line.unwrap()) {
        if _line.len() > 1{
            if caselines == 0{
                cases.push((curvec, caseamount));
                curvec = vec![];
                caseamount = _line.chars().nth(0).unwrap() as usize;
                caselines = _line.chars().nth(2).unwrap() as usize;
            }
            else{
                let num1 = _line.chars().nth(0).unwrap() as usize;
                let num2 = _line.chars().nth(2).unwrap() as usize;
                curvec.push((num1, num2));
                caselines -= 1;
            }
        }
    }
    // remove off-by-one
    cases.remove(0);
    //currently only proves subsets of the statements
    for case in cases{
        let mut statements = vec![];
        for statement in 1..case.1 {statements.push(Statement { proven: false,proves: false})}
        for implication in case.0{
            statements[implication.0].proves = true;
            statements[implication.1].proven = true;
        }
        let mut proven_need = 0;
        let mut proves_need = 0;
        for statement in statements{
            if statement.proven == false {proven_need += 1}
            if statement.proves == false {proves_need += 1}
        }
        if proven_need > proves_need{println!("{}", proven_need)}
        else {println!("{}", proves_need);}
    }
}