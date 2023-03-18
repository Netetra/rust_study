use rand::{self, Rng};

const MAZE_N:usize = 25;

fn main() {
    let mut rng = rand::thread_rng();
    let mut maze = [[0; MAZE_N]; MAZE_N];
    for n in 0..MAZE_N {
        maze[n][0] = 1;
        maze[0][n] = 1;
        maze[n][MAZE_N-1] =1;
        maze[MAZE_N-1][n] =1;
    }
    for y in 2..MAZE_N-2 {
        for x in 2..MAZE_N-2 {
            if x%2 == 1 || y%2 == 1 { continue; }
            maze[y][x] = 1;
            let rand_num = rng.gen_range(0..4);
            match rand_num {
                0 => maze[y-1][x] = 1,
                1 => maze[y+1][x] = 1,
                2 => maze[y][x-1] = 1,
                3 => maze[y][x+1] = 1,
                _ => {},
            }
        }
    }
    let tiles = ["  ", "ZZ"];
    for y in 0..MAZE_N {
        for x in 0..MAZE_N {
            print!("{}", tiles[maze[y][x]])
        }
        println!("")
    }
}
