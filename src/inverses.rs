use anyhow::Result;
use qip::inverter::Invertable;
use qip::prelude::*;
use qip_macros::{invert, program};
use std::num::NonZeroUsize;

// Make gamma and its inverse: gamma_inv
#[invert(gamma_inv)]
fn gamma<B>(
    b: &mut B,
    ra: B::Register,
    rb: B::Register,
) -> CircuitResult<(B::Register, B::Register)>
where
    B: AdvancedCircuitBuilder<f64> + Invertable<SimilarBuilder = B>,
{
    let (ra, rb) = b.toffoli(ra, rb)?;
    let (rb, ra) = b.toffoli(rb, ra)?;
    Ok((ra, rb))
}

fn main() -> Result<(), CircuitError> {
    let n = NonZeroUsize::new(3).unwrap();
    let mut b = LocalBuilder::default();
    let ra = b.register(n);
    let rb = b.register(n);

    let (ra, rb) = program!(&mut b; ra, rb;
        gamma ra[0..2], ra[2];
        gamma_inv ra[0..2], ra[2];
    )?;

    Ok(())
}
