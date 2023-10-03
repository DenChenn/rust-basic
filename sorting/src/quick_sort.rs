// sort array in ascending order
pub fn quick_sort(a: &Vec<i32>) -> Vec<i32> {
    // copy the array to prevent from modifying the original array
    let mut new_array = a.clone();

    let l = new_array.len();
    quicksort_recursion(&mut new_array, 0, (l - 1) as isize);

    return  new_array;
}

fn quicksort_recursion(arr: &mut Vec<i32>, left: isize, right: isize) {
    if left <= right {
        let pivot = separate_by_pivot(arr, left, right);
        quicksort_recursion(arr, left, pivot - 1);
        quicksort_recursion(arr, pivot + 1, right);
    }
}

fn separate_by_pivot(arr: &mut Vec<i32>, left: isize, right: isize) -> isize {
    // randomly select a pivot, here we choose the last element
    let pivot = arr[right as usize];
    let mut i = left;

    for j in left..right {
        // swap to the most left if the element is less than the pivot
        if arr[j as usize] < pivot {
            arr.swap(i as usize, j as usize);
            // modify the index of the most left element
            i += 1;
        }
    }

    // swap the pivot to the latest most left
    arr.swap(i as usize, right as usize);
    return i;
}
