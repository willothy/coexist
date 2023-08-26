use yrs::{Doc, GetString, Text, Transact};

fn main() {
    let doc = Doc::new();
    let text = doc.get_or_insert_text("file1");

    let mut txn = doc.transact_mut();

    text.insert(&mut txn, 0, "Hello world!");

    txn.commit();

    println!("{}", text.get_string(&txn));
}
