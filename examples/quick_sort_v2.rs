fn main() {
    let mut arr = [99, 2, 1, 32, 45, 2, 6, 99, 10];
    // let mut arr = [4, 1, 2, 3];
    println!("{:10}: {:?}", "original array", arr);
    quick_sort(&mut arr);
    println!("{:10}: {:?}", "sorted array", arr)
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() > 1 {
        let mid = partition(arr);
        let (left, right) = arr.split_at_mut(mid);
        quick_sort(left);
        quick_sort(&mut right[1..]);
    }
}

fn partition(arr: &mut [i32]) -> usize {
    let last = arr.len() - 1;
    let last_value = arr[last];

    let mut index = 0;
    for i in 0..last {
        if arr[i] < last_value {
            arr.swap(i, index);
            index += 1;
        }
    }
    arr.swap(index, last);

    // println!("partition: {arr:?}, index: {index}");
    index
}
