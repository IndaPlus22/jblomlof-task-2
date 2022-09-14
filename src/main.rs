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
    let mut garbageVar: u16 = 0;
    let mut grid: Vec<u16> = vec![]; 
    for i in lines.next().unwrap().split(" "){
        grid.push(i.parse::<u16>().unwrap());
    }
    let totalrows :u16 = grid[0];
    let totalcolumns :u16 = grid[1];
//println!("{}",totalcolumns);
//println!("{}",totalrows);
    for row in (1..(totalrows+1)/2+1){ // counting up for half the rows and column, then counting down.
        for column in (1..(totalcolumns+1)/2+1){
            garbageVar = dist(row,column);
            if garbageVar>=10{
                print!(".");
            }
            else {
                print!("{}",garbageVar.to_string());
            }
        }
        for column in (1..(totalcolumns-(totalcolumns+1)/2 + 1)).rev(){
            garbageVar = dist(row,column);
            if garbageVar>=10{
                print!(".");
            }
            else {
                print!("{}",garbageVar.to_string());
            }
        }
        print!("\n");
    }
    for row in (1..(totalrows-(totalrows+1)/2 + 1)).rev(){
        for column in (1..(totalcolumns+1)/2+1){
            garbageVar = dist(row,column);
            if garbageVar>=10{
                print!(".");
            }
            else {
                print!("{}",garbageVar.to_string());
            }
        }
        for column in (1..(totalcolumns-(totalcolumns+1)/2 + 1)).rev(){
            garbageVar = dist(row,column);
            if garbageVar>=10{
                print!(".");
            }
            else {
                print!("{}",garbageVar.to_string());
            }
        }
        print!("\n");
    }
    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output
}

fn dist(a:u16, b:u16) -> u16{
    if a<b{
        if a>=10 {return 10;}
    a
    }
    else {
        if b>=10 {return 10;}
    b
    }
}