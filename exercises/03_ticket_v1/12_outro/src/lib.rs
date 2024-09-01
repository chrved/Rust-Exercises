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
pub struct Order {
    product_name: String,
    quantity: i32,
    unit_price: i32
}

impl Order {
    pub fn new(name: String, q: i32, price: i32) -> Order {
        Self::is_name_valid(&name);
        Self::is_quantity_valid(&q);
        Self::is_price_valid(&price);

        Order {
            product_name: name,
            quantity: q,
            unit_price: price
        }
    }

    pub fn product_name(&self) -> &String{
        &self.product_name
    }
    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }
    pub fn total(&self) -> i32 {
        let total = self.unit_price * self.quantity;
        total
    }

    pub fn set_product_name(&mut self, name: String) {
        self.product_name = name
    }
    pub fn set_quantity(&mut self, q: i32) {
        self.quantity = q
    }
    pub fn set_unit_price(&mut self, price: i32) {
        self.unit_price = price
    }

    fn is_name_valid(val: &String) {
        if val.is_empty() {
            panic!("Name cannot be empty");
        }
        if val.len() > 300 {
            panic!("Name cannot be longer than 300 bytes");
        }
    }

    fn is_quantity_valid(val: &i32) {
        if *val <= 0 {
            panic!("Quantity cannot be zero");
        }
    }

    fn is_price_valid(val: &i32) {
        if *val <= 0 {
            panic!("Price cannot be zero");
        }
    }
}