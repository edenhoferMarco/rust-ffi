// all oft these ignored lints are actually occurring on the generated code.
// Disabled to avoid drowning in warnings. Only warnings regarding FFI safety are being kept
#[allow(dead_code, non_camel_case_types, non_upper_case_globals)]
mod bindings;

use bindings::{free_easy_struct, get_easy_struct};

fn main() {
    let val = extract_val_from_unsafe_call();
    println!("val: {val}");
}

fn extract_val_from_unsafe_call() -> u8 {
    let val: u8;
    unsafe {
        let easy = get_easy_struct(123);
        val = (*easy).value;
        free_easy_struct(easy);
    }

    val
}
