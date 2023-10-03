// sort array in ascending order
pub fn merge_sort(a: &Vec<i32>) -> Vec<i32> {
    let mut new_array = a.clone();
    mergesort_recursion(&mut new_array);
    return new_array;
}

fn mergesort_recursion(a: &mut Vec<i32>) {
    let mid = a.len() / 2;
    if mid == 0 {
        return;
    }

    let mut left_vec = a[0..mid].to_vec();
    let mut right_vec = a[mid..a.len()].to_vec();
    mergesort_recursion(&mut left_vec);
    mergesort_recursion(&mut right_vec);

    // create an vector to store the sorted array later
    // using clone here is not a good idea, but it's fine for now
    // ret will have the same length as a
    let mut ret = a.clone();

    // merge the sorted left and right array into ret
    merge(&left_vec, &right_vec, &mut ret);

    // return the result back to a
    a.copy_from_slice(&ret);
}

fn merge(arr1: &Vec<i32>, arr2: &Vec<i32>, ret: &mut Vec<i32>) {
    // left and right are the index of arr1 and arr2
    let mut li = 0;
    let mut ri = 0;
    let mut i = 0;

    // compare the elements from both arrays and put the smaller one into ret in order
    while li < arr1.len() && ri < arr2.len() {
        if arr1[li] <= arr2[ri] {
            ret[i] = arr1[li];
            i += 1;
            li += 1;
        } else {
            ret[i] = arr2[ri];
            i += 1;
            ri += 1;
        }
    }

    // if anything left
    // deal with the remaining elements
    if li < arr1.len() {
        ret[i..].copy_from_slice(&arr1[li..]);
    }
    if ri < arr2.len() {
        ret[i..].copy_from_slice(&arr2[ri..]);
    }
}
