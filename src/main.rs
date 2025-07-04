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

fn main() {
    let template: String = String::from("./tests/index.htnl");
    let htnlfile = HTNLFile { path: template };
    let c = htnlfile.contents();

    let post1 = Post {
        title: "Pimr as post".into(),
        slug: "pirmas".into(),
        author: "as".into(),
    };

    let post2 = Post {
        title: "ontras as post".into(),
        slug: "untras".into(),
        author: "as".into(),
    };

    println!("{:?}", post1.flatten());

    let mut posts_list = vec![];
    posts_list.push(post1);
    posts_list.push(post2);

    let context = posts_list.flatten();

    println!("{:?}", context);
    let htdl = builder::Builder {
        context: context,
        content: String::from(c),
    };

    println!("Build: {}", htdl.build());
}
