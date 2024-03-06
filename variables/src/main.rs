use std::u32;

fn main() {
    println!("Hello, world!");

    let tup: (i32,f64) = (200,8.2);

    println!("{},{}",tup.0,tup.1);
    print_lct(7,9);
    comper_num(6);
    comper_nummatch(6);
}

fn print_lct(c: i32,b: u32) {
    println!("c:{} b:{}",c,b);
}


//if else
fn comper_num(number:i32) {
    if number % 2 == 0 {
        println!("Verif{}",2);
    }else if number % 3 == 0 {
        println!("Verif{}",3);
    }else {
        println!("No");
    }
}

//match重构
fn comper_nummatch(number:i32) {
    match number {
        n if n % 2 == 0 => println!("Verif{}",2),
        n if n % 3 == 0 => println!("Verif{}",2),
        _ => println!("No"),
    }
}
