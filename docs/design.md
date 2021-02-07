# Open Pond Protocol Design, Version 0.1

## Overview

This document describes the design of the Open Pond Protocol software. It details the software
components and their purpose as well as the message structure of the Protocol messages. It will
also briefly describe some of the technical details provided in the current implementation of the
software.

## Message Structure

### Protocol Message

The Open Pond Protocol utilizes messages with the following structure:

| Field Name      | Bytes  | Description |
|-----------------|--------|-------------|
| Application ID  | 1      | Identifies which application to send message to or which application message originates from |
| Protocol Flags  | 1      | Flags used for internal protocol use |
| Response Port   | 2      | Port used to identify where to send data back to |
| Payload Length  | 2      | The length of the message payload |
| Message Payload | 0-1018 | Message payload |

### Protocol Flags

The protocol flags that are available in each message are:

| Flag | Value | Description |
|------|-------|-------------|
| Type | 0x80  | This flag indicates whether the message is an internal (protocol) message or a external (payload) message |

## Protocol Components

Each Open Pond node has three components:

* Request Writer
* Servicer
* Response Reader

The request writer and response reader act as the client side of data transactions while the
servicer acts as the server side of data transactions. The request writer generates request
messages in the first step of the data transfer process. The servicer reads these message requests
and stores them until the application is ready to service that request. Once the application reads
that request it is the responsibility of the application to return a response. The servicer's last
responsibility is to write the response back to the response reader of the node that made the
request. The response reader stores the message until the requesting application is ready to read
it.

## Protocol Design

The Open Pond Protocol operates at the application layer of the networking stack and is built on
top of the UDP transport protocol. Since the architecture of the Open Pond network is peer-to-peer,
every participant in the network needs to use a node that implements the protocol. The protocol
works using a request and response model. Every node in the network must initiate communication
with a request and every node in the network must be able to service requests with responses.

The request writer is responsible for generating the initial request and distributing that request
to other nodes in the network. In the current implementation, all requests are broadcast to all of
the other nodes known to that node. In the future, this will change to allow requests to individual
nodes.

The servicer is the most complex component of the architecture and is responsible for processing
the request, storing and delivering the request, waiting for the response, and finally delivering
the response back to the node that requested the message. The primary servicer thread reads new
requests and spawns new response handlers that keep track of the request and response generated
for that request. The response handler then places the request in the request mailbox of the
application that is it associated with utilizing the application ID. The response handler also
stores the response address and places its own port as the new return port. This feature allows
responses to come back to the handler before being forwarded to the actual response address. This
guarantees that a response will be tied to a specific request. The application is responsible for
reading requests from its specific mailbox. The application will be given the return information
so once a response is ready it can be send stright to the handler which sends the response back
to the requesting peer. At this point the handler completes it responsibilities and terminates.

The response reader reads and stores responses from all the requests to all of the applications
the node is supporting. It is the job of the response reader to place the response in the response
mailbox of the application it is associated with. The application is responsible for reading
responses from its specific mailbox. Once the application reads the message, this completes the
data communication process for a specific request.

An API is provided that allows applications:

* To write data to the request writer (creates a request)
* To read data from the request mailbox (get a peer's request)
* To write data back to the response handler (creates a response)
* To read data from the response mailbox (get a peer's response)
