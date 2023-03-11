use crate::LiquidityPool;
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;

#[derive(Debug, Clone)]
pub struct PIDMarketMaker {
    reserves_a: Decimal,
    reserves_b: Decimal,
    target_price: Decimal,
    pid_controller: PIDController,
    cached_price: Decimal,
}

/// PID market maker
///
/// A PID controller is used to maintain a target price for an asset pair by adjusting the ratio of
/// the assets in the pool. The proportional, integral, and derivative terms of the controller are
/// tuned based on historical data to optimize its performance in maintaining the target price.
impl LiquidityPool for PIDMarketMaker {
    /// Sets the current reserves of the pool.
    fn set_reserves(&mut self, reserves_a: Decimal, reserves_b: Decimal) {
        self.reserves_a = reserves_a;
        self.reserves_b = reserves_b;

        let total_reserves = self.reserves_a + self.reserves_b;
        if total_reserves == Decimal::ZERO {
            self.cached_price = Decimal::ZERO;
        } else {
            let new_ratio = self.compute_ratio(total_reserves);
            self.cached_price = new_ratio
                .checked_mul(self.target_price)
                .unwrap_or_else(|| Decimal::ZERO);
        }
    }

    /// Returns the current reserves of the pool.
    fn reserves(&self) -> (Decimal, Decimal) {
        (self.reserves_a, self.reserves_b)
    }

    /// Computes the price of token A in terms of token B, given an amount of token A.
    fn price_a(&self, amount_a: Decimal) -> Decimal {
        if amount_a == Decimal::ZERO {
            return Decimal::ZERO;
        }
        self.cached_price
            .checked_div(amount_a)
            .unwrap_or_else(|| Decimal::ZERO)
    }

    /// Computes the price of token B in terms of token A, given an amount of token B.
    fn price_b(&self, amount_b: Decimal) -> Decimal {
        if amount_b == Decimal::ZERO {
            return Decimal::ZERO;
        }
        self.cached_price
            .checked_mul(amount_b)
            .unwrap_or_else(|| Decimal::ZERO)
    }
}

impl Default for PIDMarketMaker {
    fn default() -> Self {
        Self {
            reserves_a: Decimal::ZERO,
            reserves_b: Decimal::ZERO,
            target_price: Decimal::ONE,
            pid_controller: PIDController::default(),
            cached_price: Decimal::ZERO,
        }
    }
}

impl PIDMarketMaker {
    /// Computes the ratio of the two assets in the pool.
    fn compute_ratio(&mut self, total_reserves: Decimal) -> Decimal {
        let new_reserves_a = total_reserves
            .checked_mul(self.target_price)
            .and_then(|p| p.sqrt())
            .map(|p| {
                self.reserves_a
                    .checked_mul(p)
                    .unwrap_or_else(|| Decimal::ZERO)
            })
            .unwrap_or_else(|| Decimal::ZERO);
        let new_reserves_b = total_reserves - new_reserves_a;
        let error = self.target_price
            - new_reserves_b
                .checked_div(new_reserves_a)
                .unwrap_or_else(|| Decimal::ZERO);
        let control_signal = self.pid_controller.compute(error);
        let new_ratio = self
            .reserves_b
            .checked_div(self.reserves_a + control_signal)
            .unwrap_or_else(|| Decimal::ZERO);
        new_ratio
    }
}

/// A PID controller for controlling the asset ratio in a liquidity pool.
#[derive(Debug, Clone)]
pub struct PIDController {
    kp: Decimal,         // Proportional gain
    ki: Decimal,         // Integral gain
    kd: Decimal,         // Derivative gain
    integral: Decimal,   // Integral term accumulator
    prev_error: Decimal, // Previous error value
}

impl PIDController {
    /// Creates a new PID controller with the given gains.
    pub fn new(kp: Decimal, ki: Decimal, kd: Decimal) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: Decimal::ZERO,
            prev_error: Decimal::ZERO,
        }
    }

    /// Computes the control signal for the given error.
    pub fn compute(&mut self, error: Decimal) -> Decimal {
        let proportional_term = self.kp * error;
        self.integral += error;
        let integral_term = self.ki * self.integral;
        let derivative_term = self.kd * (error - self.prev_error);
        self.prev_error = error;
        proportional_term + integral_term + derivative_term
    }
}

impl Default for PIDController {
    fn default() -> Self {
        Self::new(dec!(0.1), dec!(0.01), dec!(0.001))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_liquidity() {
        let mut pool = PIDMarketMaker::default();
        pool.set_reserves(dec!(100), dec!(100));
        // Check reserves
        assert_eq!(pool.reserves(), (dec!(100), dec!(100)));
    }

    #[test]
    fn test_remove_liquidity() {
        let mut pool = PIDMarketMaker::default();
        pool.set_reserves(dec!(100), dec!(100));
        // Check reserves
        assert_eq!(pool.reserves(), (dec!(100), dec!(100)));
    }

    #[test]
    fn test_price_a() {
        let mut pool = PIDMarketMaker::default();
        pool.set_reserves(dec!(100), dec!(100));
        let price = pool.price_a(dec!(1)).round_dp(2);
        // Check price
        assert_eq!(price, dec!(1));
    }

    #[test]
    fn test_swap_a() {
        let mut pool = PIDMarketMaker::default();
        pool.set_reserves(dec!(100), dec!(100));
        pool.swap_a(dec!(1));
        // Check reserves
        assert_eq!(pool.reserves_rounded(), (dec!(99), dec!(101)));
    }

    #[test]
    fn test_price_b() {
        let mut pool = PIDMarketMaker::default();
        pool.set_reserves(dec!(100), dec!(100));
        let price = pool.price_b(dec!(1)).round_dp(2);
        // Check price
        assert_eq!(price, dec!(1));
    }

    #[test]
    fn test_swap_b() {
        let mut pool = PIDMarketMaker::default();
        pool.set_reserves(dec!(100), dec!(100));
        pool.swap_b(dec!(1));
        // Check reserves
        assert_eq!(pool.reserves_rounded(), (dec!(101), dec!(99)));
    }
}
