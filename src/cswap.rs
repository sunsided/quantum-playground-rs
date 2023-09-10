use anyhow::Result;
use qip::builder::{MeasurementHandle, Qudit};
use qip::prelude::*;
use std::num::NonZeroUsize;

fn main() -> Result<()> {
    // Make a new circuit builder.
    let (b, ra, rb, m_handle) = define_circuit()?;

    // Now q is the end result of the above circuit, and we can run the circuit by referencing it.
    run_and_measure(b, ra, rb, m_handle);
    Ok(())
}

fn define_circuit() -> Result<(LocalBuilder<f64>, Qudit, Qudit, MeasurementHandle), CircuitError> {
    let mut builder = LocalBuilder::<f64>::default();
    let n = NonZeroUsize::new(3).unwrap();

    // Make three registers of sizes 1, 3, 3 (7 qubits total).
    let q = builder.qubit(); // Same as b.register(1)?;
    let ra = builder.register(n);
    let rb = builder.register(n);

    // Define circuit
    // First apply an H to q
    let q = builder.h(q);

    // Then swap ra and rb, conditioned on q.
    let mut cb = builder.condition_with(q);
    let (ra, rb) = cb.swap(ra, rb)?;
    let q = cb.dissolve();

    // Finally apply H to q again.
    let q = builder.h(q);

    // Add a measurement to the first qubit, save a reference so we can get the result later.
    let (_q, m_handle) = builder.measure(q);
    Ok((builder, ra, rb, m_handle))
}

/// Runs the circuit with a given precision.
fn run_and_measure(mut b: LocalBuilder<f64>, ra: Qudit, rb: Qudit, m_handle: MeasurementHandle) {
    let (_, measured) = b.calculate_state_with_init([(&ra, 0b000), (&rb, 0b001)]);

    // Lookup the result of the measurement we performed using the handle, and the probability
    // of getting that measurement.
    let (result, p) = measured.get_measurement(m_handle);

    // Print the measured result
    println!("Measured: {:?} (with chance {:?})", result, p);
}
