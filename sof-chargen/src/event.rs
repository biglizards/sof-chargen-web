use crate::backend::Backend;
use crate::character::Stat;
use crate::dice::{DiceRoll, MagicDice, d100};
use crate::ipc::Choice;
use crate::{CORE_STATS, choose, choose_vec, roll, input_trait};

pub gen fn prosperous_constellations<T: Backend>(backend: &T) -> Choice {
    // reroll luck
    let new_luck = d100().result();
    // choice: keep either value
    let choice = choose![
        "Pick either value",
        backend.get_stat(Stat::Luck).unwrap_or_default(),
        new_luck
    ];
    backend.set_stat(Stat::Luck, &choice);

    let trt = input_trait!("gain a trait related to arrogance, vanity or overconfidence");
    backend.gain_trait(trt);
}

pub gen fn pick_stat<T: Backend>(backend: &T) -> Choice {
    let core_stat = choose_vec!(
        "Pick a core stat to roll next",
        CORE_STATS
            .into_iter()
            .filter(|&x| backend.get_stat(x).is_none())
    );

    // in character creation we pick the lowest of all rolled dice,
    // which is the same as rolling with advantage
    let num_dice = 2 + CORE_STATS
        .into_iter()
        .filter(|&x| backend.get_stat(x).is_some_and(|x| x >= 50))
        .count();
    let roll = roll!(kl(num_dice d 100));
    backend.set_stat(core_stat, &roll);
    let stat = roll.result();

    let mut remaining_stats: Vec<Stat> = core_stat
        .subskills()
        .into_iter()
        .filter(|&x| backend.get_stat(x).is_none())
        .collect();

    for i in 0..3 {
        let choice = choose_vec!("pick a sub-skill", remaining_stats);
        let mallus_roll = roll!(stat - i d 10);
        backend.set_stat(choice, &mallus_roll);
    }
}

pub fn roll_magic<T: Backend>(backend: &T) {
    let roll = MagicDice::roll();
    if roll.result() >= 100 {
        println!("You died during character creation!");
    }

    backend.set_stat(Stat::Magic, &roll);
}
pub fn roll_luck<T: Backend>(backend: &T) {
    backend.set_stat(Stat::Luck, &d100());
}

pub fn roll_core_stats<T: Backend>(backend: &T) -> impl Iterator<Item = Choice> {
    pick_stat(backend)
        .chain(pick_stat(backend))
        .chain(pick_stat(backend))
        .chain(pick_stat(backend))
        .chain(pick_stat(backend))
}
