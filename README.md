# Ideas
- ExecutionContext for each execution rather than getting values directly from each node
- Give each graph an event queue
    - Event nodes register listeners on graph
    - Graph registers listener on core
    - Core delegates events to graphs with mpsc channel
    - Graph can process mpsc channel with `&mut self` without core requiring mutable access
    - May not work since graph event processing requires a lock on the graph