// GOOD: No technical debt comments, proper implementation

const BULK_DISCOUNT_THRESHOLD: f64 = 100.0;
const BULK_DISCOUNT_RATE: f64 = 0.9;
const TAX_RATE: f64 = 0.08;

fn calculate_price(quantity: f64, price_per_unit: f64, include_tax: bool) -> Result<f64, String> {
    if quantity < 0.0 || price_per_unit < 0.0 {
        return Err("Quantity and price must be non-negative".to_string());
    }

    let base_price = quantity * price_per_unit;
    let discounted_price = apply_bulk_discount(base_price, quantity);

    if include_tax {
        Ok(discounted_price * (1.0 + TAX_RATE))
    } else {
        Ok(discounted_price)
    }
}

fn apply_bulk_discount(price: f64, quantity: f64) -> f64 {
    if quantity > BULK_DISCOUNT_THRESHOLD {
        price * BULK_DISCOUNT_RATE
    } else {
        price
    }
}

fn calculate_shipping_cost(weight_kg: f64) -> Result<f64, String> {
    if weight_kg < 0.0 {
        return Err("Weight must be non-negative".to_string());
    }

    const BASE_RATE: f64 = 5.0;
    const PER_KG_RATE: f64 = 0.5;

    Ok(BASE_RATE + (weight_kg * PER_KG_RATE))
}

fn calculate_total(quantity: f64, price_per_unit: f64, weight: f64, include_tax: bool) -> Result<f64, String> {
    let price = calculate_price(quantity, price_per_unit, include_tax)?;
    let shipping = calculate_shipping_cost(weight)?;

    Ok(price + shipping)
}

fn main() {
    match calculate_total(150.0, 10.0, 5.0, true) {
        Ok(total) => println!("Total price: ${:.2}", total),
        Err(e) => eprintln!("Error calculating total: {}", e),
    }

    match calculate_shipping_cost(5.0) {
        Ok(shipping) => println!("Shipping: ${:.2}", shipping),
        Err(e) => eprintln!("Error calculating shipping: {}", e),
    }
}