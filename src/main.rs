use std::collections::Hashmap;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action:");
    let item = std::env::args().nth(2).expect("Please specify an item:");

    let mut todo = Todo {
        map: HashMap::new(),
    };

    if (action == "add") {
        // insert the item
        todo.insert(item);

        // run the save method (returns a result - either Ok or Err)
        match todo.save() {
            Ok(_) => println!("Todo saved!"),
            Err(why) => println!("An error occoured: {}", why) 
        }
    }
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

    // save map to disk
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k ,v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        std::fs::write("db.txt", content)
    }
}
