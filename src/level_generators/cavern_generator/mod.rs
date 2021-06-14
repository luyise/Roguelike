pub mod display;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

const FILLED: bool = true;
const EMPTY: bool = false;

pub fn generate_cavern(cv_width: usize, cv_height: usize, seed_u64: u64, p_filled: f64, n_iterations: u32) 
-> (Vec<Vec<bool>>, Vec<((usize, usize), (usize, usize))>) {

    let mut grid: Vec<Vec<bool>> = 
        vec![vec![false; cv_height]; cv_width];
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
        let nb = neighbors_grid(cv_width, cv_height, &grid);
        for i in 1..(cv_width-1) {
            for j in 1..(cv_height-1) {
                if grid[i][j] && nb[i][j] < 4 { // /!\ Règles standards : < 4, rules_4 : <= 4
                    grid[i][j] = EMPTY
                } else if nb[i][j] >= 5 { // /!\ Règles standards : >= 5, rules_4 : >= 6
                    grid[i][j] = FILLED
                }
            }
        }
    };

    // règles 0 avec post_traitement_0 : on injecte les petites composantes connexes et on rebouche les goulets dans les grandes composantes connexes
    // cc_grid stocke à quelle composante appartient quelle case.
    let mut cc_grid: Vec<Vec<usize>> = vec![vec![0; cv_height as usize]; cv_width as usize];
    // la première composante de la première case de cc_list contient le nombre de composantes connexes, les cases suivantes contiennent la iste des cases d'un composante donnée
    let mut cc_list: Vec<Vec<(usize,usize)>> = vec![vec![(0,0)]];
    // la cc-ème case de cc_bd contient la frontière de la cc-ème compoante connexe du graphe
    let mut cc_bd: Vec<Vec<(usize, usize)>> = vec![vec![]];
    // i, j sont incrémentés lorsqu'on cherche une nouvelle composante connexe.
    let mut i: usize = 0;
    let mut j: usize = 0;
    // cc est numéro de la composante connexe en cours de traitement, 0 est réservé pour les cases non traitées!
    let mut cc: usize = 0;

    while let Some((x, y)) = find_new(&grid, &mut cc_grid, cv_width, cv_height, i, j) {
        // println!("x: {}, y: {}", x, y);
        cc += 1;
        cc_list[0][0].0 += 1;
        cc_list.push(Vec::new());
        cc_bd.push(Vec::new());
        i = x;
        j = y;
        explore(&grid, &mut cc_grid, &mut cc_list, &mut cc_bd, cc, x, y);
    };

    // On injecte les petites composantes (- de 12 éléments)
    for cc in cc_list.iter().skip(1) {
        if cc.len() <= 12 {
            for cell in cc.iter() {
                grid[cell.0][cell.1] = FILLED
            }
        }
    };
    for bd in cc_bd.iter_mut().skip(1) {
        bd.dedup()
    };

    // On découpe les grandes composantes sous reserve de conserver des composantes assez grande après la coupe.
    let nb_try = 100;
    let mut claws: Vec<((usize, usize), (usize, usize))> = Vec::new();
    for bd in cc_bd.iter().skip(1) {
        let len = bd.len();
        for _ in 0..nb_try {
            // let mut tries = 0;
            let i = random_generator.gen_range(0..len);
            let (x1, y1) = (bd[i].0, bd[i].1);
            // while tries < 20 && grid[x1][y1] == EMPTY {
            //     tries += 1;
            //     x1 = random_generator.gen_range(1..cv_width-1);
            //     y1 = random_generator.gen_range(1..cv_height-1);
            // } if tries == 20 { continue }
            // tries = 0;
            let j = random_generator.gen_range(0..len);
            let (x2, y2) = (bd[j].0, bd[j].1);
            // while tries < 20 && (x2 < 1 || x2 >= cv_width-1 || y2 < 1 || y2 >= cv_height-1 || grid[x2][y2] == EMPTY) {
            //     tries += 1;
            //     x2 = x1 + random_generator.gen_range(1..20);
            //     y2 = y1 + random_generator.gen_range(1..20);
            // } if tries == 20 { continue }
            
            let ((x1, y1), (x2, y2)) = claw(&grid, cv_width, cv_height, (x1 as i64, y1 as i64), (x2 as i64, y2 as i64));
            let d = dist1((x1 as i64, y1 as i64), (x2 as i64, y2 as i64));

            if d < 5 && d > 0 {
                claws.push( ((x1 as usize, y1 as usize), (x2 as usize, y2 as usize)) )
            }
        }
    };
    claws.dedup();
    println!("found {} claws:", claws.len());
    for claw in claws.iter() {
        println!("({}, {}), ({}, {}) at distance {}", claw.0.0, claw.0.1, claw.1.0, claw.1.1, dist1((claw.0.0 as i64, claw.0.1 as i64), (claw.1.0 as i64, claw.1.1 as i64)))
    };
        
    (grid, claws)
}

