# Operators

Operators connect Pods by transforming outputs into inputs for the next step.

---

## Map Operator

The **map operator** retags outputs from one Pod so they align with the expected inputs of the next Pod.

### Input/Output Specs

Pods define input/output specs.  
For example, an `add` Pod may have:

- **Inputs:** `left`, `right`
- **Output:** `answer`

These names act like argument names in a Python function: they label what values the Pod expects and produces.

---

### Why `map` is needed

In practice, output names from one Pod often don’t match the input names of the next Pod. For example:

- **Pod A** produces an output called `answer`
- **Pod B** expects an input called `left`

Without help, Pod B won’t know how to consume Pod A’s output.  
That’s where the **map operator** comes in.

---

### What `map` does

The **map operator** renames outputs from an upstream Pod so they match the expected input names of the downstream Pod.

- **Input:** a dictionary of outputs (e.g., `{"answer": <data blob>}`)
- **Config:** a rename dictionary, e.g. `{"answer": "left"}`
- **Effect:** retags the output key `answer` as `left`

Notes:

- Keys not present in the upstream output are ignored (no error).
- You can map one or many keys at a time.

---

### Example: Map

```python
# Example: using a map operator to rename 'answer' -> 'left'

map_left = {"answer": "left"}

pipeline = Pipeline(
    graph_dot="""
    digraph {
        add_a -> map_left_a -> add_b
    }
    """,
    metadata={
        "add_a": Kernel.POD(ref=add_pod),
        "add_b": Kernel.POD(ref=add_pod),
        "map_left_a": Kernel.MAP_OPERATOR(map=map_left),
    },
    input_spec={
        "left_add_a":  [InputSpecUri(node="add_a", key="left")],
        "right_add_a": [InputSpecUri(node="add_a", key="right")],
    },
    output_spec={"answer": OutputSpecUri(node="add_b", key="answer")},
)
```

Here:

- `add_a` produces `answer`
- `map_left_a` renames `answer → left`
- `add_b` can now consume it as its `left` input

## Join Operator

The **join operator** combines outputs from multiple parent Pods by producing the **Cartesian product** of their values.  
This enables **combinatorial expansion**: every output from parent A is paired with every output from parent B (and so on).

---

### How it works

- Suppose **Parent A** produces 2 outputs, and **Parent B** produces 3 outputs.
- The join operator will generate **2 × 3 = 6 unique combinations**.
- Each combination is treated as a distinct input for the child Pod.

As new outputs arrive, the join operator continues producing combinations:

- If **Parent A** later produces a 3rd output (while B still has 3),  
  → 3 new combinations are generated (making 9 total).
- Processing happens asynchronously: combinations are queued and executed as upstream outputs become available.

---

### Key points

- **No config needed:** A join operator has no parameters. You just declare it as a join, and Orcapod automatically inspects parent outputs to compute the Cartesian product.
- **Incremental:** As parents emit more outputs, new combinations are created on the fly.
- **Analogy:** Similar to a SQL join where foreign keys reference multiple tables → Cartesian product.

---

### Example: Join

```python
# Example: join operator producing the Cartesian expansion of two mapped streams

pipeline = Pipeline(
    graph_dot="""
    digraph {
        add_a -> map_left_a
        add_b -> map_right_a
        { map_left_a map_right_a } -> cartesian_a
    }
    """,
    metadata={
        "add_a":       Kernel.POD(ref=add_pod),
        "add_b":       Kernel.POD(ref=add_pod),
        "map_left_a":  Kernel.MAP_OPERATOR(map={"answer": "left"}),
        "map_right_a": Kernel.MAP_OPERATOR(map={"answer": "right"}),
        "cartesian_a": Kernel.JOIN_OPERATOR(),  # no config
    },
    input_spec={
        "left_add_a":  [InputSpecUri(node="add_a", key="left")],
        "right_add_a": [InputSpecUri(node="add_a", key="right")],
        "left_add_b":  [InputSpecUri(node="add_b", key="left")],
        "right_add_b": [InputSpecUri(node="add_b", key="right")],
    },
    # No output_spec on purpose: join feeds a downstream Pod in real pipelines.
)
```

Here:

- `add_a` produces an `answer`, which `map_left_a` retags as `left`.
- `add_b` produces an `answer`, which `map_right_a` retags as `right`.
- `cartesian_a` pairs every `left` from `add_a` with every `right` from `add_b`, producing all combinations.
