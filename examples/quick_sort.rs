fn main() {
    let mut arr = [99, 2, 1, 32, 45, 2, 6, 99, 10];
    // let mut arr = [4, 1, 2, 3];
    let high = arr.len() - 1;
    println!("{:10}: {:?}", "original array", arr);
    quick_sort(&mut arr, 0, high);
    println!("{:10}: {:?}", "sorted array", arr)
}

fn quick_sort<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let mid = partition(arr, low, high);
        // println!("{low},{mid},{high}");
        if mid > 0 {
            quick_sort(arr, low, mid - 1);
        }
        quick_sort(arr, mid + 1, high);
    }
}

fn partition<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) -> usize {
    let povit = arr[high];
    let mut sorted = low;
    for i in low..high {
        if arr[i] < povit {
            arr.swap(i, sorted);
            sorted += 1;
        }
    }
    arr.swap(sorted, high);
    sorted
}
