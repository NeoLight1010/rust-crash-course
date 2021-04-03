// Structs are used to create custom data types

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple structs
struct TupColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string()
        }
    }

    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_first_name(&mut self, new_first:
         &str) {
        self.first_name = new_first.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
   let mut c = Color {
       red: 255,
       green: 0,
       blue: 0
   };

   println!("Color: {} {} {}", c.red, c.green, c.blue);

   c.blue = 125;
   println!("Color: {} {} {}", c.red, c.green, c.blue);

   let tup_c = TupColor(0, 126, 255);
   println!("Color: {} {} {}", tup_c.0, tup_c.1, tup_c.2);

   let mut p = Person::new("John", "Smith");
   
   p.set_first_name("Michael");
   println!("Changed person: {}", p.get_full_name());
   println!("As tuple: {:?}", p.to_tuple());
}