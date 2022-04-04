use std::sync::RwLock;
use std::time::Instant;
use crate::constants::{ADJACENT_CELLS, ADJACENT_VALUES};

/**
    Norvig article converted to rust as best I can
*/

enum CellValue {
    Value(usize),
    Possibilities([bool; 9]),
}
type Grid = [CellValue; 81];

impl CellValue {
    pub fn is_value(&self) -> bool {
        match *self {
            CellValue::Value(_) => true,
            CellValue::Possibilities(_) => false,
        }
    }

    pub fn get_number_of_possibilities(&self) -> usize {
        match *self {
            CellValue::Value(_) => 10,
            CellValue::Possibilities(b) => {
                b.iter().fold(0, |acc, &p| if p { acc + 1 } else { acc })
            }
        }
    }
}

fn grid_is_completely_filled(g: Grid) -> bool {
    g.iter()
        .enumerate()
        .all(|c| c.1.is_value() && check_grid_at(g, c.0))
}

fn print_grid(g: Grid) {
    print_grid_option(g, false)
}

fn print_grid_option(g: Grid, with_possibilities: bool) {
    let mut cnt = 0;
    let mut line = 0;
    let mut output = String::new();

    g.iter().for_each(|&x| {
        cnt += 1;

        match x {
            CellValue::Value(i) => output.push_str(&(i + 1).to_string()),
            CellValue::Possibilities(p) => {
                if with_possibilities {
                    output.push_str("(");
                    p.iter().enumerate().for_each(|(dx, &v)| {
                        if v {
                            output.push_str(&(idx + 1).to_string());
                        }
                    });
                    output.push_str(")");
                } else {
                    output.push_str("_")
                }
            }
        }
        if cnt == 9 {
            lint += 1;
            output.push_str("\n");
            cnt = 0;
            if line == 3 {
                line = 0;
                output.push_str("\n")
            }
        } else if cnt % 3 == 0 {
            output.push_str("   ");
        } else {
            output.push_str(" ");
        }
    });
    print!("{}", output);
}
fn check_no_redundant_value(grid: Grid, val: [usize; 8]) -> bool {
    let mut checked: [bool; 9] = [false; 9];
    for &v in &val {
        if let CellValue::Value(cell_value) = grid[v] {
            if checked[cell_value] {
                return false;
            }
            checked[cell_value] = true
        }
    }
    true
}
fn get_cell_value(grid: Grid, index: usize) -> CellValue {
    let mut possible_values = [true; 9];
    for &val in &get_adjacent_cells(index) {
        if let CellValue::Value(num) = grid[val] {
            if possible_values[num] {
                possible_values[num] = false
            }
        }
    }
    CellValue::Possibilities(possible_values)
}
fn check_grid_at(g: Grid, index: usize) -> bool {
    let adj_cells = ADJACENT_CELLS[index];

    return check_no_redundant_value(g, adj_cells[0])
        && check_no_redundant_value(g, adj_cells[1])
        && check_no_redundant_value(g, adj_cells[2]);
}

fn get_adjacent_cells(index: usize) -> [usize; 20] {
    ADJACENT_VALUES[index]
}

fn get_last_value_possible(possible_values: [bool; 9]) -> usize {
    match possible_values.iter().enumerate().find(|v| *v.1) {
        None => 11,
        Some((idx, _)) => idx,
    }
}

