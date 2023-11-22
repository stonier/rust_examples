// ***************************************************************************
// About
// ***************************************************************************

//! Isometry3 - How performant is it?
//
// ***************************************************************************
// Dependencies
// ***************************************************************************

type Point3 = nalgebra::geometry::Point3<f64>;
type Translation3 = nalgebra::geometry::Translation3<f64>;
type Isometry3 = nalgebra::geometry::Isometry3<f64>;
type IsometryMatrix3 = nalgebra::geometry::IsometryMatrix3<f64>;
type Rotation3 = nalgebra::geometry::Rotation3<f64>;
type Quaternion = nalgebra::geometry::UnitQuaternion<f64>;
type Vector3 = nalgebra::base::Vector3<f64>;
type Transform3 = nalgebra::geometry::Transform<f64, nalgebra::TAffine, 3>;

// ***************************************************************************
// Main
// ***************************************************************************

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let axisangle1 = Vector3::y() * std::f64::consts::FRAC_PI_2;
    let q1 = Quaternion::new(axisangle1);
    let r1 = Rotation3::new(axisangle1);
    let t1 = Translation3::new(1.0, 0.0, 0.0);
    let iso1 = Isometry3::from_parts(t1, q1);

    let axisangle2 = Vector3::y() * std::f64::consts::FRAC_PI_2;
    let q2 = Quaternion::new(axisangle2);
    let r2 = Rotation3::new(axisangle2);
    let t2 = Translation3::new(1.0, 2.0, 3.0);
    let iso2 = Isometry3::from_parts(t2, q2);

    let isom1 = IsometryMatrix3::from_parts(t1, r1);
    let isom2 = IsometryMatrix3::from_parts(t2, r2);

    // Ugh, returns you a Matrix, not a Transform
    let _this_is_4x4_matrix_not_transform = iso1.to_homogeneous();

    // Is there a sane construction method to get it checked?
    let trans1 = Transform3::from_matrix_unchecked(iso1.to_homogeneous());
    let trans2 = Transform3::from_matrix_unchecked(iso2.to_homogeneous());

    let p = Point3::new(1.0, 0.0, 0.0);

    // Usability
    //  - Check for Isometry! Good apis, good operators - no need to convert to homogenous transforms
    //  - Transforms are more awkward ... less operators exist and need 'try' on some apis

    println!("Usability - Transform Point");
    println!(" - iso1*iso2: {:?}", iso1 * iso2);
    println!(" - trans1*trans2: {:?}", trans1 * trans2);

    println!("Usability - Transform Point");
    println!(" - iso1*p {:?}", iso1 * p);
    println!(
        " - trans1.transform_point(&p): {:?}",
        trans1.transform_point(&p)
    );

    println!("Usability - Inverse");
    let identity = iso1 * iso1.inverse();
    assert_eq!(Isometry3::identity(), identity);
    println!(" - iso1*iso1.inverse() {:?}", identity);
    // can't guarantee an inverse with Transform, so...
    if let Some(inverse) = trans1.try_inverse() {
        println!(" - trans1*trans1.inverse() {:?}", trans1 * inverse);
    }

    // Performance - Transform wins here
    // TODO(stonier) - consider using a better benchmarker here, e.g. criterion
    {
        let start = std::time::SystemTime::now();
        for i in 1..=10000000 {
            let p = Point3::new(i as f64, 0.0, 0.0);
            let transform = std::hint::black_box(trans1 * trans2);
            if let Some(inverse) = transform.try_inverse() {
                let _ = std::hint::black_box(transform * inverse);
            }
            let _ = std::hint::black_box(transform.transform_point(&p));
        }
        let end = std::time::SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        println!("Transform took {} seconds", duration.as_secs_f64());
    }
    {
        let start = std::time::SystemTime::now();
        for i in 1..=10000000 {
            let p = Point3::new(i as f64, 0.0, 0.0);
            let iso = std::hint::black_box(iso1 * iso2);
            let inverse = iso.inverse();
            let _ = std::hint::black_box(iso * inverse);
            let _ = std::hint::black_box(iso * p);
        }
        let end = std::time::SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        println!("Isometry took {} seconds", duration.as_secs_f64());
    }
    {
        let start = std::time::SystemTime::now();
        for i in 1..=10000000 {
            let p = Point3::new(i as f64, 0.0, 0.0);
            let isom = std::hint::black_box(isom1 * isom2);
            let inverse = isom.inverse();
            let _ = std::hint::black_box(isom * inverse);
            let _ = std::hint::black_box(isom * p);
        }
        let end = std::time::SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        println!("IsometryMatrix took {} seconds", duration.as_secs_f64());
    }
}
