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

type User {
    name: String;
}

implement User {
    new(name: String): User {
        return User {
            name: name
        }
    }
    changeNameTo(var this, newName: String) {
        this.name = newName;
    }
    getName(ref this): String {
        return this.name;
    }
}
```
``` Rust
fn main() {
    // that boy will change name
    let mut boy = Box::new(User::new("Paul"));
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