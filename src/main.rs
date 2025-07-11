use crate::htnl::HTNLFile;

mod builder;
mod htnl;
mod tokenization;
mod tokens;

#[macro_use]
mod contextable;

use contextable::Contextable;

context! {
    struct Post {
        title: String,
        slug: String,
        author: String
    }
}
context! {
    struct Meta {
        title: String
    }
}
context! {
    struct User {
        name: String
    }
}
context! {
    struct Data {
        posts: Vec<Post>,
        meta: Meta,
        user: User
    }
}
fn main() {
    let template: String = String::from("./tests/index.htnl");
    let htnlfile = HTNLFile { path: template };
    let c = htnlfile.contents();

    let data = Data {
        posts: vec![
            Post {
                title: "Pirmas post".into(),
                slug: "Pirmo-post-slugas".into(),
                author: "Nop.f(x)".into(),
            },
            Post {
                title: "Antras postas".into(),
                slug: "Antras-post".into(),
                author: "Nop.f(x)".into(),
            },
        ],
        meta: Meta {
            title: "Blogo taitlas".into(),
        },
        user: User {
            name: "NopFx".into(),
        },
    };

    let context = data.flatten();

    let htdl = builder::Builder {
        context: context,
        content: String::from(c),
    };

    println!("Build: {}", htdl.build());
}
