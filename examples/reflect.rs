#![allow(dead_code)]

use bevy_reflect::{reflect_trait, Reflect, TypeRegistry};

trait Object: Reflect {}

#[derive(Reflect)]
struct Tree;

#[derive(Reflect)]
#[reflect(Vehicle)]
struct Car;

#[derive(Reflect)]
struct Bicycle;

impl Object for Tree {}
impl Object for Car {}
impl Object for Bicycle {}

#[reflect_trait]
trait Vehicle: Object {}

impl Vehicle for Car {}
impl Vehicle for Bicycle {}

// struct ObjectStateEstimate {
//     label: Object,
// }
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let obj: Box<dyn Reflect> = Box::new(Car {});

    dbg!(obj.is::<Car>());
    dbg!(obj.is::<Tree>());

    // Normally in rust we would be out of luck at this point. Lets use our new reflection powers to
    // do something cool!
    let mut type_registry = TypeRegistry::default();
    type_registry.register::<Car>();
    type_registry.register_type_data::<Car, ReflectVehicle>();

    // The #[reflect] attribute we put on our DoThing trait generated a new `ReflectDoThing` struct,
    // which implements TypeData. This was added to MyType's TypeRegistration.
    let reflect_vehicle = type_registry
        .get_type_data::<ReflectVehicle>(obj.type_id())
        .unwrap();

    // We can use this generated type to convert our `&dyn Reflect` reference to a `&dyn DoThing`
    // reference

    // dbg!(obj.represents::<ReflectVehicle>());
    let _my_trait: &dyn Vehicle = reflect_vehicle.get(&*obj).unwrap();

    // dbg!(obj.represents::<ReflectVehicle>());

    Ok(())
}
