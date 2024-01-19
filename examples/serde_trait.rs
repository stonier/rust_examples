use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[typetag::serde(tag = "name")]
trait Marker: Debug {
    fn foo(&self) { println!("Foo"); }
}

#[derive(Serialize, Deserialize, Debug)]
struct Vehicle;
#[derive(Serialize, Deserialize, Debug)]
struct Bicycle;

#[typetag::serde]
impl Marker for Vehicle {}

#[typetag::serde]
impl Marker for Bicycle {}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    labels: Vec<Box<dyn Marker>>,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = r#"
        labels:
            - name: Vehicle
            - name: Bicycle
    "#;

    println!("{}", file);

    let config: Config = serde_yaml::from_str(&file)?;

    println!("{:?}", config);
    
    for m in config.labels {
        println!("{:?}", m);
        print_type_of(&m);
        m.foo();
    }

    Ok(())
}
