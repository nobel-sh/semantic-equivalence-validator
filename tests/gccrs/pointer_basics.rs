extern "C" {
    fn printf(format: *const i8, ...);
}

fn derefrence_raw_const_pointers() {
    let foo = 1337;
    let bar = true;

    let foo_ptr = &foo as *const i32;
    let bar_ptr = &bar as *const bool;
    unsafe {
        printf("Derefrence raw const pointers:\n\0" as *const str as *const i8);
        printf("foo: %d\n\0" as *const str as *const i8, *foo_ptr);
        if *bar_ptr {
            printf("bar: true\n\0" as *const str as *const i8);
        } else {
            printf("bar: false\n\0" as *const str as *const i8);
        }
    }
}

fn derefrence_raw_mut_pointers() {
    let mut foo = 1337;
    let mut bar = true;

    let foo_ptr = &mut foo as *mut i32;
    let bar_ptr = &mut bar as *mut bool;
    unsafe {
        printf("Derefrence raw mutable pointers:\n\0" as *const str as *const i8);

        printf("Before mutation\n\0" as *const str as *const i8);
        printf("foo: %d\n\0" as *const str as *const i8, *foo_ptr);
        if *bar_ptr {
            printf("bar: true\n\0" as *const str as *const i8);
        } else {
            printf("bar: false\n\0" as *const str as *const i8);
        }

        printf("After mutation\n\0" as *const str as *const i8);
        *foo_ptr = 7331;
        *bar_ptr = false;
        printf("foo: %d\n\0" as *const str as *const i8, *foo_ptr);
        if *bar_ptr {
            printf("bar: true\n\0" as *const str as *const i8);
        } else {
            printf("bar: false\n\0" as *const str as *const i8);
        }
    }
}

fn pointer_comparisons() {
    let mut a = 42;
    let mut b = 99;
    let mut c = 42;

    let a_ptr = &mut a as *mut i32;
    let b_ptr = &mut b as *mut i32;
    let c_ptr = &mut c as *mut i32;

    unsafe {
        printf("Pointer Comparisons:\n\0" as *const str as *const i8);

        printf(
            "Pointer1 == Pointer2 = %s\n\0" as *const str as *const i8,
            if a_ptr == b_ptr { "true\0" } else { "false\0" } as *const str as *const i8,
        );
        printf(
            "Pointer1 == Pointer2 = %s\n\0" as *const str as *const i8,
            if a_ptr == c_ptr { "true\0" } else { "false\0" } as *const str as *const i8,
        );

        printf(
            "Pointer1 != Pointer2 = %s\n\0" as *const str as *const i8,
            if a_ptr != b_ptr { "true\0" } else { "false\0" } as *const str as *const i8,
        );
        printf(
            "Pointer1 != Pointer2 = %s\n\0" as *const str as *const i8,
            if a_ptr != c_ptr { "true\0" } else { "false\0" } as *const str as *const i8,
        );
    }

    let foo = 1337;
    let foo_shared_1 = &foo as *const i32;
    let foo_shared_2 = &foo as *const i32;
    let foo_shared_3 = foo_shared_1;

    unsafe {
        printf(
            "foo_shared_1 == foo_shared_2 = %s\n\0" as *const str as *const i8,
            if foo_shared_1 == foo_shared_2 {
                "true\0"
            } else {
                "false\0"
            } as *const str as *const i8,
        );
        printf(
            "foo_shared_3 points to foo = %s\n\0" as *const str as *const i8,
            if foo_shared_3 == &foo as *const i32 {
                "true\0"
            } else {
                "false\0"
            } as *const str as *const i8,
        );
    }
}

fn nested_pointer_tests() {
    let x = 42;
    let x_ptr: *const i32 = &x;
    let ptr_to_ptr: *const *const i32 = &x_ptr;
    let ptr_to_ptr_to_ptr: *const *const *const i32 = &ptr_to_ptr;

    unsafe {
        printf("Nested pointer tests:\n\0" as *const str as *const i8);
        let ref1: &i32 = &**(&*ptr_to_ptr);
        let ref2: &i32 = &***(&*ptr_to_ptr_to_ptr);
        printf("ref1 = %d\n\0" as *const str as *const i8, *ref1);
        printf("ref2 = %d\n\0" as *const str as *const i8, *ref2);
    }
}

// fn pointer_arithmetic() {
//     unsafe {
//         printf("Pointer Arithmetic:\n\0" as *const str as *const i8);

//         let mut array: [i32; 5] = [10, 20, 30, 40, 50];
//         let base_ptr = &array as *const i32;

//         let mut index = 0;

//         while index < 5 {
//             let current_ptr = (base_ptr as u64 + index) as *const i32;
//             printf(
//                 "Index: Pointer (%p) = Value (%d)\n\0" as *const str as *const i8,
//                 current_ptr,
//                 *current_ptr,
//             );
//         }
//         index += 1;
//     }
// }

fn main() -> i32 {
    derefrence_raw_const_pointers();
    derefrence_raw_mut_pointers();
    pointer_comparisons();
    nested_pointer_tests();
    // pointer_arithmetic();
    0
}
