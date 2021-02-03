The Open Pond Protocol is software to facilitate peer to peer communications and storage while
providing a base infrastructure for developers to build elegant decentralized applications.

*NOTE:* The Open Pond Protocol software is in the early stages of development. It is recommended
that users only use this software if they know what they are doing to prevent software misuse and
potential security holes.

## Installation

To run an Open Pond node, you need to download and install the [Rust](https://rust-lang.org/tools/install) 
programming language and the Cargo package manager to build binaries.

## Usage

Once the environment has been configured, compile and run the binary, entering the configuration
file as the only argument. Information on how to format the configuration file can be found below.

```
// Command to run (Example Node)
cargo run config/example.toml
```

This command will start a node and if connected to another node will enable communication between
the two nodes. In order for anything useful to be communicated, each node needs to have 1 or more
applications connected to their node. The applications must have matching IDs to communicate. These
IDs can be set in the node configuration file described later in this document. To run the
Akron example application, use the following command:

```
// Command to run (Example App)
cargo run --example akron config/example.toml
```

## Node Configuration

Each node in the network runs using a configuration file. This configuration has four different
sections: Settings, Local, Peers, and Apps. All four items in the configuration are required for
the correct operation of the Open Pond node.

### Settings
The settings section of the configuration file is used to define different components within the 
node itself. The provided fields are port assignments for the three different components within a
Open Pond node: the request writer, the service manager, and the response reader. These ports are
required to be unique and free prior to starting the node up.

```
[settings]
requester_write = 9090
responder_read = 9091
servicer_manager = 9092
```

### Local
The local section of the configuration file is used to define an individual user's node. The
address field is used to define the IP address and port of the public facing endpoint of the
application servicer. The name field ties a human readable string to the address.

```
[local]
address = "192.168.0.1:8081"
name = "Antelope"
```

### Peers
The peers section of the configuration file is used to identify one or more peer nodes. Since there
can be more than one peer in the configuration file, the TOML format requires double brackets
around the section title and must be used before each peer. Like the local section, the address
field is used to define the IP address and port of the public servicer endpoint and the name field
provides a human readable string to help identify that address. 

```
[[peers]]
address = "192.168.0.2:8081"
name = "Bear"

[[peers]]
address = "192.168.0.3:8081"
name = "Cheetah"
```

### Apps
The apps section of the configuration file is used to identify one or more applications. All apps
need double brackets for the same reason as the peers section. The ID field is used to distinguish
apps from each other. In order to connect to the same application on another node, the ID must be
matching. The name field provides a human readable string to associate with that application ID.

```
[[apps]]
id = 0
name = "Akron"
```

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
