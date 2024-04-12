pub fn clamp<T: PartialOrd>(num: T, min: T, max: T) -> T {
    if num <= min {
        return min;
    }
    if num >= max {
        return max;
    }
    return num;
}
