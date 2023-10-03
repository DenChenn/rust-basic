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
use std::time::Instant;

fn main() {
    let debug_mode = true;

    let mut rng = rand::thread_rng();
    let n = 10000;
    let range = Uniform::new(0, n);

    // get 100 random numbers between 0 and 10000
    let arr: Vec<i32> = (0..n).map(|_| rng.sample(&range)).collect();
    // initialize an increasing array
    let increasing_arr = (0..n).collect();

    // insertion sort
    let insertion_start = Instant::now();
    let insertion_sort_result = insertion_sort(&arr);
    let insertion_duration = insertion_start.elapsed();
    let insertion_start_for_increasing_arr = Instant::now();
    let _ = insertion_sort(&increasing_arr);
    let insertion_duration_for_increasing_arr = insertion_start_for_increasing_arr.elapsed();
    // selection sort
    let selection_start = Instant::now();
    let selection_sort_result = selection_sort(&arr);
    let selection_duration = selection_start.elapsed();
    let selection_start_for_increasing_arr = Instant::now();
    let _ = selection_sort(&increasing_arr);
    let selection_duration_for_increasing_arr = selection_start_for_increasing_arr.elapsed();
    // quick sort
    let quick_start = Instant::now();
    let quick_sort_result = quick_sort(&arr);
    let quick_duration = quick_start.elapsed();
    let quick_start_for_increasing_arr = Instant::now();
    let _ = quick_sort(&increasing_arr);
    let quick_duration_for_increasing_arr = quick_start_for_increasing_arr.elapsed();
    // merge sort
    let merge_start = Instant::now();
    let merge_sort_result = merge_sort(&arr);
    let merge_duration = merge_start.elapsed();
    let merge_start_for_increasing_arr = Instant::now();
    let _ = merge_sort(&increasing_arr);
    let merge_duration_for_increasing_arr = merge_start_for_increasing_arr.elapsed();
    // heap sort
    let heap_start = Instant::now();
    let heap_sort_result = heap_sort(&arr);
    let heap_duration = heap_start.elapsed();
    let heap_start_for_increasing_arr = Instant::now();
    let _ = heap_sort(&increasing_arr);
    let heap_duration_for_increasing_arr = heap_start_for_increasing_arr.elapsed();

    // if debug mode is on, print the results
    if debug_mode {
        println!("original array: {:?}", arr);
        println!("increasing array: {:?}", increasing_arr);
        println!("insertion sort result: {:?}", insertion_sort_result);
        println!("selection sort result: {:?}", selection_sort_result);
        println!("quick sort result: {:?}", quick_sort_result);
        println!("merge sort result: {:?}", merge_sort_result);
        println!("heap sort result: {:?}", heap_sort_result);
    }

    // print the durations
    println!("insertion sort duration: {:?}", insertion_duration);
    println!("insertion sort duration for increasing array: {:?}", insertion_duration_for_increasing_arr);
    println!("selection sort duration: {:?}", selection_duration);
    println!("selection sort duration for increasing array: {:?}", selection_duration_for_increasing_arr);
    println!("quick sort duration: {:?}", quick_duration);
    println!("quick sort duration for increasing array: {:?}", quick_duration_for_increasing_arr);
    println!("merge sort duration: {:?}", merge_duration);
    println!("merge sort duration for increasing array: {:?}", merge_duration_for_increasing_arr);
    println!("heap sort duration: {:?}", heap_duration);
    println!("heap sort duration for increasing array: {:?}", heap_duration_for_increasing_arr);
}
