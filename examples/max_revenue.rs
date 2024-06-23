// 寻找最大收益
fn main() {
    // 股票价格波动表
    let stock = [23, 42, 65, 120, 55, 10, 190, 44];

    let mut min_price = stock[0];
    let mut max_revenue = i32::MIN;

    for price in stock {
        if price <= min_price {
            min_price = price;
        } else if max_revenue < price - min_price {
            max_revenue = price - min_price
        }
    }
    println!("max revenue: {max_revenue}");
}
