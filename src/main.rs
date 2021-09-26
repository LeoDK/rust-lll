mod linalg;

use crate::linalg::{vector::Vector, lattice::Lattice};

fn main ()
{
    let u = Vector::<f64>::new (vec![201.0, 37.0, 43.0]);
    let v = Vector::<f64>::new (vec![1648.0, 297.0, -29.0]);
    let w = Vector::<f64>::new (vec![42.0, 58.0, 23.0]);
    let mut lattice = Lattice::new (vec![u,v,w]);
    println! ("Before : {}", lattice);
    lattice.size_reduce ();
    println! ("After size reduction : {}", lattice);

    let u = Vector::<f64>::new (vec![201.0, 37.0, 43.0]);
    let v = Vector::<f64>::new (vec![1648.0, 297.0, -29.0]);
    let w = Vector::<f64>::new (vec![42.0, 58.0, 23.0]);
    let mut lattice = Lattice::new (vec![u,v,w]);
    lattice.lll (0.75);
    println! ("After lll : {}", lattice);
}
