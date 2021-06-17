use std::io::{Write};
use std::fs::File;

use crossterm::{Result};

pub fn display_grid(cv_width: u16, cv_height: u16, seed: u64, p_filled: f64, 
    nb_iterations: u32, name_ext: &str, grid: Vec<Vec<bool>>, claws: Vec<((usize, usize), (usize, usize))>, sd_grid: Vec<Vec<bool>>) -> Result<()> {

    let file_name: String = 
        "cavern_generator".to_string()
        +&cv_width.to_string()+"_"
        +&cv_height.to_string()+"_"
        +&seed.to_string()+"_"
        +&p_filled.to_string()+"_"
        +&nb_iterations.to_string()+"_"
        +name_ext
        +&".txt".to_string();

    let mut f = File::create(file_name).unwrap();

    f.write("Cavern genrated with \n".as_bytes()).unwrap();
    f.write(("seed: ".to_string()+&seed.to_string()+&"\n".to_string()).as_bytes()).unwrap();
    f.write(("cavern width: ".to_string()+&cv_width.to_string()+&"\n".to_string()).as_bytes()).unwrap();
    f.write(("cavern height: ".to_string()+&cv_height.to_string()+&"\n".to_string()).as_bytes()).unwrap();
    f.write(("probability to be filled at initialization: ".to_string()+&p_filled.to_string()+&"\n".to_string()).as_bytes()).unwrap();
    f.write(("iterations: ".to_string()+&nb_iterations.to_string()+&"\n".to_string()).as_bytes()).unwrap();
    f.write(("name extension: ".to_string()+name_ext+&"\n".to_string()).as_bytes()).unwrap();
    f.write(("filled cells are displayed using: ".to_string()+&'\u{2593}'.to_string()+&"\n".to_string()).as_bytes()).unwrap();
    f.write(("empty cells are displayed using: ".to_string()+&' '.to_string()+&"\n".to_string()).as_bytes()).unwrap();
    f.write(("claws are displayed using: ".to_string()+&'\u{2588}'.to_string()+&"\n".to_string()).as_bytes()).unwrap();
    f.write(("\n".to_string()).as_bytes()).unwrap();
    f.write(("Première grille, avec pinces, injection et coupures : \n\n".to_string()).as_bytes()).unwrap();

    for j in 0..cv_height {
        for i in 0..cv_width {
            let mut c =
                if grid[i as usize][j as usize] { '\u{2593}' } else { ' ' };
            'searching: for claw in claws.iter() {
                if (i as usize, j as usize) == claw.0 || (i as usize, j as usize) == claw.1 {
                    c = '\u{2588}'; break 'searching
                }
            }
            f.write(c.to_string().as_bytes()).unwrap();
        }
        f.write("\n".as_bytes()).unwrap();
    }

    f.write(("\n Seconde grille, doublée en taille : \n\n".to_string()).as_bytes()).unwrap();

    for j in 0..2*cv_height {
        for i in 0..2*cv_width {
            let c =
                if sd_grid[i as usize][j as usize] { '\u{2593}' } else { ' ' };
            f.write(c.to_string().as_bytes()).unwrap();
        }
        f.write("\n".as_bytes()).unwrap();
    }

    Ok(())
}