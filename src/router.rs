use crate::nav_bar::NavBar;
use crate::pages::blog::{Blog, BlogList, BlogPost};
use crate::pages::home::Home;
use crate::pages::not_found::PageNotFound;
use dioxus::prelude::*;

#[derive(PartialEq, Routable, Clone)]
pub enum Route {
    #[layout(NavBar)]
      #[route("/")]
      Home {},
      #[nest("/blog")]
        #[layout(Blog)]
          #[route("/")]
          BlogList {},
          #[route("/post/:name")]
          BlogPost { name: String },
        #[end_layout]
      #[end_nest]
    #[end_layout]
    #[nest("/myblog")]
      #[redirect("/", || Route::BlogList {})]
      #[redirect("/:name", |name: String| Route::BlogPost { name })]
    #[end_nest]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
