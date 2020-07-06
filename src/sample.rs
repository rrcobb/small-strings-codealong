use argh::FromArgs;
use parse_display::{Display, FromStr};

use smartstring::{LazyCompact, SmartString};
struct SmartWrap(SmartString<LazyCompact>);

impl From<String> for SmartWrap {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

impl From<&str> for SmartWrap {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl<'de> serde::Deserialize<'de> for SmartWrap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use ::serde::de::{Error, Visitor};
        use std::fmt;

        struct SmartVisitor;

        impl<'a> Visitor<'a> for SmartVisitor {
            type Value = SmartWrap;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(v.into())
            }

            fn visit_borrowed_str<E: Error>(self, v: &'a str) -> Result<Self::Value, E> {
                Ok(v.into())
            }

            fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(v.into())
            }
        }

        deserializer.deserialize_str(SmartVisitor)
    }
}


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
            Lib::Smart => self.read_records::<SmartWrap>(),
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
