/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */

use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // get input lines as strings
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let mut lines = input.lock().lines().map(|_line| _line.ok().unwrap());
    //.collect::<Vec<String>>(); // Vector not necessary, see example solution.

    /* add code here ... */
    let totalNumbers = lines.next().unwrap().trim().parse::<usize>().unwrap();
    let mut myVec: Vec<isize> = vec![];
    let mut toOutput: isize = 0; // this will be printed as result
    for i in lines.next().unwrap().trim().split(" ") {
        myVec.push(i.parse::<isize>().unwrap());
    }
    myVec.sort(); // simple sort and the reverse on the line after to order from high to low
    myVec.reverse();
    
/*
going trough the numbers, 
(totalNumbers + 1) /2 will always go through half the numbers,
since even/2 = (even+1)/2
and if it was odd the solution should add the (odd+1)/2 numbers
*/
    for index in 0..(totalNumbers + 1) / 2 { 
        //eprintln!("adding");
        toOutput += myVec[index];
    }
    
    println!("{}", toOutput);
}
