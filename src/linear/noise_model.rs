use nalgebra::base::allocator::Allocator;
use nalgebra::base::default_allocator::DefaultAllocator;
use nalgebra::base::dimension::Dim;
use nalgebra::base::storage::Storage;
use nalgebra::base::{DMatrix, DVector, MatrixN, VectorN};
use nalgebra::RealField;
use std::fmt::Debug;

#[allow(non_snake_case)]
pub trait NoiseModel<D: Dim, T: RealField = f64>: Debug {
    fn is_constrained(&self) -> bool;

    fn is_unit(&self) -> bool;

    fn dim(&self) -> usize;

    fn sigmas(&self) -> DVector<T>;

    fn whiten(&self, v: &VectorN<T, D>) -> VectorN<T, D>
    where
        DefaultAllocator: Allocator<T, D>;

    fn whiten_mat(&self, m: &MatrixN<T, D>) -> MatrixN<T, D>
    where
        DefaultAllocator: Allocator<T, D, D>;

    fn unwhiten(&self, v: &VectorN<T, D>) -> VectorN<T, D>
    where
        DefaultAllocator: Allocator<T, D>;

    fn distance(&self, v: &VectorN<T, D>) -> T
    where
        DefaultAllocator: Allocator<T, D>;

    fn whiten_system<_D: Dim>(&self, A: &[DMatrix<T>], b: &VectorN<T, _D>)
    where
        DefaultAllocator: Allocator<T, _D>;
}

#[allow(non_snake_case)]
pub trait GaussianNoise<D: Dim, T: RealField = f64>: NoiseModel<D, T> {
    fn from_sqrtinfo(R: &MatrixN<T, D>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>;

    fn from_information(info: &MatrixN<T, D>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>;

    fn from_covariance(cov: &MatrixN<T, D>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>;

    fn sqrt_info(&self) -> Option<&MatrixN<T, D>>
    where
        DefaultAllocator: Allocator<T, D, D>;

    /// Mahalanobis distance v'*R'*R*v = <R*v,R*v>
    fn mahalanobis_dist(&self, v: &DVector<T>) -> T;
}

fn check_diagonal_upper<D: Dim, T: nalgebra::RealField>(mat: &MatrixN<T, D>) -> Option<DVector<T>>
where
    DefaultAllocator: Allocator<T, D, D>,
{
    let (m, n) = (mat.nrows(), mat.ncols());
    let mut full = false;
    for i in 0..m {
        if !full {
            for j in i + 1..n {
                if mat[(i, j)].abs() > T::default_epsilon() {
                    full = true;
                    break;
                }
            }
        }
    }

    if full {
        None
    } else {
        let mut diag = DVector::identity(m);
        for i in 0..m {
            diag[i] = mat[(i, i)]
        }
        Some(diag)
    }
}

#[derive(Debug)]
pub struct Gaussian<D: Dim, T: RealField = f64>
where
    DefaultAllocator: Allocator<T, D, D>,
{
    dim: usize,
    sqrt_info: Option<MatrixN<T, D>>,
}

impl<D: Dim, T: RealField> GaussianNoise<D, T> for Gaussian<D, T>
where
    DefaultAllocator: Allocator<T, D, D>,
{
    fn from_sqrtinfo(R: &MatrixN<T, D>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        let (m, n) = (R.nrows(), R.ncols());
        assert_eq!(m, n, "Non-square Matrix");
        if smart {
            if let Some(diagonal) = check_diagonal_upper(&R) {
                unimplemented!();
            }
        }

        Gaussian {
            dim: R.nrows(),
            sqrt_info: Some(R.to_owned()),
        }
    }

    fn from_information(info: &MatrixN<T, D>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        unimplemented!()
    }

    fn from_covariance(cov: &MatrixN<T, D>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        let (m, n) = (cov.nrows(), cov.ncols());
        assert_eq!(m, n, "Non-square Matrix");
        if smart {
            if let Some(diagonal) = check_diagonal_upper(cov) {
                unimplemented!();
            }
        }

        // NOTE: if cov = L'*L, then the square root information R can be found by
        // QR, as L.inverse() = Q*R, with Q some rotation matrix. However, R has
        // annoying sign flips with respect the simpler Information(inv(cov)),
        // hence we choose the simpler path here:
        let inv = cov.clone().try_inverse();
        Gaussian::from_information(&inv.unwrap(), false)
    }

    fn sqrt_info(&self) -> Option<&MatrixN<T, D>>
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        if let Some(s) = &self.sqrt_info {
            return Some(s);
        }
        None
    }

    /**
     * Mahalanobis distance v'*R'*R*v = <R*v,R*v>
     */
    fn mahalanobis_dist(&self, v: &DVector<T>) -> T {
        unimplemented!()
    }
}

impl<D: Dim, T: RealField> NoiseModel<D, T> for Gaussian<D, T>
where
    DefaultAllocator: Allocator<T, D, D>,
{
    fn is_constrained(&self) -> bool {
        unimplemented!()
    }

    fn is_unit(&self) -> bool {
        unimplemented!()
    }

    fn dim(&self) -> usize {
        self.dim
    }

    fn sigmas(&self) -> DVector<T> {
        unimplemented!()
    }

    fn whiten(&self, v: &VectorN<T, D>) -> VectorN<T, D>
    where
        DefaultAllocator: Allocator<T, D, D> + Allocator<T, D>,
    {
        unimplemented!()
    }

    fn whiten_mat(&self, m: &MatrixN<T, D>) -> MatrixN<T, D>
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        unimplemented!()
    }

    fn unwhiten(&self, v: &VectorN<T, D>) -> VectorN<T, D>
    where
        DefaultAllocator: Allocator<T, D, D> + Allocator<T, D>,
    {
        unimplemented!()
    }

    fn distance(&self, v: &VectorN<T, D>) -> T
    where
        DefaultAllocator: Allocator<T, D, D> + Allocator<T, D>,
    {
        unimplemented!()
    }

    fn whiten_system<_D: Dim>(&self, A: &[DMatrix<T>], b: &VectorN<T, _D>)
    where
        DefaultAllocator: Allocator<T, D, D> + Allocator<T, _D>,
    {
        unimplemented!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::base::dimension::Dynamic;
    use nalgebra::base::{Matrix4, Vector4, U4};

    #[test]
    fn gaussian_model_construction() {
        let si = DMatrix::<f64>::identity(4, 4);
        let g = Gaussian::from_sqrtinfo(&si, false);

        let se = Matrix4::<f64>::identity();
        let ge = Gaussian::from_sqrtinfo(&se, false);

        println!("{:#?}", ge.sqrt_info());
    }
}
