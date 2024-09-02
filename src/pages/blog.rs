use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    rsx! {
        h1 {
          "Blog"
        }
        Outlet::<Route> {}
    }
}

#[component]
pub fn BlogList() -> Element {
    rsx! {
      div{
        class: "text-lg",
        h2 {
          class: "text-sm font-medium",
          "Choose a post",
        }
        ul {
            li {
                Link {
                    to: Route::BlogPost {
                        name: "Blog post 1".into(),
                    },
                    "Read the first blog post"
                }
            }
            li {
                Link {
                    to: Route::BlogPost {
                        name: "Blog post 2".into(),
                    },
                    "Read the second blog post"
                }
            }
        }
      }
    }
}

#[component]
pub fn BlogPost(name: String) -> Element {
    rsx! { h2 { "Blog Post: {name}" } }
}
