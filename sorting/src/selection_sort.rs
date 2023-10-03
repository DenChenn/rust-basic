// sort array in ascending order
pub fn selection_sort(a: &Vec<i32>) -> Vec<i32> {
    let mut new_array = a.clone();
    let l = new_array.len();

    for i in 0..l {
        let mut temp = i;
        // find the smallest element in the remaining array
        for j in (i + 1)..l {
            if new_array[temp] > new_array[j] {
                temp = j;
            }
        }
        new_array.swap(i, temp);
    }

    return new_array;
}
