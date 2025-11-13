pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(id: u32, name: &str, email: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    pub fn display(&self) {
        println!("id: {}", self.id);
        println!("name: {}", self.name);
        println!("email: {}", self.email);
    }

    pub fn to_map(&self) -> Hashmap<String, String> {
        let mut map = HashMap::new();
        map.insert(String::from("id"), self.id.to_string());
        map.insert(String::from("name"), self.name.to_string());
        map.insert(String::from("email"), self.email.to_string());
        map
    }
}

fn main() {
    let user = User::new(1, "Ravone", "ravone@example.com");
    user.display();
    let user = user.to_map();
    println!("{:#?}", user);
}
