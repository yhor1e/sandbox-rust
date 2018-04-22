struct Store {
    name: String,
    items: Vec<Item>,
}

struct Item {
    name: String,
    price: f32,
}

impl Store {
    fn new(name: String) -> Store {
        Store {
            name: name,
            items: vec![],
        }
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn price(&self, item_name: &str) -> Option<f32> {
        for item in &self.items {
            if item.name == item_name {
                return Some(item.price);
            }
        }
        None
    }
    fn total_price(&self, shopping_list: &[&str]) -> Option<f32> {
        Some(0.0)
    }
}

fn build_store() -> Store {
    let mut store = Store::new(format!("Rustmart"));
    sotre.add_item(Item {
        name: format!("chocolate"),
        price: 5.0,
    });
    sotre.add_item(Item {
        name: format!("socks"),
        price: 23.0,
    });
    sotre.add_item(Item {
        name: format!("plush Mozilla dinosaur"),
        price: 13.0,
    });
    store
}

#[test]
fn total_price() {
    let store = build_store();
    let list = vec!["chocolate", "plush Mozilla dinosaur"];
    assert_eq!(store.total_price(&list), Some(18.0));
}

#[test]
fn total_price_missing() {
    let store = build_store();
    let list = vec!["chocolate", "plush Mozilla dinosaur", "fork and knife"];
    assert_eq!(store.total_price(&list), None);
}
