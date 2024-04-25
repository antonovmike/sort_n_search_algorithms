// Quicksort (Tony Hoare, 1959)
// The recursive algorithm is one of the fastest sorting algorithms, 
// running in O(log2n * n) time


fn main() {
    let mut numbers = vec![5, 2, 9, 3, 77, 5, 6];
    quicksort(&mut numbers);
    println!("Quicksort: {:?}", numbers);
}

fn quicksort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    let pivot_index = partition(arr);
    quicksort(&mut arr[0..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..len]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}