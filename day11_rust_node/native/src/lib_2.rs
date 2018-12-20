#[macro_use]
extern crate neon;
extern crate rayon;

use neon::prelude::*;
use rayon::prelude::*;


const SERIAL_NUMBER: usize = 1788;
const GRID_SIZE: usize = 300;

fn solve(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    println!("Part 2 optimized: {:?}", part2(SERIAL_NUMBER, GRID_SIZE));
    Ok(cx.undefined())
}


fn nth_digit(number: usize, idx: usize) -> Option<usize> {
    let mut n = number;
    let mut i = 1;

    while n > 0 {
        n = n / 10;
        if i == idx {
            return Some(n % 10);
        }

        i += 1;
    }

    None
}

pub fn power(grid: &Vec<Vec<i64>>, location: &(usize, usize), window_size: usize) -> i64 {
    (location.0..(location.0 + window_size))
        .map(|x| (location.1..(location.1 + window_size)).fold(0, |acc, y| acc + grid[x][y]))
        .sum()
}

pub fn build_grid(serial: usize, size: usize) -> Vec<Vec<i64>> {
    (0..size)
        .map(|x| {
            (0..size)
                .map(|y| {
                    let rack_id = x + 1 + 10;

                    let interim = (rack_id * (y + 1) + serial) * rack_id;
                    (nth_digit(interim, 2).unwrap_or(0) as i64) - 5
                }).collect()
        }).collect()
}

pub fn part2(serial: usize, size: usize) -> (usize, usize, usize) {
    let grid = build_grid(serial, size);

    let (power, (x, y), final_size) = (0..size)
        .into_par_iter()
        .map(|window| {
            (0..size - window)
                .flat_map(|x| {
                    (0..size - window)
                        .clone()
                        .map(|y| {
                            let coordinate = (x, y);
                            return (power(&grid, &coordinate, window), coordinate);
                        }).collect::<Vec<(i64, (usize, usize))>>()
                }).max_by(|(a, _), (b, _)| a.cmp(b))
                .and_then(|(power, (x, y))| Some((power, (x + 1, y + 1), window)))
                .unwrap()
        }).max_by(|(a, _, _), (b, _, _)| a.cmp(b))
        .unwrap();

    (x, y, final_size)
}

register_module!(mut cx, {
    cx.export_function("solve", solve)
});
