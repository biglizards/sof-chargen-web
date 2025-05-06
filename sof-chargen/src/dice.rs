use rand::Rng;
use std::ops::RangeInclusive;

pub trait AsPool {
    // the dice pool may be computed dynamically, so it needs to be allocated dynamically too
    // this causes un-needed copies sometimes. TODO is there a way to only copy when needed?
    fn as_pool(&self) -> Vec<i8>;

    // assuming that a pool always has dice of the same kind, return a representative dice
    fn underlying(&self) -> impl DiceRoll;
}

pub trait DiceRoll {
    fn result(&self) -> i8;
    fn render(&self) -> String;
    // assuming that a dice always returns a consecutive range of numbers, what is that range?
    fn range(&self) -> RangeInclusive<i8>;

    fn render_nested(&self) -> String {
        format!("({})", self.render())
    }
    fn render_result(&self) -> String {
        format!("{} = {}", self.render(), self.result())
    }
}

impl DiceRoll for i8 {
    fn result(&self) -> i8 {
        *self
    }

    fn render(&self) -> String {
        self.to_string()
    }

    fn range(&self) -> RangeInclusive<i8> {
        *self..=*self
    }

    fn render_nested(&self) -> String {
        self.to_string()
    }
}

impl<T: DiceRoll, const N: i8> std::ops::Add<T> for D<N> {
    type Output = Add<D<N>, T>;

    fn add(self, rhs: T) -> Self::Output {
        Add(self, rhs)
    }
}

pub struct Add<T1: DiceRoll, T2: DiceRoll>(T1, T2);

impl<T1: DiceRoll, T2: DiceRoll> DiceRoll for Add<T1, T2> {
    fn result(&self) -> i8 {
        self.0.result() + self.1.result()
    }

    fn render(&self) -> String {
        format!("{} + {}", self.0.render_nested(), self.1.render_nested())
    }

    fn range(&self) -> RangeInclusive<i8> {
        let r1 = self.0.range();
        let r2 = self.1.range();
        (r1.start() + r2.start())..=(r1.end() + r2.end())
    }
}

#[derive(Clone)]
pub struct Subtract<T1: DiceRoll, T2: DiceRoll>(pub T1, pub T2);

impl<T1: DiceRoll, T2: DiceRoll> DiceRoll for Subtract<T1, T2> {
    fn result(&self) -> i8 {
        self.0.result() - self.1.result()
    }

    fn render(&self) -> String {
        format!("{} - {}", self.0.render_nested(), self.1.render_nested())
    }

    fn range(&self) -> RangeInclusive<i8> {
        let r1 = self.0.range();
        let r2 = self.1.range();
        (r1.start() - r2.end())..=(r1.end() - r2.start())
    }
}

/// D100Pools get special treatment since they have weird interactions with advantage,
/// and want to be rendered in a way that makes them look clearly 100-y
/// many tens may be rolled when rolling with (dis)?advantage, but only ever a single units die
/// both the 10s and unit dice range from 0-9/00-90 rather than the usual 1-10, and 00+0 = 100
///
/// PickHighest<D100Pool<2>> should look like (10, *00*) + 0 = 100
/// but the only sane implementation I can think of would write it as  (10, *100*) = 100
/// this is fine for now, i guess.
#[derive(Clone)]
pub struct D100Pool {
    d100s: Vec<i8>,
    d10: i8,
}

impl D100Pool {
    pub fn roll(n: usize) -> Self {
        Self {
            d100s: (0..n)
                .map(|_| rand::rng().random_range(0..=9) * 10)
                .collect(),
            d10: rand::rng().random_range(0..=9),
        }
    }
}
impl AsPool for D100Pool {
    fn as_pool(&self) -> Vec<i8> {
        self.d100s
            .iter()
            .map(|&x| {
                if x == 0 && self.d10 == 0 {
                    100
                } else {
                    x + self.d10
                }
            })
            .collect()
    }

    fn underlying(&self) -> impl DiceRoll {
        D::<100>(0)
    }
}

/// An N-sided die ranging from 1-N inclusive
#[derive(Clone, Copy)]
pub struct D<const N: i8>(i8);

impl<const N: i8> D<N> {
    pub fn roll() -> Self {
        Self(rand::rng().random_range(1..=N))
    }
}

impl<const N: i8> DiceRoll for D<N> {
    fn result(&self) -> i8 {
        self.0.result()
    }

    fn render(&self) -> String {
        self.0.render()
    }

    fn range(&self) -> RangeInclusive<i8> {
        1..=N
    }
}

