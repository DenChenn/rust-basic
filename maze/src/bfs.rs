use std::collections::VecDeque;

struct Walk {
    now: (i32, i32),
    record: Vec<(i32, i32)>
}

/*
0 represent available position
1 represent start
2 represent destination
*/

pub fn bfs(maze: &Vec<Vec<i32>>) {
    let y_length = maze.len();
    let x_length = maze[0].len();
    let mut visited: Vec<Vec<i32>> = Vec::with_capacity(y_length);
    for y in 0..y_length {
        let mut x_list: Vec<i32> = Vec::with_capacity(x_length);
        for x in 0.. x_length {
            x_list.push(0)
        }
        visited.push(x_list);
    }

    visited.into_iter().for_each(|it| {
        println!("{:#?}", it);
    })
}
