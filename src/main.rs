use rand::Rng;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

fn main() {
    print_noise(gen_noise(55, 20, 4, 4, 100, 100));
}

fn gen_noise(
    percentage: i32,
    smoothing_level: usize,
    birth_limit: usize,
    death_limit: usize,
    height: usize,
    width: usize,
) -> Vec<Vec<bool>> {
    //we populate it
    let mut filled_vec = (0..height + 3)
        .into_par_iter()
        .map(|x| {
            let mut rng = rand::thread_rng();
            (0..width + 3)
                .into_iter()
                .map(|y| {
                    if x == 0 || y == 0 || x == width + 2 || y == height + 2 {
                        return false;
                    }
                    rng.gen_range(0..100) < percentage
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    //then we smoothe it
    (0..smoothing_level).into_iter().for_each(|_| {
        let replace_vec = (0..width + 3)
            .into_par_iter()
            .map(|x| {
                (0..height + 3)
                    .into_iter()
                    .map(|y| {
                        if x == 0 || y == 0 || x == width + 2 || y == height + 2 {
                            return false;
                        }
                        let sum: usize = filled_vec[x - 1][y - 1] as usize
                            + filled_vec[x - 1][y] as usize
                            + filled_vec[x - 1][y + 1] as usize
                            + filled_vec[x][y - 1] as usize
                            + filled_vec[x][y + 1] as usize
                            + filled_vec[x + 1][y - 1] as usize
                            + filled_vec[x + 1][y] as usize
                            + filled_vec[x + 1][y + 1] as usize;

                        if sum < death_limit && filled_vec[x][y] {
                            return false;
                        }
                        if sum > birth_limit && !filled_vec[x][y] {
                            return true;
                        }
                        filled_vec[x][y]
                    })
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>();
        filled_vec = replace_vec;
    });

    //then we cut it
    (1..width + 1)
        .into_par_iter()
        .map(|x| {
            (1..height + 1)
                .into_iter()
                .map(|y| filled_vec[x][y])
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>()
}

fn print_noise(noise: Vec<Vec<bool>>) {
    for row in noise {
        for cell in row {
            if cell {
                print!("X");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}
