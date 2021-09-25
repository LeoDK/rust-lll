mod linalg;

use crate::linalg::vector::Vector;

fn main ()
{
    //let u = Vector::<f64>::zero (3);
    let mut u = Vector::new (vec![-3.4, 2.1, 0.2]);
    let v = Vector::new (vec![1.0, 2.0, 3.0]);
    u += &v;
    println! ("{}", u);
    println! ("{}", v);
    let w = &u + &v;
    let scal = &u & &v;
    println! ("  {}", u);
    println! ("+ {}", v);
    println! ("= {}", w);
    println! ("u.v = {}", scal);
}
