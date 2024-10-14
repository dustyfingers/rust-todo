use std::collections::Hashmap;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action:");
    let item = std::env::args().nth(2).expect("Please specify an item:");

    println!("{:?} {:?}", action, item);
}

// structs are like types in typescript
struct Todo {
    // use rust built in hashmap to store kv pairs
    map: HashMap<String, bool>,
}

// impl holds the types 'methods'
impl Todo {
    // insert a new item into the map
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }
}
