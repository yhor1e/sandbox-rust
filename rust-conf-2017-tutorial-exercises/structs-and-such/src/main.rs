#![allow(dead_code)]

struct Store {
    name: String,
    items: Vec<Item>,
}

#[derive(Debug)]
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
    fn total_price(&self, _shopping_list: &[&str]) -> Option<f32> {
        println!("{:?}", _shopping_list);

        let mut sum = 0.0;

        for item in _shopping_list {
            println!("{:?}...", self.price(item));

            match self.price(item) as Option<f32> {
                Option::Some(val) => sum = sum + val,
                Option::None => None,
            }
        }
        Some(sum)
    }
}

fn build_store() -> Store {
    let mut store = Store::new(format!("Rustmart"));
    store.add_item(Item {
        name: format!("chocolate"),
        price: 5.0,
    });
    store.add_item(Item {
        name: format!("socks"),
        price: 23.0,
    });
    store.add_item(Item {
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
