extern crate gugelhupf;

use gugelhupf::*;

#[test]
fn append_simple_function() {
    let src = "function simple() { }".into();

    let ctx = Gugelhupf::new("global");

    ctx.append_sourcecode(&src);

    assert!(ctx.has_function("global.simple"));
}
