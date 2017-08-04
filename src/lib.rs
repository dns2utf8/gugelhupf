#[macro_use]
extern crate nom;

mod parser;

pub struct Gugelhupf {
    global_name: String,
    objects: Vec<JsObject>,
}

impl Gugelhupf {
    pub fn new<S>(name: S) -> Gugelhupf where S: Into<String> {
        Gugelhupf {
            global_name: name.into(),
            objects: vec![],
        }
    }

    pub fn append_sourcecode(&self, code: &String) -> &Gugelhupf {
        self
    }

    pub fn has_function<S>(&self, name: S) -> bool where S: Into<String> {
        false
    }
}

struct JsObject;
