use clap::Parser;
use crate_feature_graph::{CrateName, Workspace, WorkspaceGraph};

#[derive(Parser)]
struct Args {
    /// Path to the Cargo.toml of the workspace to analyze
    #[clap(long, short, default_value = "./Cargo.toml")]
    manifest_path: String,

    /// Comma-separated list of feature names to enable for the given manifest
    #[clap(
        short,
        long,
        default_value = "",
        use_value_delimiter = true,
        value_delimiter = ','
    )]
    features: Vec<String>,

    /// Whether to disable the default features of the given manifest
    /// Defaults to true
    #[clap(long, default_value = "false")]
    disable_default_features: bool,

    /// Only show crates in the output graph that match one of these names
    /// Only works for dot graphs
    /// Defaults to empty, which shows all crates
    #[clap(long, use_value_delimiter = true, value_delimiter = ',')]
    only_show_crates: Vec<String>,

    /// The package to consider as the root of the search
    /// The default will use the root of the workspace
    /// The features provided will be applied to this package, unless a crate prefix is provided
    #[clap(long)]
    root_package: Option<String>,
}

pub fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let args = Args::parse();

    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(&args.manifest_path)
        .exec()
        .expect("Failed to get cargo metadata");

    let workspace = Workspace::from(&metadata);
    let mut graph = WorkspaceGraph::from(workspace);

    let root = args.root_package;
    log::info!(
        "Calculating features for root: {:?} with features: {:?}, default features: {}",
        root,
        args.features,
        !args.disable_default_features
    );

    // TODO: allow focusing on a non-workspace root, i.e. package
    graph
        .calculate_enabled_features_and_dependencies_parse(args.features, root.map(CrateName::new));

    #[cfg(feature = "dot_parser")]
    let dot_graph = graph.visualise_feature_flow(args.only_show_crates)?;

    #[cfg(not(feature = "dot_parser"))]
    let dot_graph = {
        log::warn!("Feature `dot_parser` not enabled, dumping debug print instead");
        format!("{:?}", graph.workspace)
    };

    // dump the graph
    println!("{dot_graph}");

    Ok(())
}
