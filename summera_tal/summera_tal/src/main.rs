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
    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());
        //.collect::<Vec<String>>(); // Vector not necessary, see example solution.

    /* add code here ... */
    let totalNumbers = lines.next().unwrap().parse::<usize>().unwrap();
    let isOdd = if (totalNumbers % 2 == 1){true}else {false};
    let mut myVec:  Vec<usize> = vec![];
    let mut toOutput: usize = 0;
    //eprintln!("{isOdd}");
    for i in lines.next().unwrap().split(" "){
        myVec.push(i.parse::<usize>().unwrap());
    }
    myVec.sort();
    if totalNumbers==1{
        print!("{}", myVec[0]);
    }
    else {
    let mut loopCounter: usize = totalNumbers-1;
    if isOdd{
        while loopCounter>= (totalNumbers-1)/2{
            toOutput += myVec[loopCounter];
            loopCounter -= 1;
        }
    }
    else{
        while loopCounter>=totalNumbers/2{
            toOutput += myVec[loopCounter];
            loopCounter -= 1;
        }
    }
    print!("{}", toOutput);
}
    /* for (i, value) in myVec.iter().rev().enumerate(){
        eprintln!("{value} for index {i}");
        if (isOdd){
            if (i + 1 == (totalNumbers+1/2)){
                break;
            }
        }
        else{
            if (i == totalNumbers/2){
                break;
            }
        }
        eprintln!("adding");

        toOutput += value;
    } */
   // eprintln!("{}", toOutput);
    //println!("Print to standard output.");
}
