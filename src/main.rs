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

    /* add code here ... */
    let mut garbageVar: u16 = 0; //to store the value of dist()
    let mut grid: Vec<u16> = vec![];
    for i in lines.next().unwrap().trim().split(" ") {
        grid.push(i.parse::<u16>().unwrap());
    }
    let totalrows: u16 = grid[0];
    let totalcolumns: u16 = grid[1];

    for row in (1..(totalrows + 1) / 2 + 1) {
        // counting up for half the rows and column, then counting down.
        for column in (1..(totalcolumns + 1) / 2 + 1) {
            //counting half the rows up
            garbageVar = dist(row, column);
            if garbageVar >= 10 {
                print!(".");
            } else {
                print!("{}", garbageVar.to_string());
            }
        }
        for column in (1..(totalcolumns - (totalcolumns + 1) / 2 + 1)).rev() {
            //now counting down for the res
            garbageVar = dist(row, column);
            if garbageVar >= 10 {
                print!(".");
            } else {
                print!("{}", garbageVar.to_string());
            }
        }
        print!("\n");
    }

    for row in (1..(totalrows - (totalrows + 1) / 2 + 1)).rev() {
        // now counting down
        for column in (1..(totalcolumns + 1) / 2 + 1) {
            //counting half the columns up
            garbageVar = dist(row, column);
            if garbageVar >= 10 {
                print!(".");
            } else {
                print!("{}", garbageVar.to_string());
            }
        }
        for column in (1..(totalcolumns - (totalcolumns + 1) / 2 + 1)).rev() {
            // counting down
            garbageVar = dist(row, column);
            if garbageVar >= 10 {
                //print dot in pos or print number
                print!(".");
            } else {
                print!("{}", garbageVar.to_string());
            }
        }
        print!("\n"); //go to the next row
    }
}

fn dist(a: u16, b: u16) -> u16 {
    //basically a min() func
    if a < b {
        a
    } else {
        b
    }
}