fn claw(grid: &Vec<Vec<bool>>, cv_width: usize, cv_height: usize, (x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> ((i64, i64), (i64, i64)) {
    let d = dist1((x1, y1), (x2, y2));
    for dx1 in (-1)..=1 {
        let nx1 = x1 + dx1;
        for dy1 in (-1)..=1 {
            let ny1 = y1 + dy1;
            if nx1 >= 0 && nx1 < cv_width as i64 && ny1 >= 0 && ny1 < cv_height as i64
            && grid[nx1 as usize][ny1 as usize] == FILLED {
                for dx2 in (-1)..=1 {
                    let nx2 = x2 + dx2;
                    for dy2 in (-1)..=1 {
                        let ny2 = y2 + dy2;
                        if nx2 >= 0 && nx2 < cv_width as i64 && ny2 >= 0 && ny2 < cv_height as i64
                        && grid[nx2 as usize][ny2 as usize] == FILLED {
                            let d_try = dist1((nx1, ny1), (nx2, ny2));
                            if d_try < d {
                                return claw(grid, cv_width, cv_height, (nx1, ny1), (nx2, ny2))
                            }
                        }
                    }
                }
            }
        }
    };

    ((x1, y1), (x2, y2))
}

fn dist1((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (x2 - x1).abs() + (y2 - y1).abs()
}

// explore(cc, i, j) découvre récursivement la composante connexe cc.
fn explore(grid: &Vec<Vec<bool>>, cc_grid: &mut Vec<Vec<usize>>, cc_list: &mut Vec<Vec<(usize,usize)>>, cc_bd: &mut Vec<Vec<(usize, usize)>>, cc: usize, i: usize, j: usize){
    if cc_grid[i][j] != 0 {
        panic!("Error with connex component finding!")
    } else {
        cc_grid[i][j] = cc;
        cc_list[cc].push((i,j));
        for (di, dj) in [(1,0), (0, -1), (-1, 0), (0, 1)].iter() {
            if grid[(i as i64 + di) as usize][(j as i64 + dj) as usize] == EMPTY {
                if cc_grid[(i as i64 + di) as usize][(j as i64 + dj) as usize] != cc {
                    explore(grid, cc_grid, cc_list, cc_bd, cc, (i as i64 + di) as usize, (j as i64 + dj) as usize)
                }
            } else {
                cc_bd[cc].push(((i as i64 + di) as usize, (j as i64 + dj) as usize))
            }
        }
    }

}
// cherche une nouvelle composante innexplorée à partir de la coordonnée (i,j)
fn find_new(grid: &Vec<Vec<bool>>, cc_grid: &mut Vec<Vec<usize>>, cv_width: usize, cv_height: usize, i: usize, j: usize) -> Option<(usize, usize)> {
    let mut x = i;
    let mut y = j;
    while grid[x][y] == FILLED || cc_grid[x][y] != 0 {
        if x < (cv_width-1) { x += 1 }
        else if y < (cv_height-1) { y += 1; x = 0 }
        else { return None }
    };
    Some((x, y))
}

fn neighbors_grid(cv_width: usize, cv_height: usize, grid: &Vec<Vec<bool>>) 
-> Vec<Vec<u8>> {
    let mut ng = vec![vec![0; cv_height]; cv_width];
    
    for i in 1..(cv_width-1) {
        for j in 1..(cv_height-1) {
            if grid[i][j] {
                ng[i-1][j-1] += 1; ng[i][j-1] += 1; ng[i+1][j-1] += 1;
                ng[i-1][j  ] += 1;                  ng[i+1][j  ] += 1;
                ng[i-1][j+1] += 1; ng[i][j+1] += 1; ng[i+1][j+1] += 1;
            }
        }
    }

    for i in 1..(cv_width-1) {
        ng[i-1][0] += 1;                ng[i+1][0] += 1;
        ng[i-1][1] += 1; ng[i][1] += 1; ng[i+1][1] += 1;

        ng[i-1][cv_height-2] += 1; ng[i][cv_height-2] += 1; ng[i+1][cv_height-2] += 1;
        ng[i-1][cv_height-1] += 1;                          ng[i+1][cv_height-1] += 1;
    }

    for j in 1..(cv_height-1) {
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