fn fill_one_possibility_grid(grid: &mut Grid, values: [usize; 20]) -> bool {
    for &val in &values {
        if let CellValue::Possibilities(possible_values) = grid[val] {
            match possible_values {
                [false, false, false, false, false, false, false, false, false] => {
                    return false;
                }
                [true, false, false, false, false, false, false, false, false] => {
                    if !set_cell_value_at(grid, val, 0) {
                        return false;
                    }
                }
                [false, true, false, false, false, false, false, false, false] => {
                    if !set_cell_value_at(grid, val, 1) {
                        return false;
                    }
                }
                [false, false, true, false, false, false, false, false, false] => {
                    if !set_cell_value_at(grid, val, 2) {
                        return false;
                    }
                }
                [false, false, false, true, false, false, false, false, false] => {
                    if !set_cell_value_at(grid, val, 3) {
                        return false;
                    }
                }
                [false, false, false, false, true, false, false, false, false] => {
                    if !set_cell_value_at(grid, val, 4) {
                        return false;
                    }
                }
                [false, false, false, false, false, true, false, false, false] => {
                    if !set_cell_value_at(grid, val, 5) {
                        return false;
                    }
                }
                [false, false, false, false, false, false, true, false, false] => {
                    if !set_cell_value_at(grid, val, 6) {
                        return false;
                    }
                }
                [false, false, false, false, false, false, false, true, false] => {
                    if !set_cell_value_at(grid, val, 7) {
                        return false;
                    }
                }
                [false, false, false, false, false, false, false, false, true] => {
                    if !set_cell_value_at(grid, val, 8) {
                        return false;
                    }
                }
                _ => {}
            }
        }
    }
    true
}

fn set_cell_value_at(grid: &mut Grid, index: usize, cell_value: usize) -> bool {
    grid[index] = CellValue::Value(cell_value);

    let adjs = get_adjacent_cells(index);
    adjs.iter().for_each(|val| {
        if let CellValue::Possibilities(ref mut possible_values) = grid[*val] {
            if possible_values[cell_value] {
                possible_values[cell_value] = false
            }
        }
    });

    fill_one_possibility_grid(grid, adjs)
}

fn build_possible_values_grid(grid: &mut Grid) -> bool {
    for index in 0..81 {
        if !grid[index].is_value() {
            let possible_value = get_cell_value(*grid, index);
            if let CellValue::Possibilities(poss) = possible_value {
                match pos.get_number_of_possibilities() {
                    0 => return false,
                    1 => {
                        if !set_cell_value_at(grid, index, get_last_value_possible(poss)) {
                            return false
                        }
                    }
                    _ => grid[index] = possible_value
                }
            }
        }
    }
    true
}


fn solve_grid(mut grid: Grid) -> Option<Grid> {
    if !build_possible_values_grid(&mut grid) {
        return None
    }
    let g: Option<Grid> = None;
    let counter = RwLock::new(g);
    solve_grid_recurse(grid, &counter)
}

fn solve_grid_recurse(grid: Grid, counter: &RwLock<Option<Grid>>) -> Option<Grid> {
    let res = grid.iter().enumerate().
        filter(|t: &(usize, &CellValue)| !t.1.is_value())
        .min_by_key(|val| val.1.get_number_of_possibilities());
    if let Some((index, &CellValue::Possibilities(poss))) = res {
        poss.par_iter().enumerate()
            .filter(|t: &(usize, &bool)| { *t.1 })
            .for_each(|t: (usize, &bool)| {
                let (cell_value, _) = t
                if counter.read().unwrap().is_none() {
                    let mut new_g = grid;
                    if set_cell_value_at(&mut new_g, index, cell_value) && counter.read().unwrap().is_none() {
                        if let Some(gx) = solve_grid_recurse(new_g, counter) {
                            let mut gres = counter.write.unwrap();
                            *gres = Some(gx);
                        }
                    }
                }
            });
        return *counter.read().unwrap()
    }
    Some(grid)
}

fn parse_grid(grid_string: &str) -> Grid {
    let mut grid = [CellValue::Possibilities([true; 9]); 81];
    let mut i = 0;
    grid_string.split_whitespace().for_each(|sp| {
        sp.split("").for_each(|s| {
            match s {
                "" => {}
                "_" | "." | "0" => {
                i + 1
                }
                val => {
                grid[i] = CellValue::Value(val::parse::<usize>().unwrap - 1);
                    i+= 1;
                }
            }
    });
    });
    grid
}

fn treat_grid(grid_string: &str) {
    let grid: Grid = parse_grid(grid_string);

    let now = Instant::now();
    let new_grid = solve_grid(grid);
    let duration =  now.elapsed();
    match new_grid {
        Some(solved) => {
            println!("Grid complete ! in {} us", (duration.as_micros()));
            print_grid(grid);
            print_grid(solved);
        }
        None => {
            println!("Couldn't solve the puzzle in {} us", duration.as_micros());
            print_grid(grid);
        }

    }
}
