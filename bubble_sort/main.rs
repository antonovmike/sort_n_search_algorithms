// DOES NOT WORK
// Bubble sort
// One of the most inefficient sorting algorithms out there

fn main() {
    let mut array = [0,3,2,5,6,8,1,9,4,2,1,2,9,6,4,1,7,-1,-5,23,6,2,35,6,3,32];
    println!("Bubble sort: {:?}", bubble_sort(&mut array));
}

fn bubble_sort(array: &mut [i32]) -> &[i32] {
    let mut _count = 0;
    let mut i = 0;
    
    while i < array.len() {
        let mut j = 0;
        while j < array.len() - i - 1 {
            if array[j] > array[j+1] {
                array.swap(j, j+1);
                _count += 1;
            }
            j += 1;
        }
        i += 1;
    }

    array
}
