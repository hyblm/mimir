use crate::GameInstance;
use crate::{Mark::*, Target};

#[test]
fn test2() {
    let rating = GameInstance::new(["small", "group"]).score_guess(*b"order");
    if let Target::Miss(feedback) = rating {
        assert_eq!(feedback, [Misput, Solved, Absent, Absent, Absent])
    }
}
#[test]
fn seven_in_swamp_teeth() {
    let rating = GameInstance::new(["swamp", "teeth"]).score_guess(*b"seven");
    if let Target::Miss(feedback) = rating {
        assert_eq!(feedback, [Solved, Solved, Absent, Misput, Absent])
    }
}
#[test]
fn yellow_before_green() {
    let rating = GameInstance::new(["caccc", "ddddd"]).score_guess(*b"aabbb");
    if let Target::Miss(feedback) = rating {
        assert_eq!(feedback, [Absent, Solved, Absent, Absent, Absent])
    }
}
