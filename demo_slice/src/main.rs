fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // ..是RUST Range语法
    // &是引用符号
    let slice = &arr[0..3];

    println!("{}, {}", slice[0], slice.len());

    // 要取后两位
    let slice_1 = &arr[3..5];
    println!("{}, {}", slice_1[0], slice_1.len());

    let slice_2 = &arr[arr.len()-2 ..arr.len()];

    println!("{}, {}", slice_2[0], slice_2.len());

    println!("Hello, world!");

    // 常用的函数是 len() is_empty()


    // 修改数据 引用是不是会变
    let mut arr_1: [i32; 5] = [1, 2, 3, 4, 5];

    arr_1[3] = 12;

    let slice_1_1 = &arr_1[3..5];

    // 会跟着变的
    println!("{}, ===> {}", slice_1_1[0], arr_1[3]);
}
