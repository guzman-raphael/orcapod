# Pods

A **Pod** is the basic unit of computation in Orcapod.  
Each Pod wraps a containerized command with declared **inputs** and **outputs**, so that pipelines remain reproducible and easy to reason about.

## Core ideas

- **Reusable unit**: a Pod can be used on its own or as part of a pipeline.
- **Containerized execution**: Pods run inside Docker images to ensure consistent environments.
- **Declared interface**: each Pod specifies named inputs and outputs (e.g. `left`, `right`, `answer`).
- **Composable**: outputs from one Pod can be connected to inputs of another via operators like `map` and `join`.

## Example

```python
# Example: defining a simple "add" Pod

from orcapod import Pod
from textwrap import dedent

add_pod = Pod(
    image="python:alpine",
    command=[
        "python",
        "-c",
        dedent(
            """
            from pathlib import Path
            left = int(Path("/tmp/input/left.txt").read_text())
            right = int(Path("/tmp/input/right.txt").read_text())
            result = left + right
            Path("/tmp/output/answer.txt").write_text(str(result))
            """
        ),
    ],
    inputs=["left", "right"], # illustrative: named inputs
    outputs=["answer"],  # illustrative: named output
)
```
