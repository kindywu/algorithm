use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    // let test_arr = [(9, 3, 3)];
    // let test_arr = [(15, 3, 5)];

    let test_arr = [(9, 3, 3), (15, 3, 5), (15, 5, 3), (100, 25, 4)];
    for (len, rows, cols) in test_arr {
        let spiral_array = generate_spiral_array(len, rows, cols)?;
        println!("spiral array: {len}, {rows}, {cols}");
        print_spiral_array(spiral_array);
    }
    Ok(())
}

// #[allow(clippy::needless_range_loop)]
fn generate_spiral_array(len: usize, rows: usize, cols: usize) -> Result<Vec<Vec<usize>>> {
    if len != rows * cols {
        return Err(anyhow!("len {len} must equal rows {rows} * cols {cols}"));
    }
    let mut arr = vec![vec![0; cols]; rows];
    let (mut top, mut bottom, mut left, mut right) = (0, rows - 1, 0, cols - 1);
    let mut num = 1;
    while top <= bottom && left <= right {
        for i in left..=right {
            arr[top][i] = num;
            num += 1;
        }
        top += 1;

        #[allow(clippy::needless_range_loop)]
        for i in top..=bottom {
            arr[i][right] = num;
            num += 1;
        }
        right -= 1;

        if top <= bottom {
            for i in (left..=right).rev() {
                arr[bottom][i] = num;
                num += 1;
            }
            bottom -= 1;
        } else {
            break;
        }

        if left <= right {
            for i in (top..=bottom).rev() {
                arr[i][left] = num;
                num += 1;
            }
            left += 1;
        } else {
            break;
        }
    }
    Ok(arr)
}

fn print_spiral_array(spiral_array: Vec<Vec<usize>>) {
    for row in spiral_array {
        for col in row {
            print!("{col:5}");
        }
        println!()
    }
}

#[allow(unused)]
fn generate_spiral_array2(rows: usize, cols: usize) -> Result<Vec<Vec<usize>>> {
    let len = rows * cols;
    generate_spiral_array(len, rows, cols)
}
