# FAQ

<div class="faq">
    <details>
        <summary>What is Docker and why does Orcapod use it?</summary>

        Docker is a platform that packages software and its dependencies into lightweight, portable containers.
        A container includes everything needed to run a program: code, libraries, and environment, so it behaves the same on any machine.<br>

        Orcapod uses Docker to ensure Pods are reproducible and isolated.
        This means a Pod that runs on your laptop will produce the same results on a cluster or in the cloud, without dependency conflicts or hidden environment differences.

    </details>


    <details>
        <summary>What is a Pod?</summary>

        <p>A Pod is the smallest unit of work in Orcapod.
        It runs a command inside a container and has declared inputs and outputs.
        Pods can run on their own or be connected into a pipeline.</p>
        See the <a href="/concepts/pods">Pods concept page</a> for details.
    </details>


    <details>
        <summary>What is a Pipeline?</summary>

        <p>A Pipeline is a directed graph of Pods.
        Outputs from one Pod become inputs to another, and pipelines can branch or merge.</p>
        See the <a href="/concepts/pipelines">Pipelines concept page</a> to learn more.
    </details>

    <details>
        <summary>What are map and join?</summary>

        <p>Operators connect Pods in a pipeline.</p>
        <ul>
            <li><b>map</b> retags outputs so they match the next Pod’s inputs.</li>
            <li><b>join</b> creates Cartesian products of inputs for fan-out parallelism.</li>
        </ul>
        <p>See the <a href="/concepts/operators">Operators concept page</a> for examples.</p>
    </details>

    <details>
        <summary>What is an Agent?</summary>

        <p>An Agent is a worker that executes Pods locally or remotely in parallel.
        Agents report status and logs back to the orchestrator.</p>
        See the <a href="/concepts/agents">Agents concept page</a> for details.
    </details>

    <details>
        <summary>What is the Orchestrator?</summary>

        The orchestrator schedules Pods, assigns them to Agents, and tracks run status.
        It also coordinates reads and writes with the store, and collects logs and metadata from runs.
    </details>

    <details>
        <summary>What is the Store?</summary>

        The store is where Pods read inputs and write outputs.
        In the current demo, this is a simple local file store that saves results to disk.
    </details>

    <details>
        <summary>What happens if a Pod fails?</summary>

        Only dependent steps are cancelled.
        Other branches continue to run.
        This makes Orcapod fault-permissive and resilient.
    </details>

</div>
