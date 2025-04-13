#[derive(Clone)]
pub enum GUIEvent {
    Choose(usize),
    SubmitTrait(String),
    PickRoll(i8),
    ResetAll,
}

impl GUIEvent {
    pub(crate) fn should_advance(&self) -> bool {
        matches!(
            self,
            // these are the gui events corresponding to IPCs
            // ie thees ones should cause the event iter to advance now that we're done responding
            Self::Choose(_) | Self::SubmitTrait(_) | Self::PickRoll(_)
        )
    }
}
