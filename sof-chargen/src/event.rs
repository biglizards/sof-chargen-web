use crate::backend::Backend;
use crate::character::Stat;
use crate::dice::d100;
use crate::ipc::{Choice, Selection, TraitChoice};
use crate::{CORE_STATS, choose, choose_vec, dice, input_trait};
use std::rc::Rc;

pub trait Event {
    fn current_choice(&self) -> Rc<Option<Choice>>;
    fn choose(&mut self, choice: usize);
    fn submit_trait(&mut self, choice: String);
}

// Similar to Iterator::Peekable, but with two major differences:
// 1. it advances to the next step immediately on being constructed, so peek need not take a &mut
// 2. it stores the current in a Rc, rather than returning a reference
//    this allows holding the peeked item while advancing the iterator
pub struct Event_<I>
where
    I: Iterator<Item = Choice>,
{
    current_choice: Rc<Option<Choice>>,
    iter: I,
}

impl<I> Event_<I>
where
    I: Iterator<Item = Choice>,
{
    fn new(mut iter: I) -> Self {
        Self {
            current_choice: Rc::new(iter.next()),
            iter,
        }
    }

    fn advance(&mut self) {
        self.current_choice = Rc::new(self.iter.next());
    }
}

impl<I: Iterator<Item = Choice>> Event for Event_<I> {
    fn current_choice(&self) -> Rc<Option<Choice>> {
        self.current_choice.clone()
    }

    fn choose(&mut self, choice: usize) {
        match &*self.current_choice {
            Some(Choice::Selection(s)) => *s.chosen.borrow_mut() = choice,
            _ => panic!("attempted to choose when there is no choice!"),
        }
        self.advance();
    }

    fn submit_trait(&mut self, choice: String) {
        match &*self.current_choice {
            Some(Choice::String(t)) => *t.chosen.borrow_mut() = choice,
            _ => panic!("attempted to choose when there is no choice!"),
        }
        self.advance();
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

impl<'a, I> From<I> for Box<dyn Event + 'a>
where
    I: Iterator<Item = Choice> + 'a,
{
    fn from(value: I) -> Self {
        Box::new(Event_::new(value))
    }
}

pub gen fn prosperous_constellations<T: Backend>(backend: &T) -> Choice {
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

pub gen fn pick_stat<T: Backend>(backend: &T) -> Choice {
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

pub fn roll_magic<T: Backend>(backend: &T) {
    let roll = roll_magic_dice() + roll_magic_dice();
    if roll >= 100 {
        println!("You died during character creation!");
    }

    backend.set_stat(Stat::Magic, roll);
}
pub fn roll_luck<T: Backend>(backend: &T) {
    backend.set_stat(Stat::Luck, d100());
}

pub fn roll_core_stats<T: Backend>(backend: &T) -> impl Iterator<Item = Choice> {
    pick_stat(backend)
        .chain(pick_stat(backend))
        .chain(pick_stat(backend))
        .chain(pick_stat(backend))
        .chain(pick_stat(backend))
}
