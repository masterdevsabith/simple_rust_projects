use std::io;

fn main() {
    let mut first_var = 0;
    let mut second_var = 1;
    let mut nth_pos = String::new();

    io::stdin().read_line(&mut nth_pos).expect("failed to read");
    let nth_pos: u32 = nth_pos.trim().parse().expect("failed to parse");

    for _i in 0..nth_pos - 1 {
        let new_fibonacci = first_var + second_var;
        first_var = second_var;
        second_var = new_fibonacci;

        // println!("{} {}", first_var, second_var);
    }
    println!("The {}th fibonacci number is {}", nth_pos, second_var);
}
