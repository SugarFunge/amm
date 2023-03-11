use crate::LiquidityPool;
use rust_decimal::Decimal;

#[derive(Debug, Default, Clone)]
pub struct ConstantSumMarketMaker {
    reserves_a: Decimal,
    reserves_b: Decimal,
}

/// Constant sum market maker
///
/// Constant Sum Market Maker (CSMM) is an algorithm used in Automated Market Makers (AMMs) to
/// maintain a fixed sum of two assets in a pool. When a user buys one asset, the price of the
/// other asset in the pool increases to maintain the constant sum. CSMM is useful for stablecoin
/// pools where the sum of the stablecoin prices should remain constant.
impl LiquidityPool for ConstantSumMarketMaker {
    /// Sets the current reserves of the pool.
    fn set_reserves(&mut self, reserves_a: Decimal, reserves_b: Decimal) {
        self.reserves_a = reserves_a;
        self.reserves_b = reserves_b;
    }

    /// Returns the current reserves of the pool.
    fn reserves(&self) -> (Decimal, Decimal) {
        (self.reserves_a, self.reserves_b)
    }

    /// Computes the price of token A in terms of token B, given an amount of token A.
    fn price_a(&self, amount_a: Decimal) -> Decimal {
        let total_reserves = self.reserves_a + self.reserves_b;
        if total_reserves == Decimal::ZERO {
            Decimal::ZERO
        } else {
            let new_reserves_a = self.reserves_a + amount_a;
            let new_reserves_b = self.reserves_b - (amount_a * self.reserves_b / self.reserves_a);
            new_reserves_b
                .checked_div(new_reserves_a)
                .unwrap_or_else(|| Decimal::ZERO)
        }
    }

    /// Computes the price of token B in terms of token A, given an amount of token B.
    fn price_b(&self, amount_b: Decimal) -> Decimal {
        let total_reserves = self.reserves_a + self.reserves_b;
        if total_reserves == Decimal::ZERO {
            Decimal::ZERO
        } else {
            let new_reserves_b = self.reserves_b + amount_b;
            let new_reserves_a = self.reserves_a - (amount_b * self.reserves_a / self.reserves_b);
            new_reserves_a
                .checked_div(new_reserves_b)
                .unwrap_or_else(|| Decimal::ZERO)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_add_liquidity() {
        // Initialize a new ConstantSumMarketMaker with reserves of (10, 20).
        let mut pool = ConstantSumMarketMaker::default();
        pool.set_reserves(dec!(10), dec!(20));

        // Add 1 unit of token A and 2 units of token B to the pool.
        pool.add_liquidity(dec!(1), dec!(2));

        // Check that the reserves are now (11, 22).
        assert_eq!(pool.reserves(), (dec!(11), dec!(22)));
    }

    #[test]
    fn test_remove_liquidity() {
        // Initialize a new ConstantSumMarketMaker with reserves of (10, 20).
        let mut pool = ConstantSumMarketMaker::default();
        pool.set_reserves(dec!(10), dec!(20));

        // Remove 1 unit of token A and 2 units of token B from the pool.
        pool.remove_liquidity(dec!(1), dec!(2));

        // Check that the reserves are now (9, 18).
        assert_eq!(pool.reserves(), (dec!(9), dec!(18)));
    }

    #[test]
    fn test_price_a() {
        // Initialize a new ConstantSumMarketMaker with reserves of (10, 20).
        let pool = ConstantSumMarketMaker {
            reserves_a: dec!(10),
            reserves_b: dec!(20),
        };

        // Compute the price of 1 unit of token A in terms of token B.
        let price = pool.price_a(dec!(1)).round_dp(2);

        // Check that the price is 1.64
        assert_eq!(price, dec!(1.64));
    }

    #[test]
    fn test_swap_a() {
        // Initialize a new ConstantSumMarketMaker with reserves of (10, 20).
        let mut pool = ConstantSumMarketMaker {
            reserves_a: dec!(10),
            reserves_b: dec!(20),
        };

        // Swap 1 unit of token A for token B.
        pool.swap_a(dec!(1));

        // Check that the reserves are now (9, 21.64).
        assert_eq!(pool.reserves_rounded(), (dec!(9), dec!(21.64)));
    }

    #[test]
    fn test_price_b() {
        // Initialize a new ConstantSumMarketMaker with reserves of (10, 20).
        let pool = ConstantSumMarketMaker {
            reserves_a: dec!(10),
            reserves_b: dec!(20),
        };

        // Compute the price of 2 units of token B in terms of token A.
        let price = pool.price_b(dec!(2)).round_dp(2);

        // Check that the price is approximately 0.41.
        assert_eq!(price, dec!(0.41));
    }

    #[test]
    fn test_swap_b() {
        // Initialize a new ConstantSumMarketMaker with reserves of (10, 20).
        let mut pool = ConstantSumMarketMaker {
            reserves_a: dec!(10),
            reserves_b: dec!(20),
        };

        // Swap 2 units of token B for token A.
        pool.swap_b(dec!(2));

        // Check that the reserves are now (10.82, 18).
        assert_eq!(pool.reserves_rounded(), (dec!(10.82), dec!(18)));
    }
}
