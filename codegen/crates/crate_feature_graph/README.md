# Crate Feature Graph

![Example output graph](./example.png)

A tool for visualising feature flow in a rust workspace, from the perspective of build time features.

## Features
- Compute feature flow throughout your workspace
- Filter out crates you don't want to see from the graph, but retain the connections
- Output as a dot graph or if the `dot_parser` feature is disabled as a text representation