pub fn swap<A, B>((a, b): (A, B)) -> (B, A) {
    (b, a)
}

pub fn swap_if<T>(condition: bool, (a, b): (T, T)) -> (T, T) {
    if condition { (b, a) } else { (a, b) }
}
