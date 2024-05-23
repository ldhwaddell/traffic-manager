# traffic-manager
Rust implementation of Project 2, CSCI 3431, Operating Systems. SMU Winter Term 2023. 

Some changes have been made. This will use multithreading and mutex locks instead of multiprocessing and semaphores. 

The deadlock detection will be done through a graph. Nodes and edges will be updated by the bus processes, with access controlled by mutex locks. 

