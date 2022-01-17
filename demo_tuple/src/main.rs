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
