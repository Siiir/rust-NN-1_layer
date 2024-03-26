use num_traits::{One, Zero};

pub struct OneAbsorbingSubtractions;
fn panic_pupet() -> ! {
    panic!("Logic error: Puppet method called.")
}
impl Zero for OneAbsorbingSubtractions {
    fn zero() -> Self {
        panic_pupet()
    }

    fn is_zero(&self) -> bool {
        false
    }
}
impl One for OneAbsorbingSubtractions {
    fn one() -> Self {
        Self
    }
}
impl std::ops::Add for OneAbsorbingSubtractions {
    type Output = Self;

    fn add(self, _: Self) -> Self::Output {
        panic_pupet()
    }
}
impl std::ops::Mul for OneAbsorbingSubtractions {
    type Output = Self;

    fn mul(self, _: Self) -> Self::Output {
        panic_pupet()
    }
}
impl std::ops::SubAssign for OneAbsorbingSubtractions {
    fn sub_assign(&mut self, _: Self) {
        // Ignore subtraction
    }
}
