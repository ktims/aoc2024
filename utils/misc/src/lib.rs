use num_traits::Signed;
use std::fmt::Display;
use std::ops::{Add, AddAssign};

/// Wrapped signed integer with custom upper bound with wrapping of 0s to the upper bound
#[derive(Eq, Clone, Copy)]
pub struct CustomWrapped<T: Signed + Copy> {
    pub val: T,
    pub bound: T,
}

impl<T: Signed + Copy> Add<T> for CustomWrapped<T> {
    type Output = CustomWrapped<T>;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            val: ((self.val + rhs % self.bound) + self.bound) % self.bound,
            bound: self.bound,
        }
    }
}

impl<T: Signed + Copy> Add<T> for &CustomWrapped<T> {
    type Output = CustomWrapped<T>;
    fn add(self, rhs: T) -> Self::Output {
        CustomWrapped {
            val: ((self.val + rhs % self.bound) + self.bound) % self.bound,
            bound: self.bound,
        }
    }
}

impl<T: Signed + Copy> AddAssign<T> for CustomWrapped<T> {
    fn add_assign(&mut self, rhs: T) {
        self.val = ((self.val + rhs % self.bound) + self.bound) % self.bound
    }
}

impl<T: Signed + Copy> CustomWrapped<T> {
    pub fn new(val: T, bound: T) -> Self {
        Self { val, bound }
    }
}

impl<T: Signed + Copy + PartialEq> PartialEq for CustomWrapped<T> {
    fn eq(&self, other: &Self) -> bool {
        self.val.eq(&other.val)
    }
}

impl<T: Signed + PartialOrd + Copy> PartialOrd for CustomWrapped<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl<T: Signed + Ord + Copy> Ord for CustomWrapped<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl<T: Signed + PartialEq + Copy> PartialEq<T> for CustomWrapped<T> {
    fn eq(&self, other: &T) -> bool {
        self.val == *other
    }
}

impl<T: Signed + PartialOrd + Copy> PartialOrd<T> for CustomWrapped<T> {
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.val.partial_cmp(other)
    }
}

impl<T: Display + Signed + Copy> Display for CustomWrapped<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.val.fmt(f)
    }
}

// impl<T> Into<T> for CustomWrapped<T> {
//     fn into(self) -> T {
//         self.val
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