impl<const N: i8> From<D<N>> for i8 {
    fn from(val: D<N>) -> i8 {
        val.0
    }
}

// As in, 3d6 -> Many::<6>::roll(3)
#[derive(Clone)]
pub(crate) struct Many<const N: i8>(pub Vec<D<N>>);

impl<const N: i8> Many<N> {
    pub(crate) fn roll(n: usize) -> Self {
        Self((0..n).map(|_| D::roll()).collect())
    }
}

impl<const N: i8> DiceRoll for Many<N> {
    fn result(&self) -> i8 {
        self.0.iter().map(D::result).sum()
    }

    fn render(&self) -> String {
        match self.0.len() {
            0 => "0".to_string(),
            _ => self
                .0
                .iter()
                .map(D::render)
                .collect::<Vec<String>>()
                .join("+"),
        }
    }

    fn range(&self) -> RangeInclusive<i8> {
        let min = self.0.iter().map(|x| *x.range().start()).sum();
        let max = self.0.iter().map(|x| *x.range().end()).sum();
        min..=max
    }

    fn render_nested(&self) -> String {
        match self.0.len() {
            0 => "0".to_string(),
            1 => self.0[0].render(),
            _ => format!(
                "({})",
                self.0
                    .iter()
                    .map(D::render)
                    .collect::<Vec<String>>()
                    .join("+")
            ),
        }
    }
}

impl<const N: i8> AsPool for Many<N> {
    fn as_pool(&self) -> Vec<i8> {
        self.0.iter().map(|&x| x.into()).collect()
    }

    fn underlying(&self) -> impl DiceRoll {
        self.0[0]
    }
}

fn roll_magic_dice(mut v: Vec<D<10>>) -> Vec<D<10>> {
    let val = d10();
    v.push(val);
    if matches!(val.result(), 2 | 3 | 5 | 7) { roll_magic_dice(v) } else { v }
}

#[derive(Clone)]
pub struct MagicDice(Vec<D<10>>);
impl MagicDice {
    pub(crate) fn roll() -> Self {
        Self(roll_magic_dice(roll_magic_dice(vec![])))
    }
}

impl DiceRoll for MagicDice {
    fn result(&self) -> i8 {
        self.0.iter().map(D::result).sum()
    }

    fn render(&self) -> String {
        self.0
            .iter()
            .map(D::render)
            .collect::<Vec<String>>()
            .join("+")
    }

    fn range(&self) -> RangeInclusive<i8> {
        1..=100
    }
}

#[derive(Clone)]
pub struct PickedRoll<T: DiceRoll>(pub i8, pub T);

impl<T: DiceRoll> DiceRoll for PickedRoll<T> {
    fn result(&self) -> i8 {
        self.0
    }

    fn render(&self) -> String {
        self.1.render()
    }

    fn range(&self) -> RangeInclusive<i8> {
        self.1.range()
    }
}

macro_rules! render_vantage {
    ($name: literal, $max_or_min:ident, $pool:ident) => {{
        let selected_dice = ($pool)
            .iter()
            .enumerate()
            .$max_or_min(|&(_, &v)| v)
            .map(|(index, _)| index)
            .unwrap();
        format!(
            concat!($name, "({})"),
            ($pool)
                .iter()
                .enumerate()
                .map(|(i, x)| if i == selected_dice {
                    format!("*{}*", x)
                } else {
                    x.to_string()
                })
                .collect::<Vec<String>>()
                .join(", ")
        )
    }};
}

#[derive(Clone)]
pub struct PickHighest<T: AsPool>(pub T);
impl<T: AsPool> DiceRoll for PickHighest<T> {
    fn result(&self) -> i8 {
        *self.0.as_pool().iter().max().unwrap()
    }

    fn render(&self) -> String {
        let pool = self.0.as_pool();
        render_vantage!("max", max_by_key, pool)
    }

    fn range(&self) -> RangeInclusive<i8> {
        self.0.underlying().range()
    }
}

#[derive(Clone)]
pub struct PickLowest<T: AsPool>(pub T);
impl<T: AsPool> DiceRoll for PickLowest<T> {
    fn result(&self) -> i8 {
        *self.0.as_pool().iter().min().unwrap()
    }

    fn render(&self) -> String {
        let pool = self.0.as_pool();
        render_vantage!("min", min_by_key, pool)
    }

    fn range(&self) -> RangeInclusive<i8> {
        self.0.underlying().range()
    }
}

