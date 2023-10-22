mod bfs;
mod queue;

use bfs::bfs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Open a file in read-only mode
    let file = File::open("./input.txt")?;

    // Create a buffer reader for the file
    let reader = BufReader::new(file);

    // Iterate over lines in the file
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line_content = line?;
        lines.push(line_content);
    }

    let mut maze_index = 0;

    while maze_index < lines.len() {
        let scale = &lines[maze_index];

        let parts = scale.split(" ").collect::<Vec<&str>>();
        let y_length: i32 = parts[0].parse().unwrap();
        let y_bound = y_length + 1;
        let x_length: i32 = parts[1].parse().unwrap();
        let x_bound = x_length + 1;

        let mut maze: Vec<Vec<usize>> = Vec::with_capacity(y_length as usize);

        // initialize with all zero
        for _i in 0 ..y_bound {
            let mut x_list: Vec<usize> = Vec::with_capacity(x_length as usize);
            for _j in 0..x_bound {
                x_list.push(0);
            }
            maze.push(x_list);
        }

        // insert element
        let mut start: (usize, usize) = (0, 0);
        for i in 0..y_length {
            let maze_row_index: usize = (i + 1) as usize;
            let row_with_space = &lines[maze_index+maze_row_index];
            let row_vec = row_with_space.split(" ").collect::<Vec<&str>>();

            for index in 0..row_vec.len() {
                if index % 2 == 0 {
                    let x_char_index = row_vec[index];
                    if x_char_index == "0" {
                        continue
                    }

                    let x_element = row_vec[index+1];

                    let x_index : usize = x_char_index.parse().unwrap();

                    if x_element == "x" {
                        maze[maze_row_index][x_index] = 3;
                    } else if x_element == "s" {
                        maze[maze_row_index][x_index] = 1;
                        start = (maze_row_index as usize, x_index as usize);
                    } else if x_element == "t" {
                        maze[maze_row_index][x_index] = 2;
                    }
                }
            }

        }

        maze_index = maze_index + y_length as usize + 1;
        // do bfs
        let walk = bfs(&maze, start);
        println!("{:?}", walk.record);
    }

    Ok(())
}
