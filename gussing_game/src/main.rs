use rand::Rng;
use core::num;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数");

    //调用随机数生成
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("神秘数字是:{}", secret_number);
    //循环
    loop {
        //声明变量（类型根据声明进行推导）
        let mut guess = String::new();
        //读取用户输入，处理异常
        io::stdin().read_line(&mut guess).expect("无法读取");

        println!("猜数:{}", guess);
        //覆盖guss变量。处理前后空格
        // let guess: u32 = guess.trim().parse().expect("plrase type a  number");
        //处理错误输入
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //匹配模式
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
