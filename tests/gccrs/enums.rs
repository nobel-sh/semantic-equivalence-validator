#![feature(intrinsics)]
#[lang = "sized"]
pub trait Sized {}

extern "C" {
    fn printf(format: *const i8, ...);
}

extern "rust-intrinsic" {
    fn size_of<T>() -> usize;
}

enum Empty {}

#[repr(u8)]
enum Fieldless {
    A,
    B,
    C,
}

#[repr(C)]
enum FieldlessC {
    A,
    B,
    C,
}

#[repr(u16)]
enum DataInt {
    A(u32),
    B { x: f64 },
    C,
}

#[repr(C)]
enum DataC {
    A(u32),
    B { x: f64 },
    C,
}

#[repr(C, u8)]
enum DataCInt {
    A(u32),
    B { x: f64 },
    C,
}

fn test_enum_sizes() {
    unsafe {
        printf("Enum sizes:\n\0" as *const str as *const i8);
        printf(
            "Fieldless (u8): %d\n\0" as *const str as *const i8,
            size_of::<Fieldless>(),
        );
        printf(
            "Fieldless (C): %d\n\0" as *const str as *const i8,
            size_of::<FieldlessC>(),
        );
        printf(
            "Data (u16): %d\n\0" as *const str as *const i8,
            size_of::<DataInt>(),
        );
        printf(
            "Data (C): %d\n\0" as *const str as *const i8,
            size_of::<DataC>(),
        );
        printf(
            "Data (C, u8): %d\n\0" as *const str as *const i8,
            size_of::<DataCInt>(),
        );
    }
}

// fn test_discriminants() {
//     unsafe {
//         printf("Discriminant values:\n\0" as *const str as *const i8);
//         printf(
//             "Fieldless::A as u8 = %d\n\0" as *const str as *const i8,
//             Fieldless::A as u8 as u32,
//         );
//         printf(
//             "FieldlessC::C as u8 = %d\n\0" as *const str as *const i8,
//             FieldlessC::C as u8 as u32,
//         );
//     }
// }

fn test_variant_sizes() {
    unsafe {
        printf("Variant sizes:\n\0" as *const str as *const i8);
        printf(
            "Size of u32 variant: %d\n\0" as *const str as *const i8,
            size_of::<u32>(),
        );
        printf(
            "Size of f64 variant: %d\n\0" as *const str as *const i8,
            size_of::<f64>(),
        );
        printf(
            "Size of unit variant: %d\n\0" as *const str as *const i8,
            size_of::<()>(),
        );
    }
}

fn main() -> i32 {
    test_enum_sizes();
    // test_discriminants();
    test_variant_sizes();
    0
}
