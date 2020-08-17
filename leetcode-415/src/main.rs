use std::io;

fn add_strings(num1: String, num2: String) -> String {
    let mut num1 = num1;
    let mut num2 = num2;
    let mut result = String::new();

    let mut carry: u32 = 0;
    while !num1.is_empty() || !num2.is_empty() || carry != 0 {
        let mut add: u32 = carry;
        if let Some(n1) = num1.pop() {
            add += n1.to_digit(10).unwrap();
        }

        if let Some(n2) = num2.pop() {
            add += n2.to_digit(10).unwrap();
        }
        carry = add / 10;
        add -= carry * 10;
        result.push_str(&add.to_string());
    }

    return result.chars().rev().collect::<String>();
}

fn main() {
    let mut string1 = String::new();
    let mut string2 = String::new();
    io::stdin()
        .read_line(&mut string1)
        .expect("faile to read line");
    io::stdin()
        .read_line(&mut string2)
        .expect("faile to read line");

    println!(
        "{} + {} = {}",
        string1,
        string2,
        add_strings(string1.trim().to_string(), string2.trim().to_string())
    );
}
