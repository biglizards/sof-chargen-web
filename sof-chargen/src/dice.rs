use rand::Rng;

pub fn d10() -> i8 {
    rand::thread_rng().gen_range(1..=10)
}

pub(crate) fn d100() -> i8 {
    rand::thread_rng().gen_range(1..=100)
}

#[allow(dead_code)]
pub fn d100_advantage(dice: i8) -> i8 {
    if dice == 0 {
        return d100();
    }
    let d100 = (0..dice).map(|_| (d10() - 1) * 10).max().unwrap();
    d100 + d10()
}

pub fn d100_disadvantage(dice: i8) -> i8 {
    if dice == 0 {
        return d100();
    }
    let d100 = (0..dice).map(|_| (d10() - 1) * 10).min().unwrap();
    d100 + d10()
}
