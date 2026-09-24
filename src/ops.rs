use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use crate::Float;

impl Add for Float {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let max_scale = self.scale.max(rhs.scale);
        let left_val = Self::scale_up(self.value, self.scale, max_scale);
        let right_val = Self::scale_up(rhs.value, rhs.scale, max_scale);
        Self::new(left_val + right_val, max_scale)
    }
}

impl Sub for Float {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let max_scale = self.scale.max(rhs.scale);
        let left_val = Self::scale_up(self.value, self.scale, max_scale);
        let right_val = Self::scale_up(rhs.value, rhs.scale, max_scale);
        Self::new(left_val - right_val, max_scale)
    }
}

impl Mul for Float {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let new_value = self.value * rhs.value;
        let new_scale = self.scale + rhs.scale;
        Self::new(new_value, new_scale)
    }
}

impl Div for Float {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.value == 0 {
            panic!("Division by zero");
        }

        let target_scale = self.scale.max(rhs.scale);
        let temp_scale = target_scale + rhs.scale + 1;
        let amplified_left = Self::scale_up(self.value, self.scale, temp_scale);

        let raw_div = amplified_left / rhs.value;
        let last_digit = (raw_div % 10).abs();
        let mut final_value = raw_div / 10;

        if last_digit >= 5 {
            if raw_div >= 0 {
                final_value += 1;
            } else {
                final_value -= 1;
            }
        }

        Self::new(final_value, target_scale)
    }
}

impl Rem for Float {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        if rhs.value == 0 {
            panic!("Division by zero during rem");
        }
        let max_scale = self.scale.max(rhs.scale);
        let left_val = Self::scale_up(self.value, self.scale, max_scale);
        let right_val = Self::scale_up(rhs.value, rhs.scale, max_scale);
        Self::new(left_val % right_val, max_scale)
    }
}

impl AddAssign for Float {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs; 
    }
}

impl SubAssign for Float {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Float {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for Float {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl RemAssign for Float {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}