#[macro_export]
macro_rules! roll {
    ($i:literal) => {$i};
    ($i:ident) => {$i};
    (($($tail:tt)*)) => {roll!($($tail)*)};
    (1 d $d:literal) => {$crate::dice::D::<$d>::roll()};
    ($q:tt d 100) => {$crate::dice::D100Pool::roll($q)};
    ($q:tt d $d:literal) => {$crate::dice::Many::<$d>::roll($q)};
    (kh $tail:tt) => {$crate::dice::PickHighest(roll!$tail)};
    (kl $tail:tt) => {$crate::dice::PickLowest(roll!$tail)};
    ($a:tt - $($tail:tt)*) => {$crate::dice::Subtract(roll!($a), roll!($($tail)*))};
    ($a:tt + $($tail:tt)*) => {$crate::dice::Add(roll!($a), roll!($($tail)*))};
}

// aliases -- otherwise you'd have to write D::<10>::roll() which is ugly as heck
pub fn d10() -> D<10> {
    D::roll()
}
pub fn d100() -> D<100> {
    D::roll()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_macro() {
        let i = 10;
        let _: Many<6> = roll!(i d 6);
        let _: D<6> = roll!(1 d 6);
        let _: D<6> = roll!((1 d 6));
        let _: Many<6> = roll!(2 d 6);
        let _: PickHighest<Many<6>> = roll!(kh(2 d 6));
        let _: PickHighest<D100Pool> = roll!(kh(2 d 100));
        let _: PickLowest<D100Pool> = roll!(kl(2 d 100));
        let _: D<100> = roll!(1 d 100);
        let _: Add<D<6>, i8> = roll!((1 d 6) + 1);
        let _: Add<i8, D<6>> = roll!(1 + 1 d 6);
    }

    #[test]
    fn test_d100() {
        assert_eq!(
            D100Pool {
                d100s: vec![0, 10, 90],
                d10: 0,
            }
            .as_pool(),
            [100, 10, 90]
        );
        assert_eq!(
            D100Pool {
                d100s: vec![0, 10, 90],
                d10: 1,
            }
            .as_pool(),
            [1, 11, 91]
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(Add(1, 2).result(), 3);
        assert_eq!(Add(1, 2).render(), "1 + 2");
        assert_eq!(Add(1, 2).render_result(), "1 + 2 = 3");
    }

    #[test]
    fn test_subtract() {
        assert_eq!(Subtract(1, 2).result(), -1);
        assert_eq!(Subtract(1, 2).render(), "1 - 2");
        assert_eq!(Subtract(1, 2).render_result(), "1 - 2 = -1");
    }

    #[test]
    fn test_d100_advantage() {
        let throw = D100Pool {
            d100s: vec![0, 10, 90],
            d10: 0,
        };
        assert_eq!(PickLowest(throw).render_result(), "min(100, *10*, 90) = 10");
    }
    #[test]
    fn test_d100_disadvantage() {
        let throw = D100Pool {
            d100s: vec![0, 10, 90],
            d10: 0,
        };
        assert_eq!(
            PickHighest(throw).render_result(),
            "max(*100*, 10, 90) = 100"
        );
    }

    #[test]
    fn test_range() {
        assert_eq!(5i8.range().end(), &5);
        assert_eq!(5i8.range().start(), &5);
        assert!(5i8.range().contains(&5));
        assert!(!5i8.range().contains(&6));
        assert!(!5i8.range().contains(&4));

        // D
        let d10 = D::<10>::roll();
        assert_eq!(d10.range(), 1..=10);
        assert_eq!(d10.range().start(), &1);
        assert_eq!(d10.range().end(), &10);
        assert!(d10.range().contains(&10));
        assert!(!d10.range().contains(&11));
        assert!(d10.range().contains(&1));
        assert!(!d10.range().contains(&0));

        let add = Add(d10, d10);
        assert_eq!(add.range(), 2..=20);

        let subtract = Subtract(d10, d10);
        assert_eq!(subtract.range(), -9..=9);

        let d100pool = D100Pool {
            d100s: vec![5, 99],
            d10: 4,
        };
        assert_eq!(d100pool.underlying().range(), 1..=100);

        let many = Many(vec![d10, d10]);
        assert_eq!(many.range(), 2..=20);
        assert_eq!(many.underlying().range(), 1..=10);

        let magic = MagicDice::roll();
        assert_eq!(magic.range(), 1..=100);

        let highest = PickHighest(Many(vec![d10, d10]));
        assert_eq!(highest.range(), 1..=10);

        let lowest = PickLowest(Many(vec![d10, d10]));
        assert_eq!(lowest.range(), 1..=10);
    }
}
