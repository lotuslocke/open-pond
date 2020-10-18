The Open Pond Protocol is software to facilitate peer to peer communications and storage while
providing a base infrastructure for developers to build elegant decentralized applications.

## Installation

To run an Open Pond node, you need to download and install the [Rust](rust-lang.org/tools/install) 
programming language and the Cargo package manager to build binaries.

## Usage

Once the environment has been configured, compile and run the binary, entering the selected address
and port of your node as the first argument and the target node as the second argument:

```
// Command to run (Example)
cargo run "127.0.0.1:8091" "127.0.0.1.8090"
```

This command will start a node and if connected to another node will allow command line
communication between the two nodes.