use time::PrimitiveDateTime as DateTime;
use std::ops::Add;
use time::ext::NumericalDuration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let n: i64 = 10_i32.pow(9).try_into().unwrap();
    start + n.seconds()
}
