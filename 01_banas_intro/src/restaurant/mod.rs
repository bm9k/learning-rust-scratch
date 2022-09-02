mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("gluten-free"),
                cheese: String::from("mozzarella"),
                topping: String::from(topping),
            }
        }
    }

    pub mod help_customer {
        fn seat_at_table() {
            println!("Customer seated at table");
        }
        // need to explicitly make methods public
        pub fn take_order() {
            seat_at_table();

            let pizza: super::Pizza = super::Pizza::lunch("peri-peri chicken");
            serve_customer(pizza);
        }
        fn serve_customer(pizza: super::Pizza) {
            println!("The customer is served a pizza with {}", pizza.topping);
        }
    }
}

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}
