pub fn sum(a: u8, b: u8) -> Option<u8> {
    a.checked_add(b)
}

pub fn diff(a: i16, b: i16) -> Option<i16> {
    a.checked_sub(b)
}

pub fn pro(a: i8, b: i8) -> Option<i8> {
    a.checked_mul(b)
}

pub fn quo(a: f32, b: f32) -> Option<f32> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

pub fn rem(a: f32, b: f32) -> Option<f32> {
    if b == 0.0 {
        None
    } else {
        Some(a % b)
    }
}