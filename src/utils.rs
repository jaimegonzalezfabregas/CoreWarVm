
pub fn modulo<T: TryInto<isize>, Q: TryInto<isize>>(dividend: T, divisor: Q) -> usize {
    let i_divisor = unsafe { TryInto::<isize>::try_into(divisor).unwrap_unchecked() };
    let i_dividend = unsafe { TryInto::<isize>::try_into(dividend).unwrap_unchecked() };

    return (((i_dividend % i_divisor) + i_divisor) % i_divisor) as usize;
}

