# Agents

An **Agent** in Orcapod is a worker that executes Pods.  
Agents can run locally or remotely, and multiple Agents can operate in parallel.  
They are coordinated by the orchestrator, which assigns Pods to available Agents.

## Key points
- Agents execute Pods either on the same machine or across remote machines.  
- They support **parallel execution**, allowing different Pods or branches of a pipeline to run at the same time.  
- Agents send **status signals** back to the orchestrator, including:
    - progress  
    - complete  
    - fail  
    - cancel  
- Execution is designed to be **fault-permissible**: if one Pod fails, only its dependent steps are cancelled, while other branches continue.  
- The long-term architecture goal is a **mesh of agents**, so there is no single point of failure.  

## Why Agents?
Agents make it possible to scale Orcapod pipelines beyond a single machine.  
They provide distributed execution while keeping results traceable and coordinated through the orchestrator.
