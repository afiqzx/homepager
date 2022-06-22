use perseus::{Html, PerseusApp, Template};
use sycamore::view;

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new().template(|| {
        Template::new("index").template(|_| {
            view! {
                p { "Hello World!" }
                table(style="min-width:500px; background-color:yellow;") {
                    tr {
                        th { p { "header 1" } }
                        th { p { "header 2" } }
                    }
                    tr {
                        td { p { "data 1.1" } }
                        td { p { "data 1.2" } }
                    }
                    tr {
                        td { p { "data 2.1" } }
                        td { p { "data 2.2" } }
                    }
                }
            }
        })
    })
}

