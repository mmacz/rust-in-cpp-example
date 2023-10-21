#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust crate");
}

#[repr(C)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

#[no_mangle]
pub extern "C" fn calculate_area(rect: &Rectangle) -> u64 {
    (rect.width * rect.height).into()
}