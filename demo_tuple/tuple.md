## 元祖

元祖是将多个具有各种类型的值组合成一个复合类型的通用方法。元祖有固定的长度，一旦声明。他们的大小就不能增长或者收缩。

我们通过在括号内写一个逗号分隔的值来创建元祖。

元祖中的每个位置都有一个类型。元祖中不同的值的类型不必相同。

```
fn main() {
    let a: i32 = 19;
    let b: char = 'A';

    let my_tuple: (i32, char) = (a, b);

    println!("a ==> {}", my_tuple.0);
    println!("b ==> {}", my_tuple.1);

    // 解封装
    let (c, d) = my_tuple;
    println!("c ==> {}", c);
    println!("d ==> {}", d);
}

```