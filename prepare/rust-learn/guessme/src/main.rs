/*
 * ===============================================
 * author: goodfanqie.
 * description : a simple guess game to learn rust.
 * learn from: https://kaisery.github.io/trpl-zh-cn/ch02-00-guessing-game-tutorial.html?search=
 * ===============================================
 */  

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {


    println!("Guess the number!");


    let secret_number = rand::thread_rng().gen_range(1..=100);
    // 在rust中 trait是一个很重要的概念，Rng就是一个trait，
    // 定义了随机数生成器实现的方法，使用就必须在此作用域中
    // gen_range获取范围内的随机数(start..=end)


    println!("please input your guess");
    loop {

    let mut guess = String::new();
        // mut表示变量可变， rust中变量默认不可变
        // 类似于cpp，通过String::new()获取到一个String

   
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // io::stdin是一个函数，代表终端输入句柄的类型。
        // readline从句柄中获取用户输入。
        // rust中的引用&表示多出代码可以访问同一数据
        // expect在rust中很常见，readline函数返回一个枚举类型 
        // “result” enum。即表示多种状态中的一种，
        // 存在两种值：Ok，Err。通过不同的值expect实现不同的行为
        // 不同类型的个体由不同的expect函数实现    

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type number!");
                continue;
            },
        };
        // 前面已经存在了一个guess的命名变量。
        // 在rust中重名变量是被允许的，通常用于变量类型转换。
        // “shadowing”
        // trim消除前面的空格和后面的换行。
        // expect即parse自己的实现。


        println!("You guessed: {guess}");


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small！"),
            Ordering::Equal=> {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big"),
        }
        // match 可以根据函数的返回值（enum）
        // 来选择不同的做法。
    }
    


}
