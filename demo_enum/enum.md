## 枚举

带枚举值的枚举

```
// 带枚举值的枚举
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// 带参数的枚举
// Rust还支持携带类型参数的枚举
enum IpAddr {
    IPv4(u8, u8, u8, u8),
    IPv6(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)
}

// 模式匹配
fn main() {
    let localhost: IpAddr = IpAddr::IPv4(127, 0, 0, 1);
    match localhost {
        IpAddr::IPv4(a, b, c, d) => {
            println!("{} {} {} {}", a, b, c, d);
        }
        _ => {
            println!("{}", 1);
        }
    }
}

```