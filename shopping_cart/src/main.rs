use std::io;

fn main() {
    let mut items: Vec<String> = Vec::new();
    let mut prices: Vec<f64> = Vec::new();
    let mut quantities: Vec<u32> = Vec::new();
    let mut total = 0.0;

    loop {
        let mut item = String::new();
        println!("Enter item name (or 'Q' to quit): ");
        io::stdin()
            .read_line(&mut item)
            .expect("Failed to read line");
        let item = item.trim();
        if item.eq_ignore_ascii_case("Q") {
            break;
        }

        let mut quantity_input = String::new();
        println!("Enter the quantity for '{}':", item);
        io::stdin()
            .read_line(&mut quantity_input)
            .expect("Failed to read line");

        let quantity: u32 = match quantity_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid quantity. Please enter a whole number.");
                continue;
            }
        };

        let mut price_input = String::new();
        println!("Enter price per unit for '{}':", item);
        io::stdin()
            .read_line(&mut price_input)
            .expect("Failed to read line");

        let price: f64 = match price_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid price. Please enter a valid number.");
                continue;
            }
        };

        items.push(item.to_string());
        quantities.push(quantity);
        prices.push(price);
        total += price * quantity as f64;
    }

    println!("\nReceipt");
    println!("---------------------------");
    for (item, (quantity, price)) in items.iter().zip(quantities.iter().zip(prices.iter())) {
        println!("x{} {}: ${:.2}", quantity, item, price * (*quantity as f64));
    }
    println!("---------------------------");
    println!("Total: ${:.2}", total);
}
