use js_sys::{Array, Reflect};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Paths {
    key_paths: Rc<RefCell<HashMap<String, Vec<String>>>>,
    // type_paths: Rc<RefCell<HashMap<String, Vec<String>>>>,
}

type View = Rc<RefCell<JsValue>>;

impl Paths {
    pub fn new() -> Self {
        Self {
            key_paths: Rc::new(RefCell::new(HashMap::new())),
            // type_paths: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Vec<String> {
        self.key_paths.borrow().get(key).unwrap_or(&vec![]).clone()
    }

    pub fn parse(&self, root: View) {
        let mut stack: Vec<(View, Vec<String>)> = vec![(root, vec![])];

        while let Some((obj, key_path)) = stack.pop() {
            let obj = obj.borrow();
            Reflect::own_keys(&obj)
                .unwrap_or(Array::new())
                .iter()
                .for_each(|js_key| {
                    let key = js_key
                        .as_string()
                        .expect("Could not read object key as String.");

                    let js_value = Reflect::get(&obj, &JsValue::from_str(&key))
                        .expect(&format!("Couldn't read value for key {}", &key));

                    if key == "id" {
                        self.key_paths
                            .borrow_mut()
                            .insert(js_value.as_string().unwrap(), key_path.clone());
                    }

                    if js_value.is_object() {
                        let mut key_path = key_path.clone();

                        key_path.push(key.to_string());
                        stack.push((Rc::new(RefCell::new(js_value)), key_path))
                    }

                    // log(&format!("{:?}", key_path))
                });
        }
    }
}
