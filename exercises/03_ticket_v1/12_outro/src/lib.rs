// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

fn validate_product_name(product_name: &String) {
    if product_name.is_empty() {
        panic!("The product name cannot be empty.");
    }
    if product_name.len() > 300 {
        panic!("The product name cannot be longer than 300 bytes.");
    }
}

fn validate_quantity(quantity: &u16) {
    if *quantity == 0 {
        panic!("The quantity cannot be 0.");
    }
}

fn validate_unit_price(unit_price: &u32) {
    if *unit_price == 0 {
        panic!("The unit price cannot be 0.");
    }
}


pub struct Order {
    product_name: String,
    quantity: u16,
    unit_price: u32
}

impl Order {
    pub fn new(product_name: String, quantity: u16, unit_price: u32) -> Order {
        validate_product_name(&product_name);
        validate_quantity(&quantity);
        validate_unit_price(&unit_price);

        return Order{product_name, quantity, unit_price};
    }

    pub fn total(&self) -> u64 {
        return (self.quantity as u64) * (self.unit_price as u64);
    }

    pub fn product_name(&self) -> &String {
        return &self.product_name;
    }
    
    pub fn quantity(&self) -> &u16 {
        return &self.quantity;
    }

    pub fn unit_price(&self) -> &u32 {
        return &self.unit_price;
    }

    pub fn set_product_name(&mut self, product_name: String) {
        validate_product_name(&product_name);
        self.product_name = product_name;
    }

    pub fn set_quantity(&mut self, quantity: u16) {
        validate_quantity(&quantity);
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: u32) {
        validate_unit_price(&unit_price);
        self.unit_price = unit_price;
    }

}