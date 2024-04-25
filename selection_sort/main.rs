// DOESN'T WORK
// Selection sort, selection algorithm, bubble sorting, fast sorting
// O(n*n)

fn main() {
    let mut array = [0,3,2,5,6,8,1,9,4,2,2,9,6,4,1,7,-1,-5,23,6,2,35,6,3,32];
    println!("Selection sort: {:?}", selection_sort(&mut array));
}

fn selection_sort(array: &mut [i32]) -> &[i32] {
    let mut count = 0;
    let mut i = 0;
    
    while i < array.len() {
        let mut index_min = i;
        let mut j = i + 1;
        while j < array.len() {
            if array[j] < array[index_min] {
                index_min = j;
            }
            count += 1;
            j += 1;
        }
        array.swap(i, index_min);
        i += 1;
    }

    println!("Count = {}", count);
    array
}
