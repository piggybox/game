use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        print!("{}", self.greeting);
    }
}

fn main() {
    let visitor_list = [
        Visitor::new("bert", "Hello Bert"),
        Visitor::new("steve", "Hi Steve"),
        Visitor::new("fred", "Hi Fred"),
    ];

    println!("Hello, what's your name?");
    let name = what_is_your_name();
    println!("Hello, {:?}", name);

    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

    if let Some(visitor) = known_visitor {
        visitor.greet_visitor()
    } else {
        println!("You are not on the visitor list. Please leave.")
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}
