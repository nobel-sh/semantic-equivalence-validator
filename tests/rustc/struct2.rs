use std::mem::size_of;

#[repr(C)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
struct StructC {
    flag: bool,
    number: i32,
    vector: Vector3,
    matrix: [[f32; 4]; 4],
    data: [u8; 16],
}

#[repr(packed)]
struct StructPacked {
    flag: bool,
    number: i32,
    vector: Vector3,
    matrix: [[f32; 4]; 4],
    data: [u8; 16],
}

#[repr(align(16))]
struct StructAligned {
    flag: bool,
    number: i32,
    vector: Vector3,
    matrix: [[f32; 4]; 4],
    data: [u8; 16],
}

#[repr(transparent)]
struct StructTransparent(StructC);

struct StructDefault {
    flag: bool,
    number: i32,
    vector: Vector3,
    matrix: [[f32; 4]; 4],
    data: [u8; 16],
}

fn verify_sizeof() {
    let vector = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let matrix = [[1.0; 4]; 4];
    let data = [0u8; 16];

    let struct_c = StructC {
        flag: true,
        number: 42,
        vector,
        matrix,
        data,
    };

    let vector = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let struct_packed = StructPacked {
        flag: true,
        number: 42,
        vector,
        matrix,
        data,
    };

    let vector = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let struct_aligned = StructAligned {
        flag: true,
        number: 42,
        vector,
        matrix,
        data,
    };

    let vector = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    let struct_c = StructC {
        flag: true,
        number: 42,
        vector,
        matrix,
        data,
    };
    let struct_transparent = StructTransparent(struct_c);

    println!("Vector3: {}", size_of::<Vector3>());
    println!("StructC: {}", size_of::<StructC>());
    println!("StructPacked: {}", size_of::<StructPacked>());
    println!("StructAligned: {}", size_of::<StructAligned>());
    println!("StructTransparent: {}", size_of::<StructTransparent>());
    println!("StructDefault: {}", size_of::<StructDefault>());
}

fn main() {
    verify_sizeof();
}
