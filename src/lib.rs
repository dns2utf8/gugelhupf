#[macro_use]
extern crate nom;

mod parser;

/// The global context of this VM
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

    /// Check if a globally accessable function is registered
    ///
    /// If the property does not start with the global_name the function
    /// will prefix it
    pub fn has_function(&self, name: &str) -> bool {
        if name.starts_with(&*self.global_name) == false {
        }
        false
    }
}

struct JsObject;
