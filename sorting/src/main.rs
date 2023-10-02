mod insertion_sort;
mod selection_sort;
mod quick_sort;
mod merge_sort;
mod heap_sort;

use rand::{distributions::Uniform, Rng};
use insertion_sort::insertion_sort;
use selection_sort::selection_sort;
use quick_sort::quick_sort;
use merge_sort::merge_sort;
use heap_sort::heap_sort;

fn main() {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 1000);

    // get 100 random numbers between 0 and 10000
    let arr: Vec<i32> = (0..100).map(|_| rng.sample(&range)).collect();

    // insertion sort
    let insertion_sort_result = insertion_sort(&arr);
    // selection sort
    let selection_sort_result = selection_sort(&arr);
    // quick sort
    let quick_sort_result = quick_sort(&arr);
    // merge sort
    let merge_sort_result = merge_sort(&arr);
    // heap sort
    let heap_sort_result = heap_sort(&arr);


    println!("a: {:?}", arr);
    println!("insertion_sort_result: {:?}", insertion_sort_result);
    println!("selection_sort_result: {:?}", selection_sort_result);
    println!("quick_sort_result: {:?}", quick_sort_result);
    println!("merge_sort_result: {:?}", merge_sort_result);
    println!("heap_sort_result: {:?}", heap_sort_result);
}
