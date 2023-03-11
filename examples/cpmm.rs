use amm::{cpmm::*, LiquidityPool};
use rust_decimal::Decimal;

fn main() {
    let mut pool = ConstantProductMarketMaker::default();
    pool.add_liquidity(Decimal::from(100), Decimal::from(100));
    println!("pool: {:?}", pool);

    let amount = pool.swap_a(Decimal::from(10));
    println!("swap_a: {}", 10);
    println!("cost_b: {}", amount);
    println!("pool: {:?}", pool);
}
