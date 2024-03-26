#[derive(Clone, Copy, Debug)]
pub enum BoolExpectation {
    Expect(bool),
    NoExpect,
}

impl crate::util::BoolExpect for BoolExpectation {
    fn expectation(&self) -> BoolExpectation {
        *self
    }
    fn is_met_by(&self, value: bool) -> bool {
        if let Self::Expect(expected_value) = *self {
            expected_value == value
        } else {
            // No expectations
            true
        }
    }
}
