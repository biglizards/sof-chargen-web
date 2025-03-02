use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Choosable: Debug {}

impl<T> Choosable for T where T: Debug {}

#[derive(Clone)]
pub struct Selection {
    pub description: &'static str,
    pub options: Vec<Rc<dyn Choosable>>,
    pub(crate) chosen: Rc<RefCell<usize>>,
}

#[derive(Clone)]
pub struct TraitChoice {
    pub description: &'static str,
    pub(crate) chosen: Rc<RefCell<String>>,
}

#[derive(Clone)]
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
            let options = vec![$(std::rc::Rc::new($x)),*];
            let as_choosable = options.iter().map(|t| t.clone() as std::rc::Rc<dyn crate::ipc::Choosable>).collect();
            let chosen = std::rc::Rc::from(std::cell::RefCell::new(0));
            yield Selection {description: ($descr), options: as_choosable, chosen: chosen.clone()}.into();
            *options[*chosen.borrow()]
        }
    };
}
#[macro_export]
macro_rules! choose_vec {
    ($descr: literal, $x: expr) => {{
        let options: Vec<_> = ($x).into_iter().map(|x| std::rc::Rc::from(x)).collect();
        let as_choosable = options
            .iter()
            .map(|t| t.clone() as std::rc::Rc<dyn crate::ipc::Choosable>)
            .collect();
        let chosen = std::rc::Rc::from(std::cell::RefCell::new(0));
        yield Selection {
            description: ($descr),
            options: as_choosable,
            chosen: chosen.clone(),
        }
        .into();
        *options[*chosen.borrow()]
    }};
}

#[macro_export]
macro_rules! input_trait {
    ($description: literal) => {{
        let chosen = std::rc::Rc::from(std::cell::RefCell::new(String::new()));
        yield TraitChoice {
            description: ($description),
            chosen: chosen.clone(),
        }
        .into();
        chosen.take()
    }};
}
