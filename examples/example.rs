extern crate initialize_cell;

use initialize_cell::InitializeCell;

static VALUE: InitializeCell<i32> = unsafe { InitializeCell::new_uninitialized() };
static OTHER_VALUE: InitializeCell<i32> = InitializeCell::new(6);

struct ExampleStruct {
    num: u32,
    string: String,
}

static GLOBAL_STRUCT: InitializeCell<ExampleStruct> = unsafe { InitializeCell::new_uninitialized() };

fn main() {
    unsafe {
        VALUE.init(5);
    }

    unsafe {
        GLOBAL_STRUCT.init(ExampleStruct { num: 10, string: "example".to_owned()  } );
    }

    println!("{}", *VALUE);
    println!("{}", *OTHER_VALUE);
    println!("num:{} string:{}", GLOBAL_STRUCT.num, GLOBAL_STRUCT.string);
}
