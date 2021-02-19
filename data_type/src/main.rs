fn main() {
    // println!("Hello, world!");

    // u32类型注解可以用于编译器做类型推断
    // let guess: u32 = "42".parse().expect("Not a number!");

    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;

    // 取余
    let remainder = 43 % 5;

    // println!("{},{},{}, {}, {}", sum, difference, product, quotient, remainder);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    // println!("{}", y);

    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    // 数组索引超出数组长度,不会报错,但会因为错误退出程序
    println!("The value of element is: {}", element);
}
