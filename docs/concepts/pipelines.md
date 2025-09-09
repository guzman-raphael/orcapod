# Pipelines

A **Pipeline** in Orcapod is a directed graph of Pods.
Pipelines define how outputs from one Pod become inputs to another, and they can branch or merge through operators like **map** and **join**.

## Key points

- Pipelines are **graphs**, not just linear chains. They can fan out and fan in.
  - **<a href="/concepts/operators">map</a>** retags outputs so they align with the expected inputs of the next Pod.
  - **<a href="/concepts/operators">join</a>** combines multiple inputs, building Cartesian products for parallel execution.
- The **orchestrator** schedules Pods in the pipeline and assigns them to Agents.
- **<a href="/concepts/agents">Agents</a>** run Pods in parallel and send back status signals such as progress, completion, and failure.
- If one Pod fails, only its dependent steps are cancelled, while other branches of the pipeline continue (fault-permissible execution).
- Pipelines persist results through the store, making runs auditable and reproducible.

## Why Pipelines?

Pipelines allow you to connect Pods into complete scientific workflows.
They make data flow explicit, enable parallelism, and ensure results are consistent and traceable across environments.
