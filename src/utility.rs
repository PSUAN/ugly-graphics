pub fn swap<A, B>((a, b): (A, B)) -> (B, A) {
    (b, a)
}

pub fn swap_if<T>((a, b): (T, T), condition: bool) -> (T, T) {
    if condition { (b, a) } else { (a, b) }
}
