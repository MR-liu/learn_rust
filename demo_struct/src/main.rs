
// 元祖结构
struct Pair(i32, f32);

// 经典C结构
#[derive(Debug)] // 派生属性 编译时自动载入的方法
struct Person {
    name: String,
    age: u8,
}

// 无字段的单元结构
// 在泛型中较为常用
// 特点是无字段
struct Unit;

fn main() {
    // 结构体的实例化
    let pair = Pair(12, 3.3);

    let person = Person {
        name: String::from("name"), // 必须跟定义的String一致
        age: 1
    };

    let unit = Unit;

    println!("{}", pair.0);
    println!("{}", person.name);

    // 要想打印出struct 必须先引入派生属性 #[derive(Debug)]
    // 打印的时候需要用{:?} 才可以
    println!("{:?}", person);
}
