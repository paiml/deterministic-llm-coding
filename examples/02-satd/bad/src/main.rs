// BAD: Multiple TODOs and FIXMEs (PMAT will flag these)

fn calculate_price(quantity: f64, price_per_unit: f64) -> f64 {
    // TODO: Add tax calculation
    // FIXME: This doesn't handle discounts
    let base_price = quantity * price_per_unit;

    // HACK: Hardcoded discount for now
    if quantity > 100.0 {
        base_price * 0.9 // TODO: Make this configurable
    } else {
        base_price
    }
}

// TODO: Implement this function
fn apply_shipping_cost(_weight: f64) -> f64 {
    0.0 // FIXME: Actually calculate shipping
}

// FIXME: This function doesn't handle edge cases
fn calculate_total(quantity: f64, price_per_unit: f64, weight: f64) -> f64 {
    let price = calculate_price(quantity, price_per_unit);
    let shipping = apply_shipping_cost(weight);

    // TODO: Add validation for negative values
    price + shipping
}

fn main() {
    let total = calculate_total(150.0, 10.0, 5.0);
    println!("Total price: ${:.2}", total);

    // TODO: Add proper error handling
    // HACK: Just print for now
    println!("Shipping: ${:.2}", apply_shipping_cost(5.0));
}