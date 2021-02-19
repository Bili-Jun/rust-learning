fn main() {
    println!("Hello, world!");
    new_function();
    // 字符串需要用双引号,需要对字符串做转换
    new_function2("2".to_string());
    // 字符串需要用双引号
    new_function3("2");
    let result = test(5);
    println!("这是测试函数返回值的方法,仅声明函数末尾表达式,而不是带‘;’的语句, {}",  result);
}

fn new_function() {
    println!("函数名要用下划线 '_'");
}

// 函数参数必须要指定类型(类型注解),字符串类型参数声明方式1
fn new_function2(x: String) {
    println!("函数名要用下划线 '_', 这是测试函数参数声明-字符串声明方式1, 值: {}", x);
}

// 函数参数必须要指定类型(类型注解),字符串类型参数声明方式2
fn new_function3(x: &str) {
    println!("函数名要用下划线 '_', 这是测试函数参数声明-字符串声明方式2, 值: {}", x);
}

// 函数返回值,必须要在函数“->”后声明类型
fn test(x: i32) -> i32 {
   // 请注意如果,函数结尾表达式加入分号,则变成语句,例如: x.无法作为返回值; 
   // 可以直接使用return关键字组成返回值语句,例如: return x;
   x
}