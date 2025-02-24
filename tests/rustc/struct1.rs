use std::os::raw::{c_char, c_uint};

extern "C" {
    fn fprintf(stream: *mut c_char, format: *const c_char, ...);
    static stdout: *mut c_char;
}

#[repr(C)]
struct StructC {
    field1: u8,
    field2: u16,
    field3: u32,
    field4: i64,
}

#[repr(transparent)]
struct StructTransparent(u8);

#[repr(packed)]
struct StructPacked {
    field1: u8,
    field2: u16,
    field3: u32,
    field4: i64,
}

#[repr(align(8))]
struct StructAligned {
    field1: u8,
    field2: u16,
    field3: u32,
    field4: i64,
}

struct StructDefault {
    field1: u8,
    field2: u16,
    field3: u32,
    field4: i64,
    field5: [u8; 4],
}

fn print_structs() {
    let struct_c = StructC {
        field1: 255,
        field2: 65535,
        field3: 4294967295,
        field4: 9223372036854775807,
    };

    let struct_transparent = StructTransparent(128);

    let struct_packed = StructPacked {
        field1: 123,
        field2: 45678,
        field3: 987654321,
        field4: -1234567890123456789,
    };

    let struct_aligned = StructAligned {
        field1: 42,
        field2: 4242,
        field3: 42424242,
        field4: 4242424242424242,
    };

    let struct_default = StructDefault {
        field1: 1,
        field2: 2,
        field3: 3,
        field4: 4,
        field5: [33, 12, 127, 0],
    };

    unsafe {
        fprintf(
            stdout,
            b"StructC: field1 = %u, field2 = %u, field3 = %u, field4 = %lld\n\0".as_ptr()
                as *const c_char,
            struct_c.field1 as c_uint,
            struct_c.field2 as c_uint,
            struct_c.field3,
            struct_c.field4,
        );
        fprintf(
            stdout,
            b"StructTransparent: field1 = %u\n\0".as_ptr() as *const c_char,
            struct_transparent.0 as c_uint,
        );
        fprintf(
            stdout,
            b"StructPacked: field1 = %u, field2 = %u, field3 = %u, field4 = %lld\n\0".as_ptr()
                as *const c_char,
            struct_packed.field1 as c_uint,
            struct_packed.field2 as c_uint,
            struct_packed.field3,
            struct_packed.field4,
        );
        fprintf(
            stdout,
            b"StructAligned: field1 = %u, field2 = %u, field3 = %u, field4 = %lld\n\0".as_ptr()
                as *const c_char,
            struct_aligned.field1 as c_uint,
            struct_aligned.field2 as c_uint,
            struct_aligned.field3,
            struct_aligned.field4,
        );
        fprintf(
            stdout,
            b"StructDefault: field1 = %d, field2 = %d, field3 = %d, field4 = %.2f, field5 = [%d, %d, %d, %d]\n\0"
                .as_ptr() as *const c_char,
            struct_default.field1 as c_uint,
            struct_default.field2 as c_uint,
            struct_default.field3,
            struct_default.field4,
            struct_default.field5[0] as u32,
            struct_default.field5[1] as u32,
            struct_default.field5[2] as u32,
            struct_default.field5[3] as u32,

        );
    }
}

fn main() {
    print_structs();
}
