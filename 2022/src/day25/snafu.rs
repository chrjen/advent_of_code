use std::{
    collections::VecDeque,
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Snafu<T>(T)
where
    T: num::Integer;

impl<T> Copy for Snafu<T> where T: num::Integer + Copy {}

impl<T> From<T> for Snafu<T>
where
    T: num::Integer,
{
    fn from(value: T) -> Self {
        Snafu(value)
    }
}

impl Display for Snafu<u64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut n = self.0;

        let mut output = VecDeque::new();

        while n > 0 {
            let d = n % 5;
            let (c, add) = match d {
                0 => ('0', 0),
                1 => ('1', 0),
                2 => ('2', 0),
                3 => ('=', 2),
                4 => ('-', 1),
                _ => unreachable!(),
            };
            n = (n + add) / 5;

            output.push_front(c);
        }

        write!(f, "{}", output.into_iter().collect::<String>())
    }
}

impl FromStr for Snafu<u64> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut n: u64 = 0;

        for (i, c) in s.chars().enumerate() {
            n *= 5;
            match c {
                '0' => {}
                '1' => n += 1,
                '2' => n += 2,
                '=' => n -= 2,
                '-' => n -= 1,
                _ => {
                    return Err(format!(
                        "illegal character '{c}' at index {i} while parsing \"{s}\""
                    ))
                }
            }
        }

        Ok(Snafu(n))
    }
}

impl<T> Sum for Snafu<T>
where
    T: num::Integer,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Snafu(T::zero()), |acc, v| acc + v)
    }
}

impl<T> Add for Snafu<T>
where
    T: num::Integer,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T> AddAssign for Snafu<T>
where
    T: num::Integer + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl<T> Sub for Snafu<T>
where
    T: num::Integer,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<T> SubAssign for Snafu<T>
where
    T: num::Integer + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl<T> Mul for Snafu<T>
where
    T: num::Integer,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<T> MulAssign for Snafu<T>
where
    T: num::Integer + MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl<T> Div for Snafu<T>
where
    T: num::Integer,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl<T> DivAssign for Snafu<T>
where
    T: num::Integer + DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl<T> Rem for Snafu<T>
where
    T: num::Integer,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl<T> RemAssign for Snafu<T>
where
    T: num::Integer + RemAssign,
{
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, "1")]
    #[case(2, "2")]
    #[case(3, "1=")]
    #[case(4, "1-")]
    #[case(5, "10")]
    #[case(6, "11")]
    #[case(7, "12")]
    #[case(8, "2=")]
    #[case(9, "2-")]
    #[case(10, "20")]
    #[case(15, "1=0")]
    #[case(20, "1-0")]
    #[case(2022, "1=11-2")]
    #[case(12345, "1-0---0")]
    #[case(314159265, "1121-1110-1=0")]
    fn snafu_display(#[case] input: u64, #[case] expected: &str) {
        assert_eq!(format!("{}", Snafu(input)), expected)
    }

    #[rstest]
    #[case("1=-0-2", 1747)]
    #[case("12111", 906)]
    #[case("2=0=", 198)]
    #[case("21", 11)]
    #[case("2=01", 201)]
    #[case("111", 31)]
    #[case("20012", 1257)]
    #[case("112", 32)]
    #[case("1=-1=", 353)]
    #[case("1-12", 107)]
    #[case("12", 7)]
    #[case("1=", 3)]
    #[case("122", 37)]
    fn snafu_parse(#[case] input: &str, #[case] expected: u64) {
        let snafu = input.parse::<Snafu<u64>>().unwrap();
        assert_eq!(snafu.0, expected)
    }
}
