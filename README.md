The Open Pond Protocol is software to facilitate peer to peer communications and storage while
providing a base infrastructure for developers to build elegant decentralized applications.

## Installation

To run an Open Pond node, you need to download and install the [Rust](https://rust-lang.org/tools/install) 
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

## Bug Reporting

To report a bug or defect, it is helpful to understand a user's development environment and the
extent of the abnormal behavior they are experiencing. Simply open a new issue and provide a brief
description of the bug or defect in the title. Then in the description describe your development
environment, which commit you are building from, which version of Rust that is being used, and
other relevant information. This information helps to recreate the bug on our end. 

In a separate paragraph describe the unexpected or undesired behavior. The more detailed this
description is, the more likely we will be able to pinpoint the issue. Logs and screenshots
demonstrating the bug or defect are helpful as well.

## Contributions

To contribute to the project, either open a new issue for a bug or feature that currently is not
listed there or find a bug or feature present in the issue list that you would like to address. 
For features, simply provide a brief description of the feature and any design details that add
clarity to the work that needs to be done for the feature to be considered "complete". 

When setting up a Pull Request, remember to reference the issue number in the request so we know
what to look for during the review. Make sure the code is clean and free of extraneous comments or
notes. We'll get to the Pull Request as soon as we can and give our feedback on the contribution if
any is warranted. Once the request passes review, the issue will be closed and the contribution
should now appear in the `main` branch. 
