extern "C" {
    fn printf(format: *const i8, ...);
}

fn basic_function_pointer_test() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let fn_ptr: fn(i32, i32) -> i32 = add;
    unsafe {
        printf("Function Pointer Tests:\n\0" as *const str as *const i8);
        printf(
            "add(2, 3) = %d\n\0" as *const str as *const i8,
            fn_ptr(2, 3),
        );
    }
}

fn multi_level_indirection_test() {
    fn square(x: i32) -> i32 {
        x * x
    }

    let fn_ptr: fn(i32) -> i32 = square;
    let ptr_to_fn_ptr: *const fn(i32) -> i32 = &fn_ptr;
    let ptr_to_ptr_to_fn_ptr: *const *const fn(i32) -> i32 = &ptr_to_fn_ptr;
    unsafe {
        printf("Multi-Level Indirection Test:\n\0" as *const str as *const i8);
        let result = (*(*ptr_to_ptr_to_fn_ptr))(7);
        printf("square(7) = %d\n\0" as *const str as *const i8, result);
    }
}

fn mutability_test() {
    fn increment(x: i32) -> i32 {
        x + 1
    }

    fn decrement(x: i32) -> i32 {
        x - 1
    }

    let mut fn_ptr: fn(i32) -> i32 = increment;

    unsafe {
        printf("Mutability Test Cases:\n\0" as *const str as *const i8);

        let initial_result = fn_ptr(42);
        printf(
            "Initial result (increment): %d\n\0" as *const str as *const i8,
            initial_result,
        );

        fn_ptr = decrement;
        let updated_result = fn_ptr(42);
        printf(
            "Updated result (decrement): %d\n\0" as *const str as *const i8,
            updated_result,
        );

        fn_ptr = increment;
        let final_result = fn_ptr(42);
        printf(
            "Final result (increment again): %d\n\0" as *const str as *const i8,
            final_result,
        );
    }
}

struct OuterStruct {
    inner: InnerStruct,
}

struct InnerStruct {
    function: fn(i32) -> i32,
}

fn nested_struct_function_pointer_test() {
    fn double(x: i32) -> i32 {
        x * 2
    }

    let outer = OuterStruct {
        inner: InnerStruct { function: double },
    };

    unsafe {
        printf("Nested Struct Function Pointer Test:\n\0" as *const str as *const i8);

        let result = (outer.inner.function)(21);
        printf("double(21) = %d\n\0" as *const str as *const i8, result);
    }
}

fn main() -> i32 {
    basic_function_pointer_test();
    multi_level_indirection_test();
    mutability_test();
    nested_struct_function_pointer_test();
    0
}
