use crate::options::{CAVE_GENERATION_HEIGHT, CAVE_GENERATION_WIDTH};
pub mod display;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

const FILLED: bool = true;
const EMPTY: bool = false;

const cv_width: usize = CAVE_GENERATION_WIDTH as usize;
const cv_height: usize = CAVE_GENERATION_HEIGHT as usize;

pub fn generate_cavern(seed_u64: u64, p_filled: f64, n_iterations: u32) 
-> [[bool; cv_height]; cv_width] {

    let mut grid: [[bool; cv_height]; cv_width] = 
        [[false; cv_height]; cv_width];
    let mut random_generator = StdRng::seed_from_u64(seed_u64);

    for i in 0..cv_width {
        for j in 0..cv_height {
            grid[i][j] = random_generator.gen_bool(p_filled);
        }
    };
    for i in 0..cv_width {
        grid[i][0] = FILLED;
        grid[i][cv_height-1] = FILLED
    };
    for j in 0..cv_height {
        grid[0][j] = FILLED;
        grid[cv_width-1][j] = FILLED
    };

    for _ in 0..n_iterations {
        let nb = neighbors_grid(&grid);
        for i in 1..(cv_width-1) {
            for j in 1..(cv_height-1) {
                if grid[i][j] && nb[i][j] < 4 {
                    grid[i][j] = EMPTY
                } else if nb[i][j] >= 5 {
                    grid[i][j] = FILLED
                }
            }
        }
    };

    grid
}

fn neighbors_grid(grid: &[[bool; cv_height]; cv_width]) 
-> [[u8; cv_height]; cv_width] {
    let mut ng = [[0; cv_height]; cv_width];
    
    for i in 1..(cv_width-1) {
        for j in 1..(cv_height-1) as usize {
            if grid[i][j] {
                ng[i-1][j-1] += 1; ng[i][j-1] += 1; ng[i+1][j-1] += 1;
                ng[i-1][j  ] += 1;                  ng[i+1][j  ] += 1;
                ng[i-1][j+1] += 1; ng[i][j+1] += 1; ng[i+1][j+1] += 1;
            }
        }
    }

    for i in 1..(cv_width-1) as usize {
        ng[i-1][0] += 1;                ng[i+1][0] += 1;
        ng[i-1][1] += 1; ng[i][1] += 1; ng[i+1][1] += 1;

        ng[i-1][cv_height-2] += 1; ng[i][cv_height-2] += 1; ng[i+1][cv_height-2] += 1;
        ng[i-1][cv_height-1] += 1;                          ng[i+1][cv_height-1] += 1;
    }

    for j in 1..(cv_height-1) as usize {
        ng[0][j-1] += 1; ng[1][j-1] += 1;
                         ng[1][j  ] += 1;
        ng[0][j+1] += 1; ng[1][j+1] += 1;

        ng[cv_width-2][j-1] += 1; ng[cv_width-1][j-1] += 1;
        ng[cv_width-2][j  ] += 1;                   
        ng[cv_width-2][j+1] += 1; ng[cv_width-1][j+1] += 1;
    }

    ng[0][0] += 1; ng[1][0] += 1;
    ng[0][1] += 1;

    ng[0][cv_height-2] += 1;
    ng[0][cv_height-1] += 1; ng[1][cv_height-1] += 1;

                                      ng[cv_width-1][cv_height-2] += 1;
    ng[cv_width-2][cv_height-1] += 1; ng[cv_width-1][cv_height-1] += 1;

    ng[cv_width-2][0] += 1; ng[cv_width-1][0] += 1;
                            ng[cv_width-1][1] += 1;

    ng
}