use crate::backend::Backend;
use crate::character::{Stat, CORE_STATS};
use crate::dice;
use crate::dice::d100;
use async_trait::async_trait;
use std::future::IntoFuture;

#[async_trait]
pub trait Event<T: Backend> {
    async fn run(&self, backend: &mut T);
}

#[async_trait]
impl<T: Backend + Send + Sync, F, G> Event<T> for F
where
    F: Fn(&mut T) -> G + Sync + Send,
    G: std::future::Future<Output = ()> + Send + Sync,
{
    async fn run(&self, backend: &mut T) {
        self(backend).await;
    }
}

struct FileEvent<T> {
    sub_events: Vec<Box<dyn Event<T> + Sync>>,
}

#[async_trait]
impl<T: Backend + Send + Sync> Event<T> for FileEvent<T>
where
    dyn Event<T>: Send,
{
    async fn run(&self, backend: &mut T) {
        for event in self.sub_events.iter() {
            event.run(backend).await
        }
    }
}

enum BuiltinEvent {
    ProsperousConstellations,
    PickStat,
    RollMagic,
    RollLuck
}

#[async_trait]
impl<T: Backend + Send + Sync> Event<T> for BuiltinEvent
where
    dyn Event<T>: Send,
{
    async fn run(&self, backend: &mut T) {
        match self {
            BuiltinEvent::ProsperousConstellations => prosperous_constellations(backend).await,
            BuiltinEvent::PickStat => pick_stat(backend).await,
            BuiltinEvent::RollMagic => roll_magic(backend).await,
            BuiltinEvent::RollLuck => roll_luck(backend).await,
        }
    }
}


pub async fn prosperous_constellations<T: Backend>(backend: &mut T) {
    // reroll luck
    let new_luck = d100();
    // choice: keep either value
    backend.set_stat(
        Stat::Luck,
        backend
            .choose(
                "Choose either luck value",
                &vec![backend.get_stat(Stat::Luck), new_luck],
            )
            .await,
    );

    backend.gain_trait("gain a trait related to arrogance, vanity or overconfidence")
}

pub async fn pick_stat<T: Backend>(backend: &mut T) {
    let choice = *backend
        .choose(
            "Pick which stat to generate next",
            &CORE_STATS
                .iter()
                .filter(|&&x| backend.get_stat(x) == 0)
                .collect(),
        )
        .await;
    let roll = dice::d100_disadvantage(
        (2 + CORE_STATS
            .iter()
            .filter(|&&x| backend.get_stat(x) >= 50)
            .count()) as i8,
    );
    backend.set_stat(choice, roll);

    let mut subskills = choice.subskills();
    for i in 0..3 {
        let choice = backend.choose("pick a sub-skill", &subskills).await;
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
pub async fn roll_magic<T: Backend>(backend: &mut T) {
    let roll = roll_magic_dice() + roll_magic_dice();
    if roll >= 100 {
        println!("You died during character creation!");
    }

    backend.set_stat(Stat::Magic, roll);
}

pub async fn roll_luck<T: Backend>(backend: &mut T) {
    backend.set_stat(Stat::Luck, d100());
}
