use crate::error::{Error, Result};
use crate::types::OrderSummary;
use rust_decimal::Decimal;

/// Calculate the price for a market order based on order book depth
///
/// This walks the order book until enough liquidity is found to match
/// the requested amount.
///
/// # Arguments
/// * `positions` - The order book positions to walk through
/// * `amount_to_match` - The total amount (size * price) needed to match
///
/// # Returns
/// The price at which the market order can be filled, or an error if there's insufficient liquidity
///
/// # Example
/// ```no_run
/// use polymarket_rs::orders::calculate_market_price;
/// use polymarket_rs::types::OrderSummary;
/// use rust_decimal::Decimal;
///
/// let positions = vec![
///     OrderSummary {
///         price: Decimal::new(50, 2),
///         size: Decimal::new(100, 0),
///         ..Default::default()
///     },
///     OrderSummary {
///         price: Decimal::new(51, 2),
///         size: Decimal::new(200, 0),
///         ..Default::default()
///     },
/// ];
/// let price = calculate_market_price(&positions, Decimal::new(5000, 2)).unwrap();
/// ```
pub fn calculate_market_price(
    positions: &[OrderSummary],
    amount_to_match: Decimal,
) -> Result<Decimal> {
    let mut sum = Decimal::ZERO;

    for p in positions {
        sum += p.size * p.price;
        if sum >= amount_to_match {
            return Ok(p.price);
        }
    }

    Err(Error::InvalidOrder(format!(
        "Not enough liquidity to create market order with amount {}",
        amount_to_match
    )))
}
