fn main() {
    println!("Hello, world!");

    let condition  = true;
    let number = if condition {
        5
    }else {
        6
    };

    println!("number is {}", number);

    let mut counter = 0;
    let result = loop {

        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number2 = 3;

    while number2 != 0 {
        println!("{}!", number2);
        number2 -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is :{}", a[index]);
        index += 1;

    }


    for el in a.iter(){
        println!("the value is {}", el);
    }


    for n in (1..4).rev() {
        println!("{}!", n);
    }


}
