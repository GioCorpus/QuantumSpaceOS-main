#![cfg(test)]
use quantum_core::prelude::*;

#[test]
fn qubit_zero_normalized() {
    let q = Qubit::zero();
    let norm = q.state.alpha.norm_sqr() + q.state.beta.norm_sqr();
    assert!((norm - 1.0).abs() < 1e-10);
}

#[test]
fn hadamard_creates_superposition() {
    let mut q = Qubit::zero();
    apply_hadamard(&mut q);
    let p0 = q.state.prob_zero();
    let p1 = q.state.prob_one();
    assert!((p0 - 0.5).abs() < 1e-6);
    assert!((p1 - 0.5).abs() < 1e-6);
}

#[test]
fn x_gate_flips() {
    let mut q = Qubit::zero();
    let gate = quantum_core::gates::x();
    gate.apply(&mut q);
    assert_eq!(q.measure(), 1);
}

#[test]
fn cnot_entangles() {
    let mut c = Qubit::zero();
    let mut t = Qubit::zero();
    apply_hadamard(&mut c); // |+0⟩
    quantum_core::gates::cnot().apply_cnot(&mut c, &mut t);
    // Target should now be correlated with control
    let p_t1 = t.measure_weak();
    assert!((p_t1 - 0.5).abs() < 1e-6);
}

#[test]
#[should_panic(expected = "InvalidState")]
fn non_normalized_state_rejected() {
    QubitState::new(Complex::new(2.0, 0.0), Complex::new(0.0, 0.0)).unwrap();
}

#[test]
fn grover_search_simple() {
    let oracle = |bits: &[u8]| -> bool { bits[0] == 1 };
    let grover = GroverSearch::new(oracle, 2);
    let result = grover.run();
    assert!(!result.is_empty());
}
