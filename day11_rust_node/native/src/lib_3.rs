#[macro_use]
extern crate neon;
extern crate rayon;

use neon::prelude::*;
use rayon::prelude::*;

fn get_row_major_order_idx(x: usize, y: usize, width: usize) -> usize {
    return width * y + x;
}

struct SummedAreaTable {
    grid_size: usize,
    inner_array: Vec<i32>,
}

// https://en.wikipedia.org/wiki/Summed-area_table
impl SummedAreaTable {
    fn new(grid_size: usize, grid_serial_number: i32) -> SummedAreaTable {
        assert!(grid_size > 0);

        let area = (grid_size * grid_size) as usize;

        let mut inner_array = vec![0; area];

        for x in 0..grid_size {
            for y in 0..grid_size {
                let normalized_x = (x + 1) as i32;
                let normalized_y = (y + 1) as i32;

                let power_level = get_power_level(normalized_x, normalized_y, grid_serial_number);

                let index = get_row_major_order_idx(x, y, grid_size);
                inner_array[index] = power_level;
            }
        }

        for x in 0..grid_size {
            for y in 0..grid_size {
                let mut sum: i32 = inner_array[get_row_major_order_idx(x, y, grid_size)];

                if x > 0 {
                    sum += inner_array[get_row_major_order_idx(x - 1, y, grid_size)];
                }

                if y > 0 {
                    sum += inner_array[get_row_major_order_idx(x, y - 1, grid_size)];
                }

                if x > 0 && y > 0 {
                    sum -= inner_array[get_row_major_order_idx(x - 1, y - 1, grid_size)];
                }

                inner_array[get_row_major_order_idx(x, y, grid_size)] = sum;
            }
        }

        SummedAreaTable {
            grid_size: grid_size,
            inner_array: inner_array,
        }
    }

    fn get_spanned_square(&self, x: usize, y: usize, sub_grid_size: usize) -> i32 {
        assert!(x < self.grid_size);
        assert!(y < self.grid_size);
        assert!(sub_grid_size <= self.grid_size);
        assert!(x + sub_grid_size <= self.grid_size);

        let top_left = if x > 0 && y > 0 {
            self.inner_array[get_row_major_order_idx(x - 1, y - 1, self.grid_size)]
        } else {
            0
        };

        let end_x = sub_grid_size + x - 1;
        let end_y = sub_grid_size + y - 1;

        let top_right = if y > 0 {
            self.inner_array[get_row_major_order_idx(end_x, y - 1, self.grid_size)]
        } else {
            0
        };

        let bottom_left = if x > 0 {
            self.inner_array[get_row_major_order_idx(x - 1, end_y, self.grid_size)]
        } else {
            0
        };

        let bottom_right = self.inner_array[get_row_major_order_idx(end_x, end_y, self.grid_size)];

        return bottom_right + top_left - top_right - bottom_left;
    }
}

fn get_power_level(x: i32, y: i32, grid_serial_number: i32) -> i32 {
    assert!(x > 0);
    assert!(y > 0);

    let rack_id = x + 10;
    let power_level = rack_id * y;
    let power_level = power_level + grid_serial_number;
    let power_level = power_level * rack_id;

    let power_level = power_level.to_string();

    assert!(power_level.len() >= 3);

    let skip_n = power_level.len() - 3;

    let power_level: i32 = power_level
        .chars()
        .skip(skip_n)
        .next()
        .unwrap()
        .to_string()
        .parse()
        .unwrap();

    return power_level - 5;
}

fn get_total_power_level_of_square(
    start_x: i32,
    start_y: i32,
    size: i32,
    grid_serial_number: i32,
) -> i32 {
    let end_x = size + start_x - 1;
    let end_y = size + start_y - 1;

    let x_range: Vec<i32> = (start_x..=end_x).into_iter().collect();
    let y_range: Vec<i32> = (start_y..=end_y).into_iter().collect();

    let total: i32 = x_range
        .into_par_iter()
        .map(|x| -> i32 {
            let result: i32 = y_range
                .par_iter()
                .map(|y| -> i32 {
                    return get_power_level(x, *y, grid_serial_number);
                })
                .sum();

            return result;
        })
        .sum();

    return total;
}

fn part_2(summed_area_table: &SummedAreaTable) -> ((usize, usize), usize) {
    let width = summed_area_table.grid_size;
    let height = summed_area_table.grid_size;

    let sub_grid_size_range: Vec<usize> = (1..=summed_area_table.grid_size).into_iter().collect();

    let result = sub_grid_size_range
        .into_par_iter()
        .map(|sub_grid_size: usize| {
            let mut largest_total_power = 0;
            let mut best_position = None;

            for x in 1..=(width - sub_grid_size + 1) {
                for y in 1..=(height - sub_grid_size + 1) {
                    let total = summed_area_table.get_spanned_square(x - 1, y - 1, sub_grid_size);
                    if total > largest_total_power {
                        largest_total_power = total;
                        best_position = Some((x, y));
                    }
                }
            }

            return (best_position, largest_total_power, sub_grid_size);
        })
        .max_by_key(|x| {
            let (_position, total, _sub_grid_size) = x;
            return total.clone();
        })
        .unwrap();

    let (best_position, _total, best_sub_grid_size) = result;

    return (best_position.unwrap(), best_sub_grid_size);
}

fn solve(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let grid_size = 1000;
    let grid_serial_number = 1788;
    let sub_grid_size: usize = 3;

    let summed_area_table = SummedAreaTable::new(grid_size, grid_serial_number);

    let (position, grid_size) = part_2(&summed_area_table);
    println!("Part 2 super optimized: {:?} {}", position, grid_size);

    Ok(cx.undefined())
}

register_module!(mut cx, {
    cx.export_function("solve", solve)
});
