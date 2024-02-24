
pub mod stdin_util {
    use std::io;
    pub fn read_u128_number(error_message: &str) -> u128 {
        let mut read_str = String::new();
        let _ = io::stdin().read_line(&mut read_str);
        return match read_str.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("{}",error_message);
                return 0;
            }
        }

    }

    pub fn read_string() -> String {
        let mut string = String::new();
        let _ = io::stdin().read_line(&mut string)
            .expect("Can Not Read String in Standard I/O");

        return string;
    }
}
