use std::cell::Cell;
use std::fmt::Debug;
use std::rc::Rc;

// This API isn't perfect. My ideal form would have the following properties:
// - can select a thing from a vector of things
// - those things need not be copy/clone
// - it does not take ownership of the vector
// I'm pretty sure it's impossible to do all three, so we drop the third requirement.

// the decision to make this a trait is somewhat arbitrary
// we could instead make choosable a struct containing the debug string
// I had initially intended for the type to also contain other functions,
// like reminder text or a help window that explains what the option is in more detail
// these would probably all be static strings
pub trait Choosable: Debug {}

impl<T> Choosable for T where T: Debug {}

pub struct Selection {
    pub description: &'static str,
    // the main downside of the trait is that all the things are moved into an rc
    // it may be possible to get around this by making choosable a struct
    pub options: Vec<Rc<dyn Choosable>>,
    pub chosen: Rc<Cell<usize>>,
}

pub struct TraitChoice {
    pub description: &'static str,
    pub chosen: Rc<Cell<String>>,
}

pub enum Choice {
    Selection(Selection),
    String(TraitChoice),
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

impl Choice {
    pub fn description(&self) -> &'static str {
        match &self {
            Choice::Selection(s) => s.description,
            Choice::String(t) => t.description,
        }
    }
}

#[macro_export]
macro_rules! choose {
    ($descr: literal, $($x: expr),*) => {
        {
            let mut options = vec![$(std::rc::Rc::new($x)),*];
            let as_choosable = options.iter().map(|t| t.clone() as std::rc::Rc<dyn crate::ipc::Choosable>).collect();
            let chosen = std::rc::Rc::from(std::cell::Cell::new(0));
            yield crate::ipc::Selection {description: ($descr), options: as_choosable, chosen: chosen.clone()}.into();
            std::rc::Rc::try_unwrap(options.remove(chosen.get()))
                .expect("More than 1 strong reference to choice! Logic fucked up!")
        }
    };
}
#[macro_export]
macro_rules! choose_vec {
    ($descr: literal, $x: expr) => {{
        let mut options: Vec<_> = ($x).into_iter().map(|x| std::rc::Rc::from(x)).collect();
        let as_choosable = options
            .iter()
            .map(|t| t.clone() as std::rc::Rc<dyn crate::ipc::Choosable>)
            .collect();
        let chosen = std::rc::Rc::from(std::cell::Cell::new(0));
        yield crate::ipc::Selection {
            description: ($descr),
            options: as_choosable,
            chosen: chosen.clone(),
        }.into();
        std::rc::Rc::try_unwrap(options.remove(chosen.get()))
            .expect("More than 1 strong reference to choice! Logic fucked up!")
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

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(mut iter: impl Iterator<Item = Choice>) {
        loop {
            match iter.next() {
                None => break,
                Some(Choice::String(t)) => t.chosen.set(String::from("example")),
                Some(Choice::Selection(s)) => s.chosen.set(0)
            }
        }
    }

    gen fn test_ints() -> Choice {
        let vec = vec![1, 2, 3];
        let choice = choose_vec!("something", vec);
        assert_eq!(choice, 1);

        let choice = choose!("something", 1, 2, 3);
        assert_eq!(choice, 1);
    }

    gen fn test_no_copy() -> Choice {
        #[derive(Debug, Eq, PartialEq)]
        struct Foo(i32);

        let vec = vec![Foo(1), Foo(2), Foo(3)];
        let choice: Foo = choose_vec!("something", vec);
        assert_eq!(choice, Foo(1));

        let choice: Foo = choose!("something", Foo(1), Foo(2), Foo(3));
        assert_eq!(choice, Foo(1));
    }

    #[test]
    fn test_simple_choices() {
        run_test(test_ints());
        run_test(test_no_copy());
    }
}
