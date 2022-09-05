// rucursive binary search

fn main() {
    let array = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    let item = 11;
    println!("Recursive binary search: {:?}", recursive_binary_search(&array, item, 0, array.len()));
}
fn recursive_binary_search(array: &[i32; 16], item: i32, start: usize, end: usize) -> i32 {
	let mut count = 0;
	let mut middle: usize = (start + end) / 2;
	count = count + 1;
	if item == array[middle] {
		println!("Count = {}", count);
		return middle as i32
	}
	if item < array[middle] {
		return recursive_binary_search(array, item, start, middle - 1)
	} else {
		return recursive_binary_search(array, item, middle + 1, end)
	}
}
