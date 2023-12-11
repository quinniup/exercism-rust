pub fn is_armstrong_number(num: u32) -> bool {
    if num < 10 {
        return true;
    }
    let digit = (num as f32).log10() as u32 + 1;
    let mut ret = 0;
    let mut candidate = num as u64;
    while candidate > 0 {
        ret += (candidate % 10).pow(digit);
        candidate = candidate / 10
    }
    // convert u64
    ret == num as u64
}
