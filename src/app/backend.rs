use sof_chargen::{Backend, Character, Stat};
use std::fmt;
use std::sync::{mpsc, Arc, RwLock};

#[derive(Clone)]
pub struct AppBackend {
    pub character: Arc<RwLock<Character>>,
    pub choice_vec: mpsc::Sender<(String, Vec<String>, async_channel::Sender<usize>)>,
    pub trait_send: mpsc::Sender<(String, async_channel::Sender<String>)>,
}

impl Backend for AppBackend {
    async fn choose<T: Copy + fmt::Display>(&self, description: &str, options: &Vec<T>) -> T {
        let (s, r) = async_channel::bounded(1);
        self.choice_vec
            .send((
                description.to_string(),
                options.iter().map(|x| x.to_string()).collect(),
                s,
            ))
            .unwrap();
        let choice = r.recv().await.unwrap();
        options[choice]
    }

    fn set_stat(&mut self, stat: Stat, new_val: i8) {
        self.character.write().unwrap().stats[stat] = Some(new_val);
    }

    fn get_stat(&self, stat: Stat) -> Option<i8> {
        self.character.read().unwrap().stats[stat]
    }
    async fn gain_trait(&mut self, description: &str) {
        let (s, r) = async_channel::bounded(1);
        self.trait_send.send((description.to_string(), s)).unwrap();
        let thing = r.recv().await.unwrap();
        {
            self.character.write().unwrap().traits.push(thing);
        }
    }
}
