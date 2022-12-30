use js_sys::{Array, Function};
use std::cell::Ref;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub enum Query {
    Function(Function),
    Text(String),
    Object(JsValue),
    List(Array),
    None,
}

impl Query {
    pub fn new(query: JsValue) -> Query {
        return if query.is_undefined() {
            Query::None
        } else if query.is_function() {
            let query_fun = query.unchecked_into::<Function>();
            Query::Function(query_fun)
        } else if query.is_string() {
            let query_string = query.as_string().unwrap();
            Query::Text(query_string)
        } else if query.is_object() {
            if Array::is_array(&query) {
                let query_array = query.unchecked_into::<Array>();
                Query::List(query_array)
            } else {
                Query::Object(query)
            }
        } else {
            Query::None
        };
    }

    pub fn equals(&self, obj: Ref<JsValue>) -> bool {
        match self {
            // When no query, every JsValue matches.
            Query::None => true,
            _ => false,
        }
    }
}
