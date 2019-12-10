/// module, struct, interior mutability
mod mystructs;
// if no "mod mystructs;" here.
// failed to resolve: use of undeclared type or module `mystructs`
// use of undeclared type or module `mystructs`

// use mystructs::tuple_like::PrivateBounds;
// struct `PrivateBounds` is private

// use mystructs::tuple_like::{Bounds, usage_private_bounds};
// use mystructs::named_field::Point;
// use mystructs::unit_like::{*};
use mystructs::{
    named_field::Point,
    tuple_like::{usage_private_bounds, Bounds},
    unit_like::*,
};

fn main() {
    let such = Onesuch;
    println!("Got Onesuch: {:?}", such);

    let image_bounds = Bounds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);

    println!("usage_private_bounds: {}", usage_private_bounds(1, 2));

    // let point = Point{x:1024, y:768, z:100};
    // field `z` of struct `mystructs::named_field::Point` is private

    let point = Point::new(1024, 768);
    println!("{}-{}", &point.x, &point.y);
    println!("{}", &point);

    // cannot assign to `point.x`, as `point` is not declared as mutable
    // point.x = 2048;

    let mut mut_point = Point::new(100, 200);
    println!("{}", &mut_point);
    mut_point.x = 200;
    mut_point.y = 400;
    println!("{}", &mut_point);
    mut_point.pluse_z();
    println!("{}", &mut_point);

    // https://ricardomartins.cc/2016/06/08/interior-mutability
    let inmut_point = Point::new(100, 200);
    println!("{}", &inmut_point);
    inmut_point.pluse_n();
    println!("{}", &inmut_point);
}
