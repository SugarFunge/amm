use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
pub struct LiquidityPool {
    reserves_a: Decimal,
    reserves_b: Decimal,
}

impl LiquidityPool {
    /// Adds liquidity to the pool by depositing given amounts of two tokens.
    pub fn add_liquidity(&mut self, a: Decimal, b: Decimal) {
        let reserves_a = self.reserves_a.saturating_add(a);
        let reserves_b = self.reserves_b.saturating_add(b);
        self.reserves_a = reserves_a;
        self.reserves_b = reserves_b;
    }

    /// Removes liquidity from the pool by withdrawing given amounts of two tokens.
    pub fn remove_liquidity(&mut self, a: Decimal, b: Decimal) {
        let reserves_a = self.reserves_a.saturating_sub(a);
        let reserves_b = self.reserves_b.saturating_sub(b);
        self.reserves_a = reserves_a;
        self.reserves_b = reserves_b;
    }

    /// Computes the price of token A in terms of token B, given an amount of token A.
    pub fn price_a(&self, amount_a: Decimal) -> Decimal {
        if self.reserves_a == Decimal::ZERO {
            Decimal::ZERO
        } else {
            self.reserves_b
                .checked_div(self.reserves_a - amount_a)
                .unwrap_or_else(|| Decimal::ZERO)
        }
    }

    /// Computes the price of token B in terms of token A, given an amount of token B.
    pub fn price_b(&self, amount_b: Decimal) -> Decimal {
        if self.reserves_b == Decimal::ZERO {
            Decimal::ZERO
        } else {
            self.reserves_a
                .checked_div(self.reserves_b - amount_b)
                .unwrap_or_else(|| Decimal::ZERO)
        }
    }

    /// Swaps a given amount of token A for token B.
    ///
    /// Returns the amount of token B received.
    pub fn swap_a(&mut self, amount_a: Decimal) -> Decimal {
        let cost_b = self.price_a(amount_a) * amount_a;
        if cost_b == Decimal::ZERO {
            return Decimal::ZERO;
        }
        let reserves_a = self
            .reserves_a
            .checked_sub(amount_a)
            .unwrap_or_else(|| Decimal::ZERO);
        let reserves_b = self
            .reserves_b
            .checked_add(cost_b)
            .unwrap_or_else(|| Decimal::ZERO);
        self.reserves_a = reserves_a;
        self.reserves_b = reserves_b;
        cost_b
    }

    /// Swaps a given amount of token B for token A.
    ///
    /// Returns the amount of token A received.
    pub fn swap_b(&mut self, amount_b: Decimal) -> Decimal {
        let cost_a = self.price_b(amount_b) * amount_b;
        if cost_a == Decimal::ZERO {
            return Decimal::ZERO;
        }
        let reserves_b = self
            .reserves_b
            .checked_sub(amount_b)
            .unwrap_or_else(|| Decimal::ZERO);
        let reserves_a = self
            .reserves_a
            .checked_add(cost_a)
            .unwrap_or_else(|| Decimal::ZERO);
        self.reserves_b = reserves_b;
        self.reserves_a = reserves_a;
        cost_a
    }
}

fn main() {
    let mut pool = LiquidityPool::default();
    pool.add_liquidity(
        Decimal::from_str("100").unwrap(),
        Decimal::from_str("100").unwrap(),
    );
    println!("pool: {:?}", pool);

    let amount = pool.swap_a(Decimal::from_str("10").unwrap());
    println!("cost_b: {}", amount);
    println!("pool: {:?}", pool);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_add_liquidity() {
        // Create a new liquidity pool with default values.
        let mut pool = LiquidityPool::default();

        // Add 100 units of token A and 200 units of token B to the pool.
        pool.add_liquidity(dec!(100), dec!(200));

        // Assert that the reserves of token A and token B in the pool are equal to the added amounts.
        assert_eq!(pool.reserves_a, dec!(100));
        assert_eq!(pool.reserves_b, dec!(200));
    }

    #[test]
    fn test_remove_liquidity() {
        // Create a new liquidity pool with default values.
        let mut pool = LiquidityPool::default();

        // Add 100 units of token A and 200 units of token B to the pool.
        pool.add_liquidity(dec!(100), dec!(200));

        // Remove 10 units of token A and 20 units of token B from the pool.
        pool.remove_liquidity(dec!(10), dec!(20));

        // Assert that the reserves of token A and token B in the pool have been reduced by the given amounts.
        assert_eq!(pool.reserves_a, dec!(90));
        assert_eq!(pool.reserves_b, dec!(180));
    }

    #[test]
    fn test_price_a() {
        // Create a new liquidity pool with default values.
        let mut pool = LiquidityPool::default();

        // Add 100 units of token A and 200 units of token B to the pool.
        pool.add_liquidity(dec!(100), dec!(200));

        // Compute the price of token A in terms of token B, given an amount of 10 units of token A.
        let price = pool.price_a(dec!(10)).round_dp(2);

        // Assert that the computed price is equal to the expected value.
        assert_eq!(price, dec!(2.22));
    }

    #[test]
    fn test_swap_a() {
        // Create a new liquidity pool with default values.
        let mut pool = LiquidityPool::default();

        // Add 100 units of token A and 200 units of token B to the pool.
        pool.add_liquidity(dec!(100), dec!(200));

        // Swap 10 units of token A for token B.
        let cost_a = pool.swap_b(dec!(10)).round_dp(2);

        // Assert that the cost in token A for the swap is equal to the expected value.
        assert_eq!(cost_a, dec!(5.26));

        // Assert that the reserves of token A and token B in the pool have been updated accordingly.
        assert_eq!(pool.reserves_a.round_dp(2), dec!(105.26));
        assert_eq!(pool.reserves_b.round_dp(2), dec!(190));
    }

    #[test]
    fn test_price_b() {
        // Create a new liquidity pool with default values.
        let mut pool = LiquidityPool::default();

        // Add 100 units of token A and 200 units of token B to the pool.
        pool.add_liquidity(dec!(100), dec!(200));

        // Compute the price of token B in terms of token A, given an amount of 10 units of token B.
        let price = pool.price_b(dec!(10)).round_dp(2);

        // Assert that the computed price is equal to the expected value.
        assert_eq!(price, dec!(0.53));
    }

    #[test]
    fn test_swap_b() {
        // Create a new liquidity pool with default values.
        let mut pool = LiquidityPool::default();

        // Add 100 units of token A and 200 units of token B to the pool.
        pool.add_liquidity(dec!(100), dec!(200));

        // Swap 10 units of token B for token A.
        let cost_b = pool.swap_a(dec!(10)).round_dp(2);

        // Assert that the cost in token B for the swap is equal to the expected value.
        assert_eq!(cost_b, dec!(22.22));

        // Assert that the reserves of token A and token B in the pool have been updated accordingly.
        assert_eq!(pool.reserves_a.round_dp(2), dec!(90));
        assert_eq!(pool.reserves_b.round_dp(2), dec!(222.22));
    }
}
