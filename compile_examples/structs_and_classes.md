``` TypeScript // only for coloring syntax
function main() {
    // that boy will change name
    var boy: box User = new User("Paul"); // print name in console
    console.log(`${boy.getName()}`);
    boy.changeNameTo("Mark");
    console.log(`${boy.getName()}`) // and boy printing changed name

    // that girl never will change name
    let girl = new User("Martha");
    console.log(`${girl.getName()}`) // girl only can print name
}

class User {
    name: String;

    constructor(name: String) {
        this.name = name;
    }

    changeNameTo(change this, newName: String) {
        this.name = newName;
    }

    getName(read this): read String {
        return read this.name
    }
}

```
``` Rust
fn main() {
    // that boy will change name
    let mut boy: Box<User> = Box::new(User::new("Paul"));
    println!("{}", boy.get_name()); // print name in console
    boy.change_name_to("Mark");
    println!("{}", boy.get_name()); // and boy printing changed name

    // that girl never will change name
    let girl = Box::new(User::new("Martha"));
    println!("{}", girl.get_name()); // girl only can print name
}

struct User {
    name: String,
}

impl User {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: String::from(name.into()),
        }
    }
    pub fn change_name_to(&mut self, new_name: impl Into<String>) {
        self.name = new_name.into();
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
```