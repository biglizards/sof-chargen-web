use crate::backend::Backend;
use crate::character::Stat;
use crate::dice::d100;
use crate::ipc::{Choice, Selection, TraitChoice};
use crate::{CORE_STATS, choose, choose_vec, dice, input_trait};
use std::cell::UnsafeCell;

pub trait Event {
    fn current_choice(&self) -> Option<Choice>;
    fn choose(&self, choice: usize);
    fn submit_trait(&self, choice: String);
}
pub struct Event_<I>
where
    I: Iterator<Item = Choice>,
{
    current_choice: UnsafeCell<Option<Choice>>,
    iter: UnsafeCell<I>,
}

impl<I> Event_<I>
where
    I: Iterator<Item = Choice>,
{
    fn new(mut iter: I) -> Self {
        Self {
            current_choice: iter.next().into(),
            iter: iter.into(),
        }
    }

    unsafe fn advance(&self) {
        unsafe {
            // safety: we're single threaded, and the object is only borrowed during this function
            *self.current_choice.get() = (*self.iter.get()).next();
        }
    }
}

impl<I: Iterator<Item = Choice>> Event for Event_<I> {
    fn current_choice(&self) -> Option<Choice> {
        unsafe {
            // safety: we're single threaded, and the object is only borrowed during tinto_iter().into_iter().into_iter().his function
            (*self.current_choice.get()).clone()
        }
    }

    fn choose(&self, choice: usize) {
        unsafe {
            // safety: we're single threaded, and the object is only borrowed during this function
            if let Some(Choice::Selection(s)) = &*self.current_choice.get() {
                *s.chosen.borrow_mut() = choice;
                self.advance();
            } else {
                panic!("attempted to choose when there is no choice!");
            }
        }
    }

    fn submit_trait(&self, choice: String) {
        unsafe {
            // safety: we're single threaded, and the object is only borrowed during this function
            if let Some(Choice::String(t)) = &*self.current_choice.get() {
                *t.chosen.borrow_mut() = choice;
                self.advance();
            } else {
                panic!("attempted to choose when there is no choice!");
            }
        }
    }
}

impl<I> From<I> for Event_<I>
where
    I: Iterator<Item = Choice>,
{
    fn from(value: I) -> Self {
        Event_::new(value)
    }
}
impl<I> From<I> for Box<Event_<I>>
where
    I: Iterator<Item = Choice>,
{
    fn from(value: I) -> Self {
        Box::new(Event_::new(value))
    }
}

impl<I> From<I> for Box<dyn Event>
where
    I: Iterator<Item = Choice> + 'static,
{
    fn from(value: I) -> Self {
        Box::new(Event_::new(value))
    }
}

pub gen fn prosperous_constellations<T: Backend>(mut backend: T) -> Choice {
    // reroll luck
    let new_luck = d100();
    // choice: keep either value
    let choice = choose![
        "Pick either value",
        backend.get_stat(Stat::Luck).unwrap_or_default(),
        new_luck
    ];
    backend.set_stat(Stat::Luck, choice);

    let trt = input_trait!("gain a trait related to arrogance, vanity or overconfidence");
    backend.gain_trait(trt);
}

pub gen fn pick_stat<T: Backend>(mut backend: T) -> Choice {
    let core_stat = choose_vec!(
        "Pick a core stat to roll next",
        CORE_STATS
            .into_iter()
            .filter(|&x| backend.get_stat(x).is_none())
    );

    let roll = dice::d100_disadvantage(
        (2 + CORE_STATS
            .into_iter()
            .filter(|&x| backend.get_stat(x).is_some_and(|x| x >= 50))
            .count()) as i8,
    );
    backend.set_stat_by_roll(core_stat, &roll);

    for i in 0..3 {
        let choice = choose_vec!(
            "pick a sub-skill",
            core_stat
                .subskills()
                .into_iter()
                .filter(|&x| backend.get_stat(x).is_none())
        );
        let mallus: i8 = (0..i).map(|_| dice::d10()).sum();
        backend.set_stat(choice, (roll.result() - mallus).max(1));
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

pub fn roll_core_stats<T: Backend + Clone>(backend: T) -> impl Iterator<Item = Choice> {
    pick_stat(backend.clone())
        .chain(pick_stat(backend.clone()))
        .chain(pick_stat(backend.clone()))
        .chain(pick_stat(backend.clone()))
        .chain(pick_stat(backend))
}
