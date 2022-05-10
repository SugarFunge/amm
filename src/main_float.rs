#[derive(Debug, Default, Clone)]

struct Pair {
    reserves_a: f32,
    reserves_b: f32,
}

fn add_liquidity(pair: &mut Pair, a: f32, b: f32) {
    pair.reserves_a = pair.reserves_a + a;
    pair.reserves_b = pair.reserves_b + b;
}

fn buy_price_a(pair: &Pair, amount_a: f32) -> f32 {
    let numerator = pair.reserves_b * amount_a * 1000.0;
    let denominator = (pair.reserves_a - amount_a) * 1000.0;
    numerator / denominator
}

fn buy_price_b(pair: &Pair, amount_b: f32) -> f32 {
    let numerator = pair.reserves_a * amount_b * 1000.0;
    let denominator = (pair.reserves_b - amount_b) * 1000.0;
    numerator / denominator
}

fn buy_a(pair: &mut Pair, amount_a: f32) -> f32 {
    let b = buy_price_a(&pair, amount_a);
    pair.reserves_a = pair.reserves_a - amount_a;
    pair.reserves_b = pair.reserves_b + b;
    b
}

fn buy_b(pair: &mut Pair, amount_b: f32) -> f32 {
    let a = buy_price_b(&pair, amount_b);
    pair.reserves_b = pair.reserves_b - amount_b;
    pair.reserves_a = pair.reserves_a + a;
    a
}

fn sell_price_a(pair: &Pair, amount_a: f32) -> f32 {
    let numerator = amount_a * pair.reserves_b * 1000.0;
    let denominator = (amount_a + pair.reserves_a) * 1000.0;
    numerator / denominator
}

fn sell_price_b(pair: &Pair, amount_b: f32) -> f32 {
    let numerator = amount_b * pair.reserves_a * 1000.0;
    let denominator = (amount_b + pair.reserves_b) * 1000.0;
    numerator / denominator
}

fn sell_a(pair: &mut Pair, amount_a: f32) -> f32 {
    let b = sell_price_a(pair, amount_a);
    pair.reserves_a = pair.reserves_a + amount_a;
    pair.reserves_b = pair.reserves_b - b;
    b
}

fn sell_b(pair: &mut Pair, amount_b: f32) -> f32 {
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

        add_liquidity(&mut pair, 100000.0, 1000.0);
        println!("Liquidity event #{}", i);

        println!("{:#?} k: {}", pair, pair.reserves_a * pair.reserves_b);

        println!("Buy price of {} a: {} b", 1.0, buy_price_a(&pair, 1.0));
        println!("Buy price of {} b: {} a", 1.0, buy_price_b(&pair, 1.0));

        let cost_b = buy_a(&mut pair, 10.0);
        println!("Bought {} a for {} b", 10.0, cost_b);

        println!("Buy price of {} a: {} b", 1.0, buy_price_a(&pair, 1.0));
        println!("Buy price of {} b: {} a", 1.0, buy_price_b(&pair, 1.0));

        println!("Sell price of {} a: {} b", 1.0, sell_price_a(&pair, 1.0));
        println!("Sell price of {} b: {} a", 1.0, sell_price_b(&pair, 1.0));

        let cost_b = buy_a(&mut pair, 9000.0);
        println!("Bought {} a for {} b", 9000.0, cost_b);
        let cost_a = buy_b(&mut pair, 10.0);
        println!("Bought {} b for {} a", 10.0, cost_a);

        println!("Buy price of {} a: {} b", 1.0, buy_price_a(&pair, 1.0));
        println!("Buy price of {} b: {} a", 1.0, buy_price_b(&pair, 1.0));

        println!("Sell price of {} a: {} b", 1.0, sell_price_a(&pair, 1.0));
        println!("Sell price of {} a: {} b", 10.0, sell_price_a(&pair, 10.0));

        let gain_a = sell_b(&mut pair, 10.0);
        println!("Sold {} b: for {} a", 10.0, gain_a);
        let gain_b = sell_a(&mut pair, 10.0);
        println!("Sold {} a: for {} b", 10.0, gain_b);

        println!("{:#?} k: {}", pair, pair.reserves_a * pair.reserves_b);
    }
}
