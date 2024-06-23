use anyhow::{anyhow, Result};
use num::Bounded;
use std::ops::Sub;

// 寻找最大收益
fn main() -> Result<()> {
    // 股票价格波动表
    let mut stock_array = [10, 42, 9, 120, 55, 8, 190, 44];
    // let mut stock_array = [10];
    let max_revenue = cal_max_revenue::<i32>(&mut stock_array)?;
    println!("max revenue: {max_revenue}");
    Ok(())
}

fn cal_max_revenue<T>(stock_array: &mut [T]) -> Result<T>
where
    T: Ord + Copy + Bounded + Sub<Output = T>,
{
    if stock_array.len() <= 1 {
        return Err(anyhow!("stock array len must bigger than 1"));
    }

    let mut min_price = stock_array[0];
    let mut max_revenue = T::min_value();

    for price in stock_array {
        if price <= &mut min_price {
            min_price = *price;
        } else if max_revenue < *price - min_price {
            max_revenue = *price - min_price
        }
    }
    Ok(max_revenue)
}
