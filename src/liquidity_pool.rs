use rust_decimal::Decimal;

pub trait LiquidityPool {
    /// Sets the current reserves of the pool.
    fn set_reserves(&mut self, reserves_a: Decimal, reserves_b: Decimal);

    /// Returns the current reserves of the pool.
    fn reserves(&self) -> (Decimal, Decimal);

    /// Returns the current reserves of the pool rounded to 2 decimal places.
    fn reserves_rounded(&self) -> (Decimal, Decimal) {
        let (reserves_a, reserves_b) = self.reserves();
        (reserves_a.round_dp(2), reserves_b.round_dp(2))
    }

    /// Computes the price of token A in terms of token B, given an amount of token A.
    fn price_a(&self, amount_a: Decimal) -> Decimal;

    /// Computes the price of token B in terms of token A, given an amount of token B.
    fn price_b(&self, amount_b: Decimal) -> Decimal;

    /// Adds liquidity to the pool by depositing given amounts of two tokens.
    fn add_liquidity(&mut self, a: Decimal, b: Decimal) {
        let (reserves_a, reserves_b) = self.reserves();
        let reserves_a = reserves_a.saturating_add(a);
        let reserves_b = reserves_b.saturating_add(b);
        self.set_reserves(reserves_a, reserves_b);
    }

    /// Removes liquidity from the pool by withdrawing given amounts of two tokens.
    fn remove_liquidity(&mut self, a: Decimal, b: Decimal) {
        let (reserves_a, reserves_b) = self.reserves();
        let reserves_a = reserves_a.saturating_sub(a);
        let reserves_b = reserves_b.saturating_sub(b);
        self.set_reserves(reserves_a, reserves_b);
    }

    /// Swaps a given amount of token A for token B.
    ///
    /// Returns the amount of token B received.
    fn swap_a(&mut self, amount_a: Decimal) -> Decimal {
        let cost_b = self.price_a(amount_a) * amount_a;
        if cost_b == Decimal::ZERO {
            return Decimal::ZERO;
        }
        let reserves = self.reserves();
        let reserves_a = reserves
            .0
            .checked_sub(amount_a)
            .unwrap_or_else(|| Decimal::ZERO);
        let reserves_b = reserves
            .1
            .checked_add(cost_b)
            .unwrap_or_else(|| Decimal::ZERO);
        self.set_reserves(reserves_a, reserves_b);
        cost_b
    }

    /// Swaps a given amount of token B for token A.
    ///
    /// Returns the amount of token A received.
    fn swap_b(&mut self, amount_b: Decimal) -> Decimal {
        let cost_a = self.price_b(amount_b) * amount_b;
        if cost_a == Decimal::ZERO {
            return Decimal::ZERO;
        }
        let reserves = self.reserves();
        let reserves_b = reserves
            .1
            .checked_sub(amount_b)
            .unwrap_or_else(|| Decimal::ZERO);
        let reserves_a = reserves
            .0
            .checked_add(cost_a)
            .unwrap_or_else(|| Decimal::ZERO);
        self.set_reserves(reserves_a, reserves_b);
        cost_a
    }
}
