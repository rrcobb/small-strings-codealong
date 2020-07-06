use argh::FromArgs;

#[derive(FromArgs)]
/// Run sample code
#[argh(subcommand, name = "sample")]
pub struct Sample {}

impl Sample {
    pub fn run(self) {
        self.read_records();
    }

    fn read_records(&self) {
       use serde::Deserialize; 

       #[derive(Deserialize)]
       struct Record<'a> {
           #[allow(unused)]
           city: &'a str,
           #[allow(unused)]
           state: &'a str,
       }

       crate::ALLOCATOR.set_active(true);
       let input = std::fs::read_to_string("cities.json").unwrap();
       let records: Vec<Record> = serde_json::from_str(&input).unwrap();
       crate::ALLOCATOR.set_active(false);
       println!("Read {} records", records.len());
    }
}
