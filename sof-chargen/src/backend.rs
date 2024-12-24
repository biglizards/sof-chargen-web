use crate::character::{Character, Stat};
use std::fmt;

pub trait Backend {
    fn choose<T: Copy + fmt::Display + Send + Sync>(
        &self,
        description: &str,
        options: &Vec<T>,
    ) -> impl std::future::Future<Output = T> + Send;
    fn set_stat(&mut self, stat: Stat, new_val: i8);
    fn get_stat(&self, stat: Stat) -> i8;
    fn gain_trait(&mut self, description: &str);
}
#[derive(Debug, Default)]
pub struct BaseBackend {
    pub character: Character,
}

impl Backend for BaseBackend {
    async fn choose<T: Copy + fmt::Display + Send + Sync>(
        &self,
        description: &str,
        options: &Vec<T>,
    ) -> T {
        *options
            .first()
            .expect("attempted to choose from 0 options!")
    }

    fn set_stat(&mut self, stat: Stat, new_val: i8) {
        self.character.stats[stat] = new_val;
    }
    fn get_stat(&self, stat: Stat) -> i8 {
        self.character.stats[stat]
    }

    fn gain_trait(&mut self, description: &str) {
        // just don't
        // normally you'd prompt the user for input and store it somewhere
        println!("{}", description)
    }
}
