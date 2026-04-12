``` TypeScript // only for coloring syntax
function main() {
    // that boy will change name
    var boy = Box.new(User.new("Paul")); // print name in console
    console.log(`${boy.getName()}`);
    boy.changeNameTo("Mark");
    console.log(`${boy.getName()}`) // and boy printing changed name

    // that girl never will change name
    let girl = Box.new(User.new("Martha"));
    console.log(`${girl.getName()}`) // girl only can print name
}

class User {
    name: String;
}

extend User {
    new(name: String): User {
        return User {
            name: name
        };
    }
    changeNameTo(mut this, newName: own String) {
        this.name = newName;
    }
    getName(ref this): ref String {
        return ref this.name;
    }
}
```
``` Rust
fn main() {
    // that boy will change name
    let mut boy = Box::new(User::new(String::from("Paul")));
    println!("{}", boy.get_name()); // print name in console
    boy.change_name_to("Mark");
    println!("{}", boy.get_name()); // and boy printing changed name

    // that girl never will change name
    let girl = Box::new(User::new(String::from("Martha")));
    println!("{}", girl.get_name()); // girl only can print name
}

struct User {
    name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
        }
    }
    pub fn change_name_to(&mut self, new_name: String) {
        self.name = new_name;
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
```