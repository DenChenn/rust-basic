pub fn heap_sort(a: &Vec<i32>) -> Vec<i32> {
    // copy the array to prevent from modifying the original array
    let mut new_array = a.clone();

    //  build a valid max-heap.
    let end = new_array.len();
    for start in (0..end / 2).rev() {
        represent_in_max_heap_order(&mut new_array, start, end - 1);
    }

    // swap the root(maximum value) of the heap with the last element of the heap
    for end in (1..new_array.len()).rev() {
        new_array.swap(end, 0);
        represent_in_max_heap_order(&mut new_array, 0, end - 1);
    }

    return new_array;
}

// make out array satisfy the max-heap order
// For any node at index i, its left child is at index 2 * i
// For any node at index i, its right child is at index 2 * i + 1
fn represent_in_max_heap_order(a: &mut Vec<i32>, start: usize, end: usize) {
    let mut root = start;
    loop {
        // Get the left child
        let mut child = root * 2;

        // if the child is out of the array, it means the root is a leaf node
        if child > end {
            break;
        }
        if child + 1 <= end && a[child] < a[child + 1] {
            // Right child exists and is greater.
            child += 1;
        }

        if a[root] < a[child] {
            // If child is greater than root, swap them.
            a.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}
