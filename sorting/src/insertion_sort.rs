// sort array in ascending order
pub fn insertion_sort(a: &Vec<i32>) -> Vec<i32> {
    // copy the array to prevent from modifying the original array
    let mut new_array = a.clone();

    // sort the array
    for i in 1..new_array.len() {
        let mut j = i;
        while j > 0 && new_array[j - 1] > new_array[j] {
            // swap if the previous element is greater
            new_array.swap(j - 1, j);
            j -= 1;
        }
    }

    // return the sorted array
    return new_array;
}
