use crate::inference::factor::Factor;
use crate::linear::gaussian_like::GaussianLikeFactor;
use nalgebra as na;

pub struct GaussianFactor {}

impl Factor for GaussianFactor {
    fn keys(&mut self) -> &mut Vec<u64> {
        unimplemented!()
    }
}

impl GaussianLikeFactor for GaussianFactor {
    fn augmented_jacobian(&self) -> na::DMatrix<f64> {
        unimplemented!()
    }

    fn jacobian(
        &self,
    ) -> (
        na::MatrixMN<f64, na::Dynamic, na::Dynamic>,
        na::VectorN<f64, na::Dynamic>,
    ) {
        unimplemented!()
    }

    fn augmented_information(&self) -> na::MatrixMN<f64, na::Dynamic, na::Dynamic> {
        unimplemented!()
    }

    fn information(&self) -> na::MatrixMN<f64, na::Dynamic, na::Dynamic> {
        unimplemented!()
    }

    fn hessian_diagonal(&self) -> Vec<(u64, na::VectorN<f64, na::Dynamic>)> {
        unimplemented!()
    }

    fn hessian_block_diagonal(&self) -> Vec<(u64, na::MatrixN<f64, na::Dynamic>)> {
        unimplemented!()
    }
}
