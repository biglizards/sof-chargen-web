use crate::Backend;
use crate::event::{Event, birth};

#[derive(Copy, Clone, Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum LifeStage {
    Intro,
    RollStats,
    RollParents,
    RollOmens,
    Newborn,
    Events,
}

// defines the order in which one rolls events
impl LifeStage {
    pub fn next(&self, backend: &'static impl Backend) -> Option<(LifeStage, Box<dyn Event>)> {
        match self {
            LifeStage::Intro => Some((
                LifeStage::RollStats,
                Box::new(birth::roll_core_stats(backend)),
            )),
            LifeStage::RollStats => {
                // this one has no choices in, so do it before rolling parent stuff
                birth::roll_location_of_birth(backend);
                Some((
                    LifeStage::RollParents,
                    Box::new(birth::affiliation_rank_careers(backend)),
                ))
            }
            LifeStage::RollParents => {
                Some((LifeStage::RollOmens, Box::new(birth::pick_omens(backend))))
            }

            _ => None,
            // LifeStage::RollOmens => {}
            // LifeStage::Newborn => {}
            // LifeStage::Events => {}
        }
    }
}

impl Default for LifeStage {
    fn default() -> Self {
        Self::Intro
    }
}
