mod bfs;
mod queue;

use bfs::bfs;

fn main() {
    let mut vec_2d: Vec<Vec<usize>> = Vec::new();

    vec_2d = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 3, 0, 0, 0],
        vec![0, 0, 1, 0, 3, 0],
        vec![0, 0, 0, 3, 0, 6],
        vec![0, 0, 3, 0, 3, 2],
    ];

    let w = bfs(&vec_2d, (2, 2));

    println!("{:?}", w.now);
    println!("{:?}", w.record);
}
