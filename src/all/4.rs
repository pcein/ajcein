/* These lines are required to facilitate
 * the score submission process. Don't worry
 * about them!
 */
#![allow(dead_code, unused_imports)]

mod submit;
mod myconfig;

extern crate hyper;

use submit::{Question, Score};

/*--------------------------------------------*/

fn greater2(a: i32, b: i32) -> i32 {
 
}

fn greater3(a: i32, b: i32, c: i32) -> i32 {

}

fn main() {

    assert_eq!(greater3(1, 2, 3), 3);
    assert_eq!(greater3(5, 9, 4), 9);
    assert_eq!(greater3(10, 8, 9), 10);
    assert_eq!(greater3(10, 10, 10), 10);
    assert_eq!(greater3(10, 10, 11), 11);
 
    println!("\nAll tests passed!!\n");   
    // submit::submit(Question(4), Score(5));
}
