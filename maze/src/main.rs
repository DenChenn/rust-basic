mod bfs;

use bfs::bfs;

fn main() {
    let mut vec_2d: Vec<Vec<i32>> = Vec::new();

    vec_2d = vec![
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9]
    ];

    bfs(&vec_2d);

    println!("Hello, world!");
}
