// DOES NOT WORK
// Bubble sort
// алгоритм выбора, сортировка пузырьком, быстрая сортировка
// Один из самых не эффективных алгоритмов сортировки

fn main() {
    let mut array = [0,3,2,5,6,8,1,9,4,2,1,2,9,6,4,1,7,-1,-5,23,6,2,35,6,3,32];
    let item = 11;
    println!("Bubble sort: {:?}", bubble_sort(&mut array));
}


fn bubble_sort(array: &mut [i32; 26]) -> i32 {
	let mut count = 0;
	let mut i = 0;
	let mut j = 0;
	
	while i < array.len() {
		while j < array.len() {
			if array[j+1] < array[j] {
				let tmp = array[j+1];
				array[j] = array[j+1];
				array[j+1] = tmp;
			}
			count = count + 1;
			j = j + 1;	
		}
		i = i + 1;
	}

	return 0
}
