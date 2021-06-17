pub mod display;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;
use dmsort::sort_by;

const FILLED: bool = true;
const EMPTY: bool = false;

pub fn generate_cavern(cv_width: usize, cv_height: usize, seed_u64: u64, p_filled: f64, n_iterations: u32) 
-> (Vec<Vec<bool>>, Vec<((usize, usize), (usize, usize))>, Vec<Vec<bool>>) {

    let mut grid: Vec<Vec<bool>> = 
        vec![vec![false; cv_height]; cv_width];
    let mut random_generator: StdRng = StdRng::seed_from_u64(seed_u64);

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
    // cc est numéro de la composante connexe en cours de traitement, 0 est réservé pour les cases non traitées!
    let mut cc: usize = 0;
    // Nombre d'essais maximum à éffectuer pour découper une composante connexe
    let nb_try = 200;
    // On retient les pinces trouvées qui ont permis de scinder les composantes connexes trop grandes
    let mut claws: Vec<((usize, usize), (usize, usize))> = Vec::new();

    while let Some((x, y)) = find_new(&grid, &mut cc_grid, cv_width, cv_height, 0, 0) {
        // println!("x: {}, y: {}", x, y);
        cc += 1;
        cc_list[0][0].0 += 1;
        cc_list.push(Vec::new());
        cc_bd.push(Vec::new());
        // i = x;
        // j = y;
        explore(&grid, &mut cc_grid, &mut cc_list, &mut cc_bd, cc, x, y);
        // On injecte si la taille de la cc est <= 12
        let size = cc_list[cc].len();
        if size <= 12 {
            for cell in cc_list[cc].iter() {
                grid[cell.0][cell.1] = FILLED
            };
            cc -= 1;
            cc_list.pop();
            cc_bd.pop();
        } 
        // On tente de découper si la composante est grande (>= 100 cellules)
        else {
            'cutting: while cc_list[cc].len() >= 100 {
                let succeed: bool = try_to_cut(&mut random_generator, &mut claws, &mut grid, &mut cc_grid, &mut cc_list, &mut cc_bd, cc, cv_width, cv_height, nb_try);
                if !succeed { break 'cutting }
            }
        }
    };

    claws.dedup();

    // On dédouble la grille en taille
    let mut sd_grid: Vec<Vec<bool>> = vec![vec![false; cv_height*2]; cv_width*2];
    for x in 0..cv_width {
        for y in 0..cv_height {
            sd_grid[2*x][2*y] = grid[x][y];
            sd_grid[2*x + 1][2*y] = grid[x][y];
            sd_grid[2*x][2*y + 1] = grid[x][y];
            sd_grid[2*x + 1][2*y + 1] = grid[x][y]
        }
    };

    // Puis on réeffectue une itération de l'automate cellulaire pour addoucir les coins formés par l'opération qui précède
    let nb = neighbors_grid(2*cv_width, 2*cv_height, &sd_grid);
    for i in 1..(2*cv_width-1) {
        for j in 1..(2*cv_height-1) {
            if sd_grid[i][j] && nb[i][j] < 4 { // /!\ Règles standards : < 4, rules_4 : <= 4
                sd_grid[i][j] = EMPTY
            } else if nb[i][j] >= 5 { // /!\ Règles standards : >= 5, rules_4 : >= 6
                sd_grid[i][j] = FILLED
            }
        }
    };

    // Ensuite on calcule à nouveau les composantes connexes de la grille étendue.
    let mut cc_grid: Vec<Vec<usize>> = vec![vec![0; 2*cv_height as usize]; 2*cv_width as usize];
    // la première composante de la première case de cc_list contient le nombre de composantes connexes, les cases suivantes contiennent la iste des cases d'un composante donnée
    let mut cc_list: Vec<Vec<(usize,usize)>> = vec![vec![(0,0)]];
    // la cc-ème case de cc_bd contient la frontière de la cc-ème compoante connexe du graphe
    let mut cc_bd: Vec<Vec<(usize, usize)>> = vec![vec![]];
    // cc est numéro de la composante connexe en cours de traitement, 0 est réservé pour les cases non traitées!
    let mut cc: usize = 0;
    // On a plus besoin de repartir de (0, 0) pour cherche les composantes car on ne fait pas d'opération sur la carte, donc on stocke la position (i, j) à laquelle on cherche une
    // nouvelle composante connexe
    let mut i = 0;
    let mut j = 0;

    while let Some((x, y)) = find_new(&sd_grid, &mut cc_grid, 2*cv_width, 2*cv_height, i, j) {
        // println!("x: {}, y: {}", x, y);
        cc += 1;
        cc_list[0][0].0 += 1;
        cc_list.push(Vec::new());
        cc_bd.push(Vec::new());
        i = x;
        j = y;
        explore(&sd_grid, &mut cc_grid, &mut cc_list, &mut cc_bd, cc, x, y);
    }

    // Puis on creuse des tunnels entre les différentes composantes connexes de la carte
    let mut linked: Vec<Vec<bool>> = vec![vec![false; cc+1]; cc+1];
    for i in 0..=cc {
        linked[i][i] = true
    };

    let mut to_supress: Vec<(usize, usize)> = Vec::new();
    
    for c in 1..=cc {
        let mut dist: Vec<Vec<i64>> = vec![vec![-1; 2*cv_height]; 2*cv_width];
        for cell in cc_list[c].iter() { dist[cell.0][cell.1] = 0 };
        for cell in cc_bd[c].iter() { dist[cell.0][cell.1] = 1 };
        let mut current: Vec<(usize, usize)> = cc_bd[c].clone();
        let mut next: Vec<(usize, usize)> = Vec::new();
        let mut d = 2;
        let mut reached: Vec<(usize, usize)> = Vec::new();
        let mut n_reached: usize = 0;

        'reaching: while n_reached < 2 {
            if current.is_empty() {
                break 'reaching
            }
            for cell in current.iter() {
                for (di, dj) in [(1, 0), (0, -1), (-1, 0), (0, 1)].iter() {
                    let ni: i64 = cell.0 as i64 + di;
                    let nj: i64 = cell.1 as i64 + dj;
                    if 0 <= ni && ni < 2*(cv_width as i64)
                    && 0 <= nj && nj < 2*(cv_height as i64)
                    && dist[ni as usize][nj as usize] == -1 {
                        next.push((ni as usize, nj as usize));
                        dist[ni as usize][nj as usize] = d;
                        // On regarde si on vient de tomber dans une nouvelle composante connexe
                        if cc_grid[ni as usize][nj as usize] != c && sd_grid[ni as usize][nj as usize] == EMPTY {
                            let mut new_cc = true;
                            for cell_reached in reached.iter(){
                                if cc_grid[cell_reached.0][cell_reached.1] == cc_grid[ni as usize][nj as usize] {
                                    new_cc = false
                                }
                            };
                            let c_reached = cc_grid[ni as usize][nj as usize];
                            if new_cc && !linked[c][c_reached] {

                                reached.push((ni as usize, nj as usize));
                                
                                linked[c][c_reached] = true;
                                linked[c_reached][c] = true;

                                n_reached += 1;
                            
                            } else if new_cc {
                                n_reached += 1;

                            }
                            if n_reached >= 2 { break 'reaching }
                        }
                    }
                }
            };
            current = next.clone();
            next = Vec::new();
            d += 1;

            // On insert dans to_supress les cellules à supprimer pour relier les différentes composantes trouvées lors de l'exploration précédente.
            for cell_reached in reached.iter() {
                let mut i = cell_reached.0 as i64;
                let mut j = cell_reached.1 as i64;
                let mut current_dist = dist[i as usize][j as usize];
                while cc_grid[i as usize][j as usize] != c {
                    let mut di = -1i64;
                    let mut dj = -1i64;
                    while dist[(i + di) as usize][(j + dj) as usize] >= current_dist
                                || dist[(i + di) as usize][(j + dj) as usize] == (-1) {
                        if di < 1 {
                            di += 1;
                        } else if dj < 1 {
                            di = -1;
                            dj +=1;
                        } else {
                            panic!("didn't succend at digging a path from ({}, {}) to the {}-th component", cell_reached.0, cell_reached.1, c);
                        };
                    };
                    i += di;
                    to_supress.push((i as usize, j as usize));
                    j += dj;
                    to_supress.push((i as usize, j as usize));
                    current_dist = dist[i as usize][j as usize];
                }
            }
        }
    };

    for cell in to_supress.iter() {
        sd_grid[cell.0][cell.1] = EMPTY
    };

    (grid, claws, sd_grid)
}

