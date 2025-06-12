use crate::dice::DiceRoll;
use std::cell::Cell;
use std::fmt::Display;
use std::rc::Rc;
// An API for implementing the axiom of choice by presenting a vector of options to a user
// - can select a thing from a vector of things
// - those things need not be copy/clone
// - it need not take ownership of the vector (but optionally can)

// I had initially intended for the type to also contain other functions,
// like reminder text or a help window that explains what the option is in more detail
// these would probably all be strings
pub struct Choosable {
    pub description: String,
}

impl Choosable {
    // there isn't really a downside to doing this early, since any user-facing interface
    // is going to have to render it to a string at some point
    pub fn from(t: &impl Display) -> Self {
        Self {
            description: t.to_string(),
        }
    }
}

pub struct Selection {
    pub description: &'static str,
    // the main downside of the trait is that all the things are moved into an rc
    // it may be possible to get around this by making choosable a struct
    pub options: Vec<Choosable>,
    // this RC is how we signal home
    // Rust doesn't support passing additional arguments into generator expressions like Python
    pub chosen: Rc<Cell<usize>>,
}

pub struct TraitChoice {
    pub description: &'static str,
    pub chosen: Rc<Cell<String>>,
}

// prophetic stars lets you pick a roll two times - this represents that choice
// this is either a pure dice roll, or on a table (ie multiple numbers may have the same outcome)
// for table rolls, we can probably just use selection above with a "pick randomly" option added
pub struct PickRoll {
    pub description: &'static str,
    pub roll: Box<dyn DiceRoll>,
    pub chosen: Rc<Cell<Option<i8>>>,
}

pub struct Question {
    pub description: String,
    pub chosen: Rc<Cell<bool>>,
}

pub enum Choice {
    Selection(Selection),
    String(TraitChoice),
    PickRoll(PickRoll),
    Question(Question),
}

impl From<Selection> for Choice {
    fn from(value: Selection) -> Self {
        Choice::Selection(value)
    }
}
impl From<TraitChoice> for Choice {
    fn from(value: TraitChoice) -> Self {
        Choice::String(value)
    }
}
impl From<PickRoll> for Choice {
    fn from(value: PickRoll) -> Self {
        Choice::PickRoll(value)
    }
}

impl Choice {
    pub fn description(&self) -> &str {
        match &self {
            Choice::Selection(s) => s.description,
            Choice::String(t) => t.description,
            Choice::PickRoll(p) => p.description,
            Choice::Question(q) => &q.description,
        }
    }
}

#[macro_export]
macro_rules! choose {
    ($descr: literal, $($x: expr),*) => {
        {
            let mut orig = vec![$($x),*];
            let options = orig.iter().map(|x| crate::ipc::Choosable::from(x)).collect();
            let chosen = std::rc::Rc::from(std::cell::Cell::new(0));
            yield crate::ipc::Selection {description: ($descr), options: options, chosen: chosen.clone()}.into();
            orig.remove(chosen.get())
        }
    };
}
#[macro_export]
macro_rules! choose_vec {
    ($descr: literal, $x: ident) => {{
        let options = $x.iter().map(|x| crate::ipc::Choosable::from(x)).collect();
        let chosen = std::rc::Rc::from(std::cell::Cell::new(0));
        yield crate::ipc::Selection {
            description: ($descr),
            options: options,
            chosen: chosen.clone(),
        }
        .into();
        $x.remove(chosen.get())
    }};
    ($descr: literal, $x: expr) => {{
        // maybe we got passed an iter - if so, consume it into a vector
        let mut orig: Vec<_> = ($x).into_iter().collect();
        choose_vec!($descr, orig)
    }};
    // consume causes the macro to take ownership of the vector
    // this allows it to be rebound as mutable even if it was originally immutable
    // see tests::test_ints for an example of where this is needed
    (consume $descr: literal, $x: expr) => {{
        // put it in parentheses to ensure that it's always an expression and not an ident
        choose_vec!($descr, ($x))
    }};
}

#[macro_export]
macro_rules! input_trait {
    ($description: literal) => {{
        let chosen = std::rc::Rc::from(std::cell::Cell::new(String::new()));
        yield crate::ipc::TraitChoice {
            description: ($description),
            chosen: chosen.clone(),
        }
        .into();
        chosen.take()
    }};
}

#[macro_export]
macro_rules! pick_roll {
    ($description: literal, $roll: expr) => {{
        let roll = $roll;
        let chosen = std::rc::Rc::from(std::cell::Cell::new(None));
        yield crate::ipc::PickRoll {
            description: $description,
            roll: Box::new(roll.clone()),
            chosen: chosen.clone(),
        }
        .into();
        chosen.take()
    }};
}

// not really sure which section this macro goes in, since it does both dice rolling and rpc
// sorry i cant really separate these concerns
#[macro_export]
macro_rules! maybe_roll {
    ($description: literal, $backend: ident, $($tail:tt)*) => {{
        let roll = roll!($($tail)*);
        match $backend.get_omen() {
            Some(BirthOmen::PropheticSigns(charges)) if charges != 0 => {
                match pick_roll!($description, roll) {
                    None => crate::dice::PickedRoll(roll, roll.result()),
                    Some(i) => {
                        $backend.set_omen(BirthOmen::PropheticSigns(charges-1));
                        crate::dice::PickedRoll(roll, i)
                    },
                }
            }
            _ => crate::dice::PickedRoll(roll, roll.result()),
        }
    }};
}

#[macro_export]
macro_rules! ask {
    ($description: expr) => {{
        let answer = Rc::new(Cell::new(false));
        let question = Choice::Question(Question {
            description: $description,
            chosen: answer.clone(),
        });
        yield question;
        answer.get()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Formatter;

    fn run_test(mut iter: impl Iterator<Item = Choice>) {
        loop {
            match iter.next() {
                None => break,
                Some(Choice::String(t)) => t.chosen.set(String::from("example")),
                Some(Choice::Selection(s)) => s.chosen.set(0),
                Some(Choice::PickRoll(p)) => p.chosen.set(Some(*p.roll.range().end())),
                Some(Choice::Question(q)) => q.chosen.set(true),
            }
        }
    }

    gen fn test_ints() -> Choice {
        let vec = vec![1, 2, 3];
        let choice = choose_vec!(consume "something", vec);
        assert_eq!(choice, 1);

        let choice = choose!("something", 1, 2, 3);
        assert_eq!(choice, 1);
    }

    gen fn test_no_copy() -> Choice {
        #[derive(Debug, Eq, PartialEq)]
        struct Foo(i32);
        impl Display for Foo {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "Foo")
            }
        }

        let mut vec = vec![Foo(1), Foo(2), Foo(3)];
        let choice: Foo = choose_vec!("something", vec);
        assert_eq!(choice, Foo(1));
        assert_eq!(vec, [Foo(2), Foo(3)]);

        let choice: Foo = choose!("something", Foo(1), Foo(2), Foo(3));
        assert_eq!(choice, Foo(1));
    }

    #[test]
    fn test_simple_choices() {
        run_test(test_ints());
        run_test(test_no_copy());
    }
}
