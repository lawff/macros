use macros::AutoDeref;

#[allow(unused)]
#[derive(Debug, AutoDeref)]
#[auto_deref(field = "inner")]
pub struct RespBulkString {
    inner: String,
    nothing: (),
}

fn main() {
    let s = RespBulkString {
        inner: "hellosssss".to_string(),
        nothing: (),
    };
    println!("{:?}", s.len());
}