fn try_to_cut(random_generator: &mut StdRng, claws: &mut Vec<((usize, usize), (usize, usize))>, grid: &mut Vec<Vec<bool>>, cc_grid: &mut Vec<Vec<usize>>, 
    cc_list: &mut Vec<Vec<(usize,usize)>>, cc_bd: &mut Vec<Vec<(usize, usize)>>, cc: usize, cv_width: usize, cv_height: usize, nb_try: usize)
-> bool {
    // On découpe les grandes composantes sous reserve de conserver des composantes assez grande après la coupe.
    let mut claws_aux: Vec<((usize, usize), (usize, usize))> = Vec::new();
    let bd = &cc_bd[cc];
    let len = bd.len();
    for _ in 0..nb_try {
        let i = random_generator.gen_range(0..len);
        let (x1, y1) = (bd[i].0, bd[i].1);
        let j = random_generator.gen_range(0..len);
        let (x2, y2) = (bd[j].0, bd[j].1);
        
        let ((x1, y1), (x2, y2)) = claw(&grid, cv_width, cv_height, (x1 as i64, y1 as i64), (x2 as i64, y2 as i64));
        let d = dist2_sqr((x1 as i64, y1 as i64), (x2 as i64, y2 as i64));

        if d < 25 && d > 0 {
            claws_aux.push( ((x1 as usize, y1 as usize), (x2 as usize, y2 as usize)) )
        }
    };
    claws_aux.dedup();
    fn cmp_by_dist(a: &((usize, usize), (usize, usize)), b: &((usize, usize), (usize, usize))) -> std::cmp::Ordering {
        let (a0x, a0y) = a.0; let (a1x, a1y) = a.1; let (b0x, b0y) = b.0; let (b1x, b1y) = b.1;
        dist2_sqr((a0x as i64, a0y as i64), (a1x as i64, a1y as i64)).cmp(&dist2_sqr((b0x as i64, b0y as i64), (b1x as i64, b1y as i64)))
    }
    sort_by(&mut claws_aux, cmp_by_dist);

    //println!("found {} claws:", claws_aux.len());
    'trying_claws: for claw in claws_aux.iter() {
        //println!("trying to cut {}-th cc with ({}, {}), ({}, {}) at distance {}", 
        //    cc, claw.0.0, claw.0.1, claw.1.0, claw.1.1, dist2_sqr((claw.0.0 as i64, claw.0.1 as i64), (claw.1.0 as i64, claw.1.1 as i64)));
        
        let mut modifs: Vec<(usize, usize)> = Vec::new();

        // On remplie les cases se trouvant sur le plus court chemin reliant i à j.
        let mut i = claw.0.0;
        let mut j = claw.0.1;
        while (i,j) != claw.1 {
            if claw.1.0 > i {
                i += 1;
            } else if claw.1.0 < i {
                i -= 1;
            };
            if grid[i][j] == EMPTY {
                grid[i][j] = FILLED;
                modifs.push((i, j))
            };
            if claw.1.1 > j {
                j += 1;
            } else if claw.1.1 < j {
                j -= 1;
            };
            if grid[i][j] == EMPTY {
                grid[i][j] = FILLED;
                modifs.push((i,j))
            }
        };

        let mut cutted_wrong_component = false;
        for cell in modifs.iter() {
            if cc_grid[cell.0][cell.1] != cc {
                cutted_wrong_component = true
            }
        };
        if cutted_wrong_component {
            for cell in modifs.iter() {
                grid[cell.0][cell.1] = EMPTY
            };
            //println!("oups, cutted wrong component! aborting this attempt");
            continue 'trying_claws
        };

        // On vérifie que l'on a pas formé des composantes trop petites.
        for cell in cc_list[cc].iter() {
            cc_grid[cell.0][cell.1] = 0
        };
        let mut cc_aux = 0;
        let mut cc_list_aux: Vec<Vec<(usize,usize)>> = vec![vec![(0,0)]];
        let mut cc_bd_aux: Vec<Vec<(usize, usize)>> = vec![vec![]];

        for cell in cc_list[cc].iter() {
            if cc_grid[cell.0][cell.1] == 0 && grid[cell.0][cell.1] == EMPTY {
                cc_aux += 1;
                cc_list_aux[0][0].0 += 1;
                cc_list_aux.push(Vec::new());
                cc_bd_aux.push(Vec::new());
                explore(&grid, cc_grid, &mut cc_list_aux, &mut cc_bd_aux, cc_aux, cell.0, cell.1);
                if cc_list_aux[cc_aux].len() <= 12 {
                    //println!("didn't succeed with this claw");
                    for cell in cc_list[cc].iter() {
                        cc_grid[cell.0][cell.1] = cc
                    };
                    for cell in modifs.iter() {
                        grid[cell.0][cell.1] = EMPTY
                    };
                    continue 'trying_claws
                }
            }
        };

        // if cc_list_aux[0][0].0 <= 1 {
        //     println!("didn't succeed with this claw");
        //     for cell in cc_list[cc].iter() {
        //         cc_grid[cell.0][cell.1] = cc
        //     };
        //     for cell in modifs.iter() {
        //         grid[cell.0][cell.1] = EMPTY
        //     };
        //     continue 'trying_claws
        // }

        for cell in cc_list[cc].iter() {
            cc_grid[cell.0][cell.1] = 0
        };
        for cell in cc_list_aux[1].iter() {
            cc_grid[cell.0][cell.1] = cc
        };
        cc_list[cc] = cc_list_aux[1].clone();
        cc_bd[cc] = cc_bd_aux[1].clone();
        //println!("Succeed at cutting with this claw!");
        //println!("Cutted into {} parts", cc_list_aux[0][0].0);
        claws.push(*claw);
        return true
    };
    
    //println!("didn't suceed at cutting {}-th component", cc);
    false
}

fn claw(grid: &Vec<Vec<bool>>, cv_width: usize, cv_height: usize, (x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> ((i64, i64), (i64, i64)) {
    let d = dist2_sqr((x1, y1), (x2, y2));
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
                            //let d_try = dist1((nx1, ny1), (nx2, ny2));
                            let d_try = dist2_sqr((nx1, ny1), (nx2, ny2));
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

fn _dist1((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (x2 - x1).abs() + (y2 - y1).abs()
}
fn dist2_sqr((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (x2 - x1)*(x2 - x1) + (y2 - y1)*(y2 - y1)
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