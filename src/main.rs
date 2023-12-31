use std::{error::Error, io, process};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Person {
    id: String,
    age: String,
    name: String,
}

/// Read the csv and transform in some data structure.
fn read() -> Result<Vec<Person>, Box<dyn Error>> {
    let mut persons: Vec<Person> = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for r in rdr.deserialize() {
        let p: Person = r?;
        persons.push(p);
    }

    Ok(persons)
}

fn main() {
    let persons = read().unwrap_or_else(|err| {
        println!("error running example: {err}");
        process::exit(1);
    });

    println!("{:#?}", persons);
}
