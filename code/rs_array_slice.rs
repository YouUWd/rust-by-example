use std::mem;

// 此函数借用一个 slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // 定长数组（类型标记是多余的）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];

    // 下标从 0 开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);
    // `len` 返回数组的大小
    println!("array size: {}", xs.len());
    // 数组是在栈中分配的
    println!("array xs occupies {} bytes", mem::size_of_val(&xs));
    println!("array ys occupies {} bytes", mem::size_of_val(&ys));

    // 数组可以自动被借用成为 slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slice 可以指向数组的一部分.
    // &array[start..end]：创建一个从array中的start索引到end索引（但不包含 end 索引）的切片。
    // start和end是范围操作符..的参数，用于定义切片的开始位置和结束位置（但不包括结束位置）。
    // 注意：start索引可以不写，不写时默认为0；end索引也可以不写，不写时默认为array的最后一个元素的索引。
    println!("borrow a section of the array as a slice");
    analyze_slice(&xs[1..3]);
    analyze_slice(&xs[1..]);
    analyze_slice(&xs[..3]);
}
