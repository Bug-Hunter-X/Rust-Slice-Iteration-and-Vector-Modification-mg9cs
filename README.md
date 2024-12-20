# Rust Slice Iteration and Vector Modification Bug

This repository demonstrates a common error in Rust: attempting to iterate over a slice while modifying the underlying vector.  The code includes both the buggy version and a corrected version, illustrating how to avoid this error. This error occurs because slices borrow the underlying vector's data, and any modification to the vector while the slice is in use breaks Rust's borrow checker rules, causing a runtime panic.

## Running the Code

1. Clone the repository: `git clone ...`
2. Navigate to the repository directory: `cd ...`
3. Compile and run the buggy code: `rustc bug.rs && ./bug` (This will panic)
4. Compile and run the corrected code: `rustc bugSolution.rs && ./bugSolution` (This will run successfully)

## Understanding the Bug and Solution

The bug stems from the simultaneous access and mutation of the same memory location. The solution involves making a copy of the vector's contents before processing it or using an iterator that doesn't require mutable borrowing of the underlying vector. 
