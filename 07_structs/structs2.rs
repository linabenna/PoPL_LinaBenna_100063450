#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

fn main() {
    // Creating the order template
    let order_template = create_order_template();

    // Creating your own order using the update syntax
    let your_order = Order {
        name: String::from("Testing Name"),
        count: 1,
        ..order_template // Using the template to copy over other fields
    };
    
    // Printing the created order to verify
    println!("{:?}", your_order);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();

        // Creating your own order using the update syntax
        let your_order = Order {
            name: String::from("RustName"),
            count: 1,
            ..order_template // Using the template to copy over other fields
        };

        assert_eq!(your_order.name, "RustName");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}
