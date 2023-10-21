use std::env::current_exe;
use crate::queue::Queue;

pub struct Walk {
    pub now: (usize, usize),
    pub record: Vec<(usize, usize)>
}

impl Walk {
    pub fn get_value(&self) -> Walk {
        return Walk{now: self.now, record: self.record.clone()}
    }
}

/*
0 represent available position
1 represent start
2 represent destination
3 represent obstacles
start: (y, x)
*/

pub fn bfs(maze: &Vec<Vec<usize>>, start: (usize, usize)) -> Walk {
    let y_length = maze.len();
    let x_length = maze[0].len();

    println!("y length {:?}", y_length);
    println!("x length {:?}", x_length);

    // initialize a visited 2D vector for storing visited position of maze
    let mut visited: Vec<Vec<bool>> = Vec::with_capacity(y_length);
    for _y in 0..y_length {
        let mut x_list: Vec<bool> = Vec::with_capacity(x_length);
        for _x in 0.. x_length {
            x_list.push(false)
        }
        visited.push(x_list);
    }
    // the start point should be visited
    visited[start.1][start.0] = true;

    // initialize queue
    let mut q:Queue<Walk> = Queue::new();
    let mut record: Vec<(usize, usize)> = Vec::new();
    record.push(start);
    q.push_back(Walk{now: start, record });

    while !q.is_empty() {
        {
            let current = q.peek().unwrap();

            // check if this walk reach the destination
            if maze[current.now.0][current.now.1] == 2 {
                break
            }
        }

        let current = q.pop_front();
        let y = current.now.0;
        let x = current.now.1;

        // check four directions
        if y - 1 > 0 && maze[y - 1][x] != 3 && !visited[y - 1][x] {
            let mut new_record = current.record.clone();
            new_record.push((y - 1, x));
            q.push_back(Walk{now: (y - 1, x), record: new_record});
        }
        if y + 1 < y_length && maze[y + 1][x] != 3 && !visited[y + 1][x] {
            let mut new_record = current.record.clone();
            new_record.push((y + 1, x));
            q.push_back(Walk{now: (y + 1, x), record: new_record});
        }
        if x - 1 > 0 && maze[y][x - 1] != 3 && !visited[y][x - 1] {
            let mut new_record = current.record.clone();
            new_record.push((y, x - 1));
            q.push_back(Walk{now: (y, x - 1), record: new_record});
        }
        if x + 1 < x_length && maze[y][x + 1] != 3 && !visited[y][x + 1] {
            let mut new_record = current.record.clone();
            new_record.push((y, x + 1));
            q.push_back(Walk{now: (y, x + 1), record: new_record});
        }
    }

    let shortest_walk = q.peek().unwrap();
    return shortest_walk.get_value()
}
