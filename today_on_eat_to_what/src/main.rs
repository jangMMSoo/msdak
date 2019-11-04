use std::fs;

fn main() {
    let contents = fs::read_to_string("../today_on_eat_to_what/food_data.json")
        .expect("Somthing went wron..read file");

    println!("with text \n{}", contents);
}
