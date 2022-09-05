// DOESN'T WORK
// Selection sort
// алгоритм выбора, сортировка пузырьком, быстрая сортировка
// O(n*n)


fn main() {
    let mut array = [0,3,2,5,6,8,1,9,4,2,2,9,6,4,1,7,-1,-5,23,6,2,35,6,3,32];
    let item = 11;
    println!("Selection sort: {:?}", selection_sort(&mut array));
}


fn selection_sort(array: &mut [i32; 25]) -> i32 {
	let mut count = 0;
	let mut i = 0;
	
	while i < array.len() {
		let mut index_min = i;
		let j = i + 1;
		while j < array.len() {
			if array[i] < array[index_min] {
				index_min = j				
			}
			count = count + 1;
			i = i + 1;
		}
		let tmp = array[i];
		array[i] = array[index_min];
		array[index_min] = tmp;
	}
	println!("Count = {}", count);
	return 0
}
