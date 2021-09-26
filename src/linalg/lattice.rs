use std::fmt::{Display, Result, Formatter};
use std::cmp;
use super::vector::Vector;

// Euclidian lattice
pub struct Lattice
{
    basis: Vec<Vector<f64>>, // B
    orth_basis: Vec<Vector<f64>>, // B*
    mu: Vec<Vec<f64>> // mu[i,j] = <b_i, b*_j> / || b*_j ||^2
}

impl Display for Lattice
{
    fn fmt (&self, f: &mut Formatter) -> Result
    {
        let mut ret = String::from ("B :");
        for vector in self.basis.iter ()
        {
            let s = format!("\n{}", vector);
            ret.push_str (&s);
        }
        write! (f, "{}", ret)
    }
}

impl Lattice
{
    // Creates lattice from basis
    pub fn new (basis: Vec<Vector<f64>>) -> Lattice
    {
        let orth_basis = basis.clone ();
        let mu = Lattice::empty_mu (basis.len ());
        let mut ret = Lattice { basis, orth_basis, mu };
        ret.gram_schmidt ();
        ret
    }

    // Initialize mu to zeros (mu_i,j is defined for i>=j)
    fn empty_mu (dim: usize) -> Vec<Vec<f64>>
    {
        let mut ret = Vec::<Vec<f64>>::new ();
        for i in 0..dim
        {
            ret.push (vec![0.0; i+1]);
        }
        ret
    }

    pub fn dim (&self) -> usize
    {
        self.basis.len ()
    }

    fn compute_mu (&mut self, i: usize, j: usize)
    {
        self.mu[i][j] = &self.basis[i] & &self.orth_basis[j];
        self.mu[i][j] /= &self.orth_basis[j] & &self.orth_basis[j];
    }

    // Compute the whole B* when given B
    pub fn gram_schmidt (&mut self)
    {
        for i in 0..self.dim ()
        {
            for j in 0..i
            {
                self.compute_mu (i,j);
                let projection = &self.orth_basis[j] * self.mu[i][j];
                self.orth_basis[i] -= &projection;
            }
        }
        for i in 0..self.dim ()
        {
            self.compute_mu (i, i)
        }
    }

    // Compute only b*_i and mu_i,j for i>=start
    // Used to avoid repetition during update
    fn gram_schmidt_from (&mut self, start: usize)
    {
        for i in start..self.dim ()
        {
            self.orth_basis[i] = self.basis[i].clone ();
            for j in 0..i
            {
                self.compute_mu (i,j);
                let projection = &self.orth_basis[j] * self.mu[i][j];
                self.orth_basis[i] -= &projection;
            }
        }
        for i in start..self.dim ()
        {
            self.compute_mu (i, i)
        }
    }

    pub fn size_reduce (&mut self)
    {
        for i in 0..self.dim ()
        {
            for j in 0..i
            {
                let approx = &self.basis[j] * self.mu[i][j].round ();
                self.basis[i] -= &approx;
            }
        }
    }

    fn check_lovasz (&self, i: usize, delta: f64) -> bool
    {
        let p1 = &self.orth_basis[i+1] & &self.orth_basis[i+1];
        let p2 = (delta - self.mu[i+1][i].powi(2)) * (&self.orth_basis[i] & &self.orth_basis[i]);
        p1 >= p2
    }

    pub fn lll (&mut self, delta: f64)
    {
        if (delta > 1.0) || (delta <= 0.25)
        {
            panic! ("Delta provided for LLL not in bounds !");
        }

        let mut k = 1;
        while k < self.dim ()
        {
            // Size reduction only for the current vector
            for i in 0..k
            {
                let approx = &self.basis[i] * self.mu[k][i].round ();
                self.basis[k] -= &approx;
                // Update mu
                self.mu[k][i] = self.mu[k][i] - self.mu[k][i].round() * self.mu[i][i];
            }
            if self.check_lovasz (k-1, delta)
            {
                k += 1;
            }
            else {
                // Swap b_k and b_{k-1}
                let bk = self.basis.remove (k);
                let bk_1 = self.basis.remove (k-1);
                self.basis.insert (k-1, bk);
                self.basis.insert (k, bk_1);
                // Update mu and B*
                self.gram_schmidt_from (k-1);
                k = cmp::max (k-1, 1);
            }
        }
    }
}
