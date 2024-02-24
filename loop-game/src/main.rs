use rand::Rng;
use std::cmp::Ordering;

mod utils;
use utils::stdin_util;

fn main() {
    println!("Hello, Rust!");
    'game_loop: loop {
        let rand_num = rand::thread_rng().gen_range(1..=100);
        loop{
            let num:u128 = stdin_util::read_u128_number("include 1 ~ 100 number");
            if num == 0 {
                continue;
            }
            println!("rand Number : {}", rand_num);
            match num.cmp(&rand_num){
                Ordering::Less => println!("Too Small"),
                Ordering::Greater => println!("Too Greater"),
                Ordering::Equal => {
                    println!("Equal");
                    println!("Continue? if do not want continue the game please enter the 'y'");
                    if stdin_util::read_string().trim() == "y"{
                        break 'game_loop;
                    }
                    break;
                } 
            }
        }
    }
}

