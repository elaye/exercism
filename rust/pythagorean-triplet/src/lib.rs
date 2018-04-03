// 'a', 'b', and 'c' must satisfy the following equations:
//
// a + b + c = 1000
// a^2 + b^2 = c^2
//
// By substituing 'c' by sqrt(a^2 + b^2) and simplifying
// we obtain the following equation:
//
// a = 1000 * (500 - b) / (1000 - b)
//
// 'a', 'b', and 'c' are natural integers so we need to find
// 'b' between 1 and 500 so that 'a' is an integer,
// i.e. 'b' must satisfy:
//
// 1000 * (500 - b) % (1000 - b) == 0
//
// Once we have 'b', it is then easy to deduce 'a' and 'c'
pub fn find() -> Option<u32> {
    let b = (1..501).find(|i| 1000 * (500 - i) % (1000 - i) == 0).unwrap();
    let a = 1000 * (500 - b) / (1000 - b);
    let c = 1000 - b - a;
    Some(a * b * c)
}
