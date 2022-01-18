## 在不同类型之间转换
Rust 是一门强类型语言,因此不支持隐式类型转换,Rust 为了实现类型之间的转换提供了几种不同的方法

### as 语法
as 语法是 Rust 最基础的一种类型转换方法,它通常用于整数,浮点数和字符数据之间的类型转换.
```

fn main() {
    let a: i8 = -10;

    let b = a as u8;

    println!("a={} b-{}", a, b);
}

```

数值转换的语义是:

● 两个相同大小的整型之间(例如:132->u32)的转换是一个no-op
● 从一个大的整型转换为一个小的整型(例如:u32->u8)会截断
● 从一个小的整型转换为一个大的整型(例如:u8->u32)会
  。如果源类型是无符号的会补零(zero-extend)
  。 如果源类型是有符号的会符号(sign-extend)
● 从一个浮点转换为一个整型会向0舍入
● 从一个整型转换为一个浮点会产生整型的浮点表示,如有必要会舍入(未指定舍入策略)
● 从f32转换为f64是完美无缺的
● 从f64转换为f32 会产生最接近的可能值(未指定舍入策略)

### transmute
as 只允许安全的转换,例如会拒绝例如尝试将4个字节转换为一个 u32:

```
let a = [0u8, 0u8, 0u8, 0u8];
let b = a as u32; // Four u8s makes a u32.
```

但是我们知道u32在内存中表示为4个连续的u8,因此我们可以使用一种危险的方法:告诉编译器直
接以另一种数据类型对待内存中的数据,编译器会无条件信任你,但是,除非你知道自己在干什么,不
然并不推荐使用 transmute. 要使用 transmute,需要将代码写入 unsafe 块中.
fn main() {
unsafe {
let a = [Ou8, 1u8, Ou8, Ou8];
let b - mem: : transmute: : <[u8; 4], u32>(a);
println! ("{}", b); // 256
// Or, more concisely:
let c: 032 = mem: : transmute(a);
println!("{}", c); // 256