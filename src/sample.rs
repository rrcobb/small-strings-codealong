use argh::FromArgs;
use parse_display::{Display, FromStr};

#[derive(FromArgs)]
/// Run sample code
#[argh(subcommand, name = "sample")]
pub struct Sample {
    #[argh(option)]
    /// which library to use
    lib: Lib,
}

#[derive(Display, FromStr)]
#[display(style = "snake_case")]
enum Lib {
    Std,
    Smol,
    Smart,
}

impl Sample {
    pub fn run(self) {
        match self.lib {
            Lib::Std => self.read_records::<String>(),
            Lib::Smol => self.read_records::<smol_str::SmolStr>(),
            Lib::Smart => todo!(),
        }
    }

    fn read_records<S>(&self)
        where
            S: serde::de::DeserializeOwned,
    {
       use serde::Deserialize; 

       #[derive(Deserialize)]
       struct Record<S> {
           #[allow(unused)]
           city: S,
           #[allow(unused)]
           state: S,
       }

       use std::fs::File;
       let f = File::open("cities.json").unwrap();
       crate::ALLOCATOR.set_active(true);
       let records: Vec<Record<S>> = serde_json::from_reader(f).unwrap();
       crate::ALLOCATOR.set_active(false);
       println!("Read {} records", records.len());
    }
}
