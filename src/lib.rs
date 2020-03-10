extern crate snowball;

use std::borrow::Cow;

use snowball::SnowballEnv;

mod languages {
    #![allow(non_upper_case_globals)]
    #![allow(non_snake_case)]
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    
    pub mod russian {
        include!(concat!(env!("OUT_DIR"), "/russian.rs"));
    }
    pub mod english {
        include!(concat!(env!("OUT_DIR"), "/english.rs"));
    }
}

#[derive(Debug)]
pub enum Stemmer {
    Russian,
    English,
}

impl Stemmer {
    pub fn stem<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut env = SnowballEnv::create(input);
        match self {
            Stemmer::Russian => languages::russian::stem(&mut env),
            Stemmer::English => languages::english::stem(&mut env),
        };
        env.get_current()
    }
}

    
