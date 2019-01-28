#![feature(proc_macro_hygiene)]

extern crate virtual_dom_rs;
use virtual_dom_rs::prelude::*;

// Your text contains punctuation.
// We're not sure about whether or not it originally contained spaces.
// You'll need to wrap it in quotes
fn main () {
    html! {
        <div>
            Hello.
            Hi,
            Hola!
            Bonjour/
            Bonjour\
        </div>
    };
}
