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
    let y_length = maze.len() as usize;
    let x_length = maze[0].len() as usize;
    let mut visited: Vec<Vec<i32>> = Vec::with_capacity(y_length * x_length);
    for y in 0..y_length {
        for x in 0.. x_length {
            visited[y][x] = 0
        }
    }

    visited.into_iter().for_each(|it| {
        println!("{:#?}", it);
    })
}
