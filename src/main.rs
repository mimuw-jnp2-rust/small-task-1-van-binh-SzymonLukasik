use enum_iterator::IntoEnumIterator;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::stdin;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, IntoEnumIterator)]
enum Dish {
    ThaiChicken,
    Tofu,
    FriedRice,
}

impl Dish {
    fn price(&self) -> u32 {
        match self {
            Dish::ThaiChicken => 20,
            Dish::Tofu => 15,
            Dish::FriedRice => 12,
        }
    }
}

const TAKEAWAY_FEE: u32 = 1;

#[derive(Debug, Clone)]
struct Order {
    dishes: HashMap<Dish, u32>,
    takeaway: bool,
}

impl Order {
    fn new() -> Order {
        Order {
            dishes: Dish::into_enum_iter()
                .map(|dish| (dish, 0))
                .collect::<HashMap<Dish, u32>>(),
            takeaway: false,
        }
    }

    fn add_dish(&mut self, dish: Dish) {
        self.dishes.entry(dish).and_modify(|x| *x += 1);
    }

    fn set_takeaway(&mut self) {
        self.takeaway = true;
    }

    fn dish_count(&self, dish: Dish) -> u32 {
        *self.dishes.get(&dish).unwrap()
    }

    fn items_count(&self) -> u32 {
        self.dishes.values().sum::<u32>()
    }

    fn is_takeaway(&self) -> bool {
        self.takeaway
    }

    fn total(&self) -> u32 {
        let sum = self
            .dishes
            .iter()
            .map(|(dish, n)| dish.price() * n)
            .sum::<u32>();

        if self.is_takeaway() {
            sum + self.items_count() * TAKEAWAY_FEE
        } else {
            sum
        }
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "chicken: {}, tofu: {}, rice: {}, takeway: {}",
            self.dish_count(Dish::ThaiChicken),
            self.dish_count(Dish::Tofu),
            self.dish_count(Dish::FriedRice),
            self.is_takeaway()
        )
    }
}

struct Customer {
    name: String,
    favorite_order: Order,
}

impl Customer {
    pub fn new(name: String, favorite_order: Order) -> Customer {
        Customer {
            name,
            favorite_order,
        }
    }
}

struct VanBinh {
    orders_count: u32,
    customers: Vec<Customer>,
}

impl VanBinh {
    pub fn new() -> VanBinh {
        VanBinh {
            orders_count: 0,
            customers: Vec::new(),
        }
    }

    fn add_customer(&mut self, name: String, favorite_order: Order) {
        self.customers.push(Customer::new(name, favorite_order));
    }

    fn get_saved_customer(&self, name: &str) -> Option<&Customer> {
        self.customers.iter().find(|c| c.name == name)
    }

    fn increase_orders_count(&mut self) {
        self.orders_count += 1;
    }

    fn get_orders_count(&self) -> u32 {
        self.orders_count
    }
}

fn get_line() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn yes_no(question: &str) -> bool {
    println!("{} (y/n)", question);
    get_line() == "y"
}

fn get_order() -> Order {
    let mut order = Order::new();
    loop {
        println!("Enter dish name or empty line to finish:");
        let line = get_line();
        if line.is_empty() {
            break;
        }
        if line.contains("chicken") {
            order.add_dish(Dish::ThaiChicken);
        } else if line.contains("tofu") {
            order.add_dish(Dish::Tofu);
        } else if line.contains("rice") {
            order.add_dish(Dish::FriedRice);
        } else {
            println!("Unknown dish name: {}", line);
        }
    }

    if yes_no("Takeaway?") {
        order.set_takeaway();
    }

    order
}

fn main() {
    let mut van_binh = VanBinh::new();
    loop {
        println!("Hi! Welcome to Van Binh! What's your name?");
        let name = get_line();

        if name.is_empty() {
            break;
        }

        let order = if let Some(customer) = van_binh.get_saved_customer(&name) {
            println!("Welcome back, {}!", customer.name);
            if yes_no("Same as usual?") {
                customer.favorite_order.clone()
            } else {
                get_order()
            }
        } else {
            println!("Welcome, {}!", name);
            let order = get_order();
            if yes_no("Would you like to save this order?") {
                van_binh.add_customer(name, order.clone())
            }
            order
        };

        if order.items_count() == 0 {
            println!("Your order is empty!");
            continue;
        } else {
            van_binh.increase_orders_count();
        }

        println!("This is order no. {}", van_binh.get_orders_count());
        println!(
            "There you go: {}, it's going to be {} zł",
            order,
            order.total()
        );
    }
    println!("Bye!");
}
