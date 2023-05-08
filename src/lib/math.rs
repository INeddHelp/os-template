//! Math library functions

/// Returns the absolute value of a number
pub fn abs(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}

/// Returns the maximum of two numbers
pub fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

/// Returns the minimum of two numbers
pub fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

// TODO: Add more math functions here