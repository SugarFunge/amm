use sp_core::U256;

// pub const MILLICENTS: U256 = 10_000_000_000_000;
// pub const CENTS: U256 = 1_000 * MILLICENTS; // assume this is worth about a cent.
// pub const DOLLARS: U256 = 100 * CENTS;

#[derive(Debug, Default, Clone)]
struct Pair {
    reserves_a: U256,
    reserves_b: U256,
}

fn add_liquidity(pair: &mut Pair, a: U256, b: U256) {
    pair.reserves_a = pair.reserves_a + a;
    pair.reserves_b = pair.reserves_b + b;
}

fn buy_price_a(pair: &Pair, amount_a: U256) -> U256 {
    let numerator = pair
        .reserves_b
        .saturating_mul(amount_a)
        .saturating_mul(U256::from(100000u128));
    let denominator =
        (pair.reserves_a.saturating_sub(amount_a)).saturating_mul(U256::from(100000u128));
    numerator
        .checked_div(denominator)
        // .and_then(|r| r.checked_add(U256::one())) // add 1 to correct possible losses caused by remainder discard
        .unwrap_or(U256::from(0u128))
}

fn buy_price_b(pair: &Pair, amount_b: U256) -> U256 {
    let numerator = pair
        .reserves_a
        .saturating_mul(amount_b)
        .saturating_mul(U256::from(100000u128));
    let denominator =
        (pair.reserves_b.saturating_sub(amount_b)).saturating_mul(U256::from(100000u128));
    numerator
        .checked_div(denominator)
        // .and_then(|r| r.checked_add(U256::one())) // add 1 to correct possible losses caused by remainder discard
        .unwrap_or(U256::from(0u128))
}

fn buy_a(pair: &mut Pair, amount_a: U256) -> U256 {
    let b = buy_price_a(&pair, amount_a);
    pair.reserves_a = pair.reserves_a.saturating_sub(amount_a);
    pair.reserves_b = pair.reserves_b.saturating_add(b);
    b
}

fn buy_b(pair: &mut Pair, amount_b: U256) -> U256 {
    let a = buy_price_b(&pair, amount_b);
    pair.reserves_b = pair.reserves_b.saturating_sub(amount_b);
    pair.reserves_a = pair.reserves_a.saturating_add(a);
    a
}

fn sell_price_a(pair: &Pair, amount_a: U256) -> U256 {
    let numerator = amount_a
        .saturating_mul(pair.reserves_b)
        .saturating_mul(U256::from(100000u128));
    let denominator = amount_a
        .saturating_add(pair.reserves_a)
        .saturating_mul(U256::from(100000u128));
    numerator.checked_div(denominator).unwrap_or(0u128.into())
}

fn sell_price_b(pair: &Pair, amount_b: U256) -> U256 {
    let numerator = amount_b
        .saturating_mul(pair.reserves_a)
        .saturating_mul(U256::from(100000u128));
    let denominator = amount_b
        .saturating_add(pair.reserves_b)
        .saturating_mul(U256::from(100000u128));
    numerator.checked_div(denominator).unwrap_or(0u128.into())
}

fn sell_a(pair: &mut Pair, amount_a: U256) -> U256 {
    let b = sell_price_a(pair, amount_a);
    pair.reserves_a = pair.reserves_a + amount_a;
    pair.reserves_b = pair.reserves_b - b;
    b
}

fn sell_b(pair: &mut Pair, amount_b: U256) -> U256 {
    let a = sell_price_b(pair, amount_b);
    pair.reserves_b = pair.reserves_b + amount_b;
    pair.reserves_a = pair.reserves_a - a;
    a
}

fn main() {
    println!("Hello, AMM!");

    let mut pair = Pair::default();

    for i in 0..3 {
        println!("----------------------------------------");

        add_liquidity(
            &mut pair,
            1000000000000000u128.into(),
            10000000000000u128.into(),
        );
        println!("Liquidity event #{}", i);

        println!("{:#?} k: {}", pair, pair.reserves_a * pair.reserves_b);

        println!(
            "Buy price of {} a: {} b",
            10000000000u128,
            buy_price_a(&pair, 10000000000u128.into())
        );
        println!(
            "Buy price of {} b: {} a",
            10000000000u128,
            buy_price_b(&pair, 10000000000u128.into())
        );

        let cost_b = buy_a(&mut pair, 100000000000u128.into());
        println!("Bought {} a for {} b", 100000000000u128, cost_b);

        println!(
            "Buy price of {} a: {} b",
            10000000000u128,
            buy_price_a(&pair, 10000000000u128.into())
        );
        println!(
            "Buy price of {} b: {} a",
            10000000000u128,
            buy_price_b(&pair, 10000000000u128.into())
        );

        println!(
            "Sell price of {} a: {} b",
            10000000000u128,
            sell_price_a(&pair, 10000000000u128.into())
        );
        println!(
            "Sell price of {} b: {} a",
            10000000000u128,
            sell_price_b(&pair, 10000000000u128.into())
        );

        let cost_b = buy_a(&mut pair, 90000000000000u128.into());
        println!("Bought {} a for {} b", 90000000000000u128, cost_b);
        let cost_a = buy_b(&mut pair, 100000000000u128.into());
        println!("Bought {} b for {} a", 100000000000u128, cost_a);

        println!(
            "Buy price of {} a: {} b",
            10000000000u128,
            buy_price_a(&pair, 10000000000u128.into())
        );
        println!(
            "Buy price of {} b: {} a",
            10000000000u128,
            buy_price_b(&pair, 10000000000u128.into())
        );

        println!(
            "Sell price of {} a: {} b",
            10000000000u128,
            sell_price_a(&pair, 10000000000u128.into())
        );
        println!(
            "Sell price of {} a: {} b",
            100000000000u128,
            sell_price_a(&pair, 100000000000u128.into())
        );

        let gain_a = sell_b(&mut pair, 100000000000u128.into());
        println!("Sold {} b: for {} a", 100000000000u128, gain_a);
        let gain_b = sell_a(&mut pair, 100000000000u128.into());
        println!("Sold {} a: for {} b", 100000000000u128, gain_b);

        println!("{:#?} k: {}", pair, pair.reserves_a * pair.reserves_b);
    }
}
