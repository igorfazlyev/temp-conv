use std::io;

fn main() {
    let mut user_input = String::new();
    println!("enter the temperature");
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
   
    let temp:f32 = user_input.trim().parse().expect("Please type a number!");

    println!("Enter 0 to convert from f to c or 1 to convert from c to f: ");
    user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let choice: u32 = user_input.trim().parse().expect("Please type a number!");
    if choice == 0 {
        println!("the temp in C is {}", f_to_c(temp));
    }else if choice == 1 {
        println!("the temp in F is {}", c_to_f(temp));
    }else{
        println!("unknown choice");
    }



}

fn f_to_c(temp:f32) -> f32 {
    return (temp - 32.0) * 5.0 /9.0;
}

fn c_to_f(temp:f32) -> f32 {
    return temp * 9.0/5.0 + 32.0;
}