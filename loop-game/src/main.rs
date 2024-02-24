use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, Rust!");
    'game_loop: loop {
        let rand_num = rand::thread_rng().gen_range(1..=100);
        loop{
            let mut num:u128 = read_number();
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
                    if read_string().trim() == "y"{
                        break 'game_loop;
                    }
                    break;
                } 
            }
        }
    }
}

fn read_number() -> u128 {
    let mut str = String::new();
    io::stdin().read_line(&mut str);
    let parse_result= str.trim().parse();
    return match parse_result {
        Ok(number) => number,
        Err(error) => {
            println!("please write a number range of (1 ~ 100)");
            return 0;
        }
    }
}

fn read_string() -> String {
    let mut str = String::new();
    io::stdin().read_line(&mut str);
    return str;
}
