// Binary search O(log n 2) (“O” from the logarithm of “n” on base 2)
// In 4 iterations you can find any element in an array of 16 elements

fn main() {
    let array = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    let item = 11;
    println!("Binary search: {:?}", binary_search(&array, item));
}
fn binary_search(array: &[i32; 16], item: i32) -> i32 {
	let mut count = 0;
	let mut start = 0;
	let mut end = array.len();
	let mut middle;
	let mut position = 0;

	while start <= end {
		count = count + 1;
		middle = (start + end) / 2;
		if array[middle] == item {
			position = middle;
			println!("Count = {}", count);
			return position as i32
		}
		if item < array[middle] {
			end = middle - 1
		} else {
			start = middle + 1
		}
	}
	return position as i32
}
