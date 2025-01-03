struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// ANCHOR: here
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    //       "CustomSmartPointerがmainの終端の前でドロップされた。"
    println!("CustomSmartPointer dropped before the end of main.");
}
// ANCHOR_END: here
