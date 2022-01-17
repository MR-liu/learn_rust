## 数组

另一种拥有多个数据集合的方法是使用数组。

***与元祖不同，数组中的每个元素都必须具有相同的类型***

Rust中的数组不同其他语言的数组，Rust中的数组具有固定长度。

数组下标以0开始，同时 ***RUST中存在越界检查***

```
fn main() {
    // 创建数组 [i32; 12] 是数组的类型提示，表示元素的类型是i32共有12个元素
    let my_array: [i32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 11, 12];

    // 根据索引获取一个值，数组下标从0开始
    println!("{:?}", my_array[1]);

    // 索引不能越界
    // 这时候是编译时既报错
    // println!("{:?}", my_array[13]);

    // 换个方式 将索引放在堆上
    // let index = "13".parse::<usize>().unwrap();
    // println!("{:?}", my_array[index]);
    // 此时编译不报错
    // 执行报错
    // thread 'main' panicked at 'index out of bounds: the len is 12 but the index is 13', src/main.rs:14:22
    // 

    // 如果数组每个元素相同，我们可以简化数组初始化
    let my_array_1: [i32; 13] = [1; 13];
    println!("{:?}", my_array_1[12]);

    // 不加mut不可以改变
    let mut my_buffer: [u32; 32 * 1024] = [1; 32 * 1024];

    println!("my_buffer ====> {:?}", my_buffer[1024]);

    my_buffer[1024] = 0;

    println!("my_buffer ====> {:?}", my_buffer[1024]);
}



```