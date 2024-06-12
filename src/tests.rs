use crate::GameInstance;
use crate::{Letter::*, Target};

#[test]
fn test2() {
    let rating = GameInstance::new(["small", "group"]).rate_guess(*b"order");
    if let Target::Miss(feedback) = rating {
        assert_eq!(feedback, [Misput, Found, Absent, Absent, Absent])
    }
}
#[test]
fn seven_in_swamp_teeth() {
    let rating = GameInstance::new(["swamp", "teeth"]).rate_guess(*b"seven");
    if let Target::Miss(feedback) = rating {
        assert_eq!(feedback, [Found, Found, Absent, Misput, Absent])
    }
}
#[test]
fn yellow_before_green() {
    let rating = GameInstance::new(["caccc", "ddddd"]).rate_guess(*b"aabbb");
    if let Target::Miss(feedback) = rating {
        assert_eq!(feedback, [Absent, Found, Absent, Absent, Absent])
    }
}
