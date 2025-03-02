use rand::Rng;

pub struct Roll {
    all_dice: Vec<i8>,
    selected_indices: Vec<usize>,
}

impl Roll {
    pub fn result(&self) -> i8 {
        self.selected_indices
            .iter()
            .map(|&i| self.all_dice[i])
            .sum()
    }
}

impl From<i8> for Roll {
    fn from(value: i8) -> Self {
        Self {
            all_dice: vec![value],
            selected_indices: vec![0],
        }
    }
}

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

pub fn d100_disadvantage(dice: i8) -> Roll {
    if dice == 0 {
        return d100().into();
    }
    let mut all_dice: Vec<i8> = (0..dice).map(|_| (d10() - 1) * 10).collect();
    let selected_dice = all_dice
        .iter()
        .enumerate()
        .min_by_key(|&(_, &v)| v)
        .map(|(index, _)| index)
        .unwrap();

    all_dice.push(d10());

    Roll {
        selected_indices: vec![selected_dice, all_dice.len() - 1],
        all_dice,
    }
}
