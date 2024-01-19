
// ***************************************************************************
// Objects
// ***************************************************************************

#[derive(Debug)]
enum ObjectType {
    Car,
    Truck,
    Human,
}

// ***************************************************************************
// Vehicle Collector
// ***************************************************************************

pub struct Vehicle {
    object_type: ObjectType
}

inventory::collect!(Vehicle);

// ***************************************************************************
// Vehicle Inventory
// ***************************************************************************

inventory::submit! {
    Vehicle {
        object_type: ObjectType::Car
    }
}

inventory::submit! {
    Vehicle {
        object_type: ObjectType::Truck
    }
}

fn main() {
    for v in inventory::iter::<Vehicle> {
        println!("object type: {:?}", v.object_type);
    }
}
