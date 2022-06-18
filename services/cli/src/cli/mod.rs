use std::convert::TryFrom;

use ::clap::Parser;

pub mod adapter;
pub mod benchmark;
pub mod clap;
pub mod sub;
pub mod testbed;
pub mod wide;

use crate::cli::clap::CliBencher;
use crate::BencherError;
use sub::map_sub;
use sub::Sub;
use wide::Wide;

#[derive(Debug)]
pub struct Bencher {
    wide: Wide,
    sub: Option<Sub>,
}

impl TryFrom<CliBencher> for Bencher {
    type Error = BencherError;

    fn try_from(bencher: CliBencher) -> Result<Self, Self::Error> {
        Ok(Self {
            wide: Wide::try_from(bencher.wide)?,
            sub: map_sub(bencher.sub)?,
        })
    }
}

impl Bencher {
    pub fn new() -> Result<Self, BencherError> {
        let args = CliBencher::parse();
        Self::try_from(args)
    }

    pub async fn run(&self) -> Result<(), BencherError> {
        if let Some(sub) = &self.sub {
            sub.run(&self.wide).await
        } else {
            Ok(())
        }
    }
}
