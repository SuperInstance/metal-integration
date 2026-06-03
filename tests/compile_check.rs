//! Integration test — verifies all Rust metal crates compile and link together.
//! Uses a public constant/type from each crate to confirm linkage.

#[test]
fn all_crates_link() {
    let _ = conservation_spectral_topology::compute_spectral_budget;
    let _ = sheaf_agents::CellularSheaf::new;
    let _ = hodge_belief::hodge_decompose_1;
    let _ = spectral_graph_agent::SpectralGraph::new;
    let _ = ergodic_transport::MAX_STATES;
    let _ = evolving_sheaf::sheaf::SheafConfig::static_sheaf(0.5);
    assert!(true, "All 6 crates compiled and linked successfully");
}
