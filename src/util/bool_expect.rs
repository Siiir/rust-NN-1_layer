use super::BoolExpectation;

pub trait BoolExpect {
    fn expectation(&self) -> BoolExpectation;
    fn is_met_by(&self, value: bool) -> bool {
        self.expectation().is_met_by(value)
    }
}
impl BoolExpect for bool {
    fn expectation(&self) -> BoolExpectation {
        BoolExpectation::Expect(*self)
    }
    fn is_met_by(&self, value: bool) -> bool {
        value == *self
    }
}
