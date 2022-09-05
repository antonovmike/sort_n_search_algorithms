// Linear search O(n)

fn main() {
    let array = [1,4,5,8,51,2,7,5,2,11];
    let item = 11;
    println!("Linear search: {:?}", linear_search(&array, item));
}

fn linear_search(array: &[i32; 10], item: i32) -> i32 {
	let mut count = 0;
	let mut i = 0;
	while i < array.len() {
		if array[i] == item {
			println!("Count = {}", count + 1);
			return i as i32;
		}
		i = i + 1 as usize;
		count = count + 1;
	}
	return 0
}
