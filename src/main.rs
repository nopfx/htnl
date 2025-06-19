#[macro_use]
mod macros;

mod builder;
mod tokenization;
mod tokens;

Contextable! {
    struct UserTest {
        name: String,
        age: i32,
    }
}
fn main() {
    let templ = "Sveikas, {{ UserTest.name }}! {% if UserTest.age >= 18 %} you can buy alco {% else %} dude you are too little {% endif %} !!!";
    let user = UserTest {
        name: "lazymonad".into(),
        age: 23,
    };

    let htdl = builder::Builder {
        context: user.flatten(),
        content: String::from(templ),
    };

    println!("Build: {}", htdl.build());
}
