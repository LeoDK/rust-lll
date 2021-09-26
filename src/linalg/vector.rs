use std::ops::*;
use std::clone::Clone;
use std::fmt::{Display, Result, Formatter};

pub struct Vector<T>
{
    coord: Vec<T>
}

impl<T> Vector<T>
{
    pub fn new (coord: Vec<T>) -> Self
    {
        Self { coord }
    }

    pub fn size (&self) -> usize
    {
        self.coord.len()
    }
}

impl Vector<isize>
{
    pub fn zero (dim: usize) -> Self
    {
        Vector { coord: vec![0; dim] }
    }
}

impl Vector<f64>
{
    pub fn zero (dim: usize) -> Self
    {
        Vector { coord: vec![0.0; dim] }
    }
}

impl<T: Display> Display for Vector<T>
{
    fn fmt (&self, f: &mut Formatter) -> Result
    {
        let mut ret = String::from("[ ");
        for val in self.coord.iter()
        {
            let s = format!("{} ", val);
            ret.push_str (&s);
        }
        ret.push (']');
        write! (f, "{}", ret)
    }
}

impl<T: Clone> Clone for Vector<T>
{
    fn clone (&self) -> Self
    {
        let mut coord = Vec::<T>::new ();
        for elem in &self.coord[..]
        {
            coord.push (elem.clone ());
        }
        Vector::<T> { coord }
    }
}

impl<T> Add for &Vector<T>
where
    T: Add<Output=T> + Copy
{
    type Output = Vector<T>;

    fn add (self, other: Self) -> Self::Output
    {
        if self.size () != other.size ()
        {
            panic! ("Different size in vector sum");
        }
        let mut ret = Vector::<T>::new (vec![]);
        for i in 0..self.size ()
        {
            ret.coord.push (self.coord[i] + other.coord[i]);
        }
        ret
    }
}

impl<T> AddAssign<&Self> for Vector<T>
where
    T: AddAssign + Copy
{
    fn add_assign (&mut self, other: &Self)
    {
        if self.size () != other.size ()
        {
            panic! ("Different size in vector sum");
        }
        for i in 0..self.size ()
        {
            self.coord[i] += other.coord[i];
        }
    }
}

impl<T> Sub for &Vector<T>
where
    T: Sub<Output=T> + Copy
{
    type Output = Vector<T>;

    fn sub (self, other: Self) -> Self::Output
    {
        if self.size () != other.size ()
        {
            panic! ("Substracting vectors with different sizes");
        }
        let mut ret = Vector::<T>::new (vec![]);
        for i in 0..self.size ()
        {
            ret.coord.push (self.coord[i] - other.coord[i]);
        }
        ret
    }
}

impl<T> SubAssign<&Self> for Vector<T>
where
    T: SubAssign + Copy
{
    fn sub_assign (&mut self, other: &Self)
    {
        for i in 0..self.size ()
        {
            self.coord[i] -= other.coord[i];
        }
    }
}

impl<T> BitAnd for &Vector<T>
where
    T: AddAssign + Mul<Output=T> + Copy
{
    type Output = T;

    fn bitand (self, other: Self) -> Self::Output
    {
        if self.size () != other.size ()
        {
            panic! ("Different size in vector dot product");
        }
        let mut ret = self.coord[0] * other.coord[0];
        for i in 1..self.size ()
        {
            ret += self.coord[i] * other.coord[i];
        }
        ret
    }
}

impl<T> Mul<T> for &Vector<T>
where
    T: Mul<Output=T> + Copy
{
    type Output = Vector<T>;

    fn mul (self, other: T) -> Vector<T>
    {
        let mut ret = Vector::<T>::new (vec![]);
        for i in 0..self.size ()
        {
            ret.coord.push (other * self.coord[i]);
        }
        ret
    }
}

impl<T> MulAssign<T> for Vector<T>
where
    T: MulAssign + Copy
{
    fn mul_assign (&mut self, other: T)
    {
        for i in 0..self.size ()
        {
            self.coord[i] *= other;
        }
    }
}
