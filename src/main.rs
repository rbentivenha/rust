use std::io;

fn main () {
    println!("Please the nth element of fibonnaci");

    let mut guess = String::new();
    let mut n1 = 0;
    let mut n2 = 1;
    let mut count = 1;

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess : u32 = guess.trim().parse()
        .expect("Type a number, please");

    loop {
        if count == guess {
            break;
        }
        let actual = n1 + n2;
        n1 = n2;
        n2 = actual;
        count = count + 1;
    }

    println!("Nth: {}", n2);
}