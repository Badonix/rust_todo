use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::str::FromStr;
#[derive(Debug)]
struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        std::fs::write("db.txt", content)
    }
    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;

        let mut content = String::new();

        f.read_to_string(&mut content)?;
        let mut map = HashMap::new();

        for entries in content.lines() {
            let mut values = entries.split('\t');
            let key = values.next().expect("No key");
            let val = values.next().expect("No value");

            map.insert(String::from(key), bool::from_str(val).unwrap());
        }
        Ok(Todo { map })
    }
    fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }
    fn complete(&mut self, key: String) -> Option<()> {
        match self.map.get_mut(&key) {
            Some(v) => Some(*v = true),
            None => None,
        }
    }
}
fn main() {
    loop {
        println!("Type 1 to show all todos");
        println!("Type 2 following task to add todo (2 make pizza)");
        println!("Type 3 following task to mark as completed (3 make cakes)");
        println!("Type 4 to exit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read line");

        let mut todo = Todo::new().expect("Initialization of db failed :((");
        let mut words = input.split_whitespace();

        if let Some(action) = words.next() {
            if action == "1" {
                println!("=====");
                let _ = read_file_line_by_line("db.txt");
            } else if action == "4" {
                break;
            } else {
                if let Some(item) = words.next() {
                    if action == "2" {
                        todo.insert(item.to_string());
                        println!("======");
                        match todo.save() {
                            Ok(_) => println!("Todo saved"),
                            Err(why) => println!("Error Occurred, {}", why),
                        }
                    } else if action == "3" {
                        println!("======");
                        match todo.complete(item.to_string()) {
                            None => println!("No such todo :("),
                            Some(_) => match todo.save() {
                                Ok(_) => println!("Todos Updated :)"),
                                Err(why) => println!("Couldn't update todos, that's why: {}", why),
                            },
                        }
                    } else {
                        println!("Invalid action");
                    }
                } else {
                    println!("Specify an item");
                }
            }
        }
        println!("======\n");
    }
}

fn read_file_line_by_line(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
