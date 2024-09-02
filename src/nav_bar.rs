use crate::router::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div {
          class: "flex justify-center p-2",
          nav {
            class: "flex justify-center w-auto border-2 rounded-full p-1 bg-whitesmoke",
              ul {
                  class: "flex space-x-2 justify-center",
                  NavBarItem{ route: Route::Home {}, name: "Home" }
                  NavBarItem{ route: Route::BlogList {}, name: "Blog" }
              }
          }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn NavBarItem(route: Route, name: String ) -> Element {
    rsx! {
        li {
            class: "p-1 rounded-full bg-transparent hover:bg-gray-400",
            Link { to: route, {name} }
        }
    }
}
