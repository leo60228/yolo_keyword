#![feature(stmt_expr_attributes, proc_macro_hygiene, custom_inner_attributes)]
#![rustfmt::skip]

use yolo_keyword::yolo;

#[yolo] fn deref(x: *const i32) -> i32 {
    *x
}

fn main() {
    let x = 5;
    let ptr: *const i32 = &x;
    let y = #[yolo] {
        deref(ptr)
    };
    println!("{}", y);
}
