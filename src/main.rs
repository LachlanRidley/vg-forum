#[macro_use]
extern crate rocket;

use maud::{html, Markup, DOCTYPE};
use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
struct Post {
    id: String,
    title: String,
    url: String,
    likes: u32,
}

impl Post {
    fn new(id: &str, title: &str, url: &str) -> Post {
        Post {
            id: String::from(id),
            title: String::from(title),
            url: String::from(url),
            likes: 0,
        }
    }

    fn up_doot(self: &mut Post) {
        self.likes += 1;
    }
}

fn page(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            (content)
        }
    }
}

fn header(page_title: &str) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            title { (page_title) }
            script src="https://unpkg.com/htmx.org@2.0.2/dist/htmx.js" integrity="sha384-yZq+5izaUBKcRgFbxgkRYwpHhHHCpp5nseXp0MEQ1A4MTWVMnqkmcuFez8x5qfxr" crossorigin="anonymous" { }
            link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css" { }
        }
    }
}

fn post_list_item(post: &Post) -> Markup {
    html! {
        li class=(post.id) {
            span { (post.title) " [" (post.likes) "]" }
            button hx-post="/updoot" hx-target= { "." (post.id) } {
                "Updoot"
            }
        }
    }
}

#[get("/")]
fn index() -> Markup {
    let post = Post::new("test-1", "First post!", "https://www.google.com");
    let post2 = Post::new("test-2", "Second post!", "https://www.google.com");

    let posts = vec![post, post2];

    page(html! {
        (header("News"))
        body {
            header {
                h1 { "vg-forum" }
            }
            main class="container" {
                h2 { "Post!" }
                ul {
                    @for post in &posts {
                        (post_list_item(&post))
                    }
                }
            }
        }
    })
}

#[post("/updoot")]
fn updoot() -> Markup {
    let post = Post::new("test-1", "First post!", "https://www.google.com");
    let mut post2 = Post::new("test-2", "Second post!", "https://www.google.com");

    post2.up_doot();

    let _posts = vec![post, post2];

    html! {
        "clicked!"
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, updoot])
}
