## Zeromq rust client and server

Working example of zeromq rust client and server

-   Uses Serde to serialise and deserialise json messages.

## How to run

Start up 1 instance
`cargo run --bin server`

Start up any number of workers, work gets distributed automatically by zmq
`cargo run --bin worker`
