mod linalg;

use crate::linalg::{vector::Vector, lattice::Lattice};

fn main ()
{
    let u = Vector::<f64>::new (vec![201.0, 37.0]);
    let v = Vector::<f64>::new (vec![1648.0, 297.0]);
    let mut lattice = Lattice::new (vec![u,v]);
    println! ("{}", lattice);
    lattice.lll (0.75);
    println! ("{}", lattice);
}
