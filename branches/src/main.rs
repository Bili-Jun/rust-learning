fn main() {
    // println!("Hello, world!");

    let number = 3;

    if number < 10 {
        println!("这是测试if语句用法");
    }

    // if 之后的表达式必须为bool类型否则会报错
    // if number {
    //     println!("这是测试if报错,这里不会执行")
    // }

    // if 表达式可在给用于赋值
    let number2 = if true {
        6
    } else {
        5
        // "five" 
        // if 表达式分支的返回值类型要保持一致,否则会报错
    }; // 注意:分号是一定要写的

    println!("这是测试if表达式赋值: {}", number2);


    loop_test(5);

    let mut number3 = 3;
    // while 循环表达式和if表达式写法一样,需要bool类型作为条件,注意不用像其他语言一样写‘()’
    while number3 != 0 {
        number3 -= 1;
        println!("这是测试while循环 {}", number3);
        if number3 == 0 {
            println!("while循环结束");
        }
    }

    let array = [10, 20, 30, 40, 50];
    // for循环,可以规避超出数组长度的问题
    for item in array.iter() {
        println!("这是测试for循环: {}", item);
    }
    c_to_f(30.0);
    f_to_c(30.0);
}

fn loop_test(stop: i32) {
    let mut count = 0;
    loop {
        count += 1;
        println!("这是测试loop循环. again!");
        if count == stop {
            break println!("loop循环结束 {}", stop);
        }
    }
}

fn c_to_f(c: f32) {
    println!("摄氏度{}℃转华氏度: {}℃", c, c*1.8 + 32.0);
}

fn f_to_c(f: f32) {
    println!("华氏度{}℉转摄氏度: {}℉", f, (f - 32.0) / 1.8);
}