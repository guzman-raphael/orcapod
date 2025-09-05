# Pods

A **Pod** is the basic unit of computation in Orcapod.  
Each Pod wraps a containerized command with declared **inputs** and **outputs**, so that pipelines remain reproducible and easy to reason about.

## Core ideas

- **Reusable unit**: a Pod can be used on its own or as part of a pipeline.  
- **Containerized execution**: Pods run inside Docker images to ensure consistent environments.  
- **Declared interface**: each Pod specifies named inputs and outputs (e.g. `left`, `right`, `answer`).  
- **Composable**: outputs from one Pod can be connected to inputs of another via operators like `map` and `join`.

## Example

*Example coming soon...*
