use crate::backend::{Backend, BaseBackend};
use crate::character::{Stat, CORE_STATS};
use crate::dice;
use crate::dice::d100;

pub trait Event<T: Backend> {
    fn run(&self, backend: &mut T);

    fn then<'a>(&'a self, other: fn(&mut T)) -> Box<dyn Fn(&mut T) + 'a>
    where
        T: 'a,
    {
        Box::from(move |x: &mut T| {
            self.run(x);
            other.run(x)
        })
    }
}

impl<T: Backend, F> Event<T> for F
where
    F: Fn(&mut T),
{
    fn run(&self, backend: &mut T) {
        self(backend);
    }
}

struct FileEvent<T> {
    sub_events: Vec<Box<dyn Event<T>>>,
}

impl<T: Backend> Event<T> for FileEvent<T> {
    fn run(&self, backend: &mut T) {
        for event in self.sub_events.iter() {
            event.run(backend)
        }
    }
}

pub fn prosperous_constellations<T: Backend>(backend: &mut T) {
    // reroll luck
    let new_luck = d100();
    // choice: keep either value
    backend.set_stat(
        Stat::Luck,
        backend.choose(
            "Choose either luck value",
            &vec![backend.get_stat(Stat::Luck), new_luck],
        ),
    );

    backend.gain_trait("gain a trait related to arrogance, vanity or overconfidence")
}

pub fn pick_stat<T: Backend>(backend: &mut T) {
    let choice = *backend.choose(
        "Pick which stat to generate next",
        &CORE_STATS
            .iter()
            .filter(|&&x| backend.get_stat(x) == 0)
            .collect(),
    );
    let roll = dice::d100_disadvantage(
        (2 + CORE_STATS
            .iter()
            .filter(|&&x| backend.get_stat(x) >= 50)
            .count()) as i8,
    );
    backend.set_stat(choice, roll);

    let mut subskills = choice.subskills();
    for i in 0..3 {
        let choice = backend.choose("pick a sub-skill", &subskills);
        subskills.remove(subskills.iter().position(|&x| x == choice).unwrap());
        let mallus: i8 = (0..i).map(|_| dice::d10()).sum();
        backend.set_stat(choice, (roll - mallus).max(1));
    }
}

fn is_prime(val: i8) -> bool {
    match val {
        2 | 3 | 5 | 7 => true,
        _ => false,
    }
}
fn roll_magic_dice() -> i8 {
    let val = dice::d10();
    if is_prime(val) {
        val + roll_magic_dice()
    } else {
        val
    }
}
pub fn roll_magic<T: Backend>(backend: &mut T) {
    let roll = roll_magic_dice() + roll_magic_dice();
    if roll >= 100 {
        println!("You died during character creation!");
    }

    backend.set_stat(Stat::Magic, roll);
}

pub fn roll_luck<T: Backend>(backend: &mut T) {
    backend.set_stat(Stat::Luck, d100());
}

fn test<T: Backend>() {
    let what: Vec<Box<dyn Event<T>>> = vec![Box::new(pick_stat)];
}
