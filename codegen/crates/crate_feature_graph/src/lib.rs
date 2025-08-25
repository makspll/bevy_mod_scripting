mod feature;
mod graph;

pub use feature::*;
pub use graph::*;

#[cfg(test)]
mod test {
    use std::{collections::HashMap, hash::RandomState, path::PathBuf};

    use indexmap::IndexSet;
    use petgraph::dot::Dot;

    use super::*;
    #[test]
    fn test_parse_metadata_into_workspace() {
        unsafe { std::env::set_var("RUST_LOG", "trace") };
        env_logger::init();

        let test_manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("..")
            .join("test_crates")
            .join("workspace")
            .join("root")
            .join("Cargo.toml");

        let meta = cargo_metadata::MetadataCommand::new()
            .manifest_path(test_manifest)
            .exec()
            .unwrap();

        let workspace = Workspace::from(&meta);

        // Just ensure that we have some crates parsed
        assert!(!workspace.workspace_crates.is_empty());
        assert!(!workspace.other_crates.is_empty());

        // now calculate the graph
        let mut graph = WorkspaceGraph::from(workspace);
        // enable the crate_a chain of features, expect that default feature
        graph.calculate_enabled_features_and_dependencies(
            HashMap::from_iter([(
                CrateName("root".into()),
                (vec![FeatureName::new("enable_crate_a")], true),
            )]),
            false,
        );

        let conditional_graph = format!(
            "{:?}",
            Dot::with_config(&graph.dependency_conditions_graph, &[])
        );
        let expected = r#"digraph {
    0 [ label = "CrateName(\"crate_a\")" ]
    1 [ label = "CrateName(\"crate_b\")" ]
    2 [ label = "CrateName(\"root\")" ]
    3 [ label = "CrateName(\"cfg-if\")" ]
    4 [ label = "CrateName(\"log\")" ]
    5 [ label = "CrateName(\"once_cell\")" ]
    0 -> 4 [ label = "Always" ]
    1 -> 5 [ label = "Always" ]
    2 -> 3 [ label = "Always" ]
    2 -> 0 [ label = "IfFeature(CrateName(\"root\"), FeatureName(\"enable_crate_a\"))" ]
    2 -> 1 [ label = "IfFeature(CrateName(\"root\"), FeatureName(\"enable_crate_b\"))" ]
}"#;
        pretty_assertions::assert_eq!(conditional_graph.trim(), expected.trim());

        // finally assert all the correct features were enabled
        // we are expecting the following structure:
        // root
        //   |- crate_a (workspace)
        //   |- crate_b (workspace)

        let root_crate = graph
            .workspace
            .find_crate(&CrateName("root".into()), || "Root crate not found".into())
            .unwrap();

        let crate_a = graph
            .workspace
            .find_crate(&CrateName("crate_a".into()), || "crate_a not found".into())
            .unwrap();

        let crate_b = graph
            .workspace
            .find_crate(&CrateName("crate_b".into()), || "crate_b not found".into())
            .unwrap();

        assert_eq!(root_crate.in_workspace, Some(true));
        assert_eq!(crate_a.in_workspace, Some(true));
        assert_eq!(crate_b.in_workspace, Some(true));

        // we also expect some dependency crates like cfg_if
        let cfg_if = graph
            .workspace
            .find_crate(&CrateName("cfg-if".into()), || "cfg-if not found".into())
            .unwrap();
        assert_eq!(cfg_if.in_workspace, Some(false));

        // assert features
        pretty_assertions::assert_eq!(
            root_crate.active_features.as_ref().unwrap(),
            &IndexSet::<_, RandomState>::from_iter([
                FeatureName::new("default"),
                FeatureName::new("enable_crate_a"),
            ])
        );
        assert_eq!(root_crate.is_enabled, Some(true));

        pretty_assertions::assert_eq!(
            crate_a.active_features.as_ref().unwrap(),
            &IndexSet::<_, RandomState>::from_iter([
                FeatureName::new("crate_a_default"),
                FeatureName::new("default"),
            ])
        );

        assert_eq!(crate_a.is_enabled, Some(true));

        pretty_assertions::assert_eq!(crate_b.active_features, None);
        assert_eq!(crate_b.is_enabled, None);
    }
}
