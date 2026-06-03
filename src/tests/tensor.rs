#[test]
fn all_deps_link() {
    // Verify all 6 direct deps compile and link
    use conservation_spectral_topology::*;
    use sheaf_agents::*;
    use hodge_belief::*;
    use spectral_graph_agent::*;
    use ergodic_transport::*;
    use evolving_sheaf::*;
    assert!(true, "all 6 crates compiled and linked");
}
