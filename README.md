## ZKP Protocol

This repository includes the client and the server implementation for the ZKP protocol. Both are implemented in separate Rust projects, though it would have ideally been a monorepo with shared libraries and shared protos, but done this way due to limited knowledge of Rust.

## Instructions to run

Run the server in a new terminal, and the client in another terminal. Follow the instructions of the console for running the client and interacting with the server.

```

cd server && cargo run
cd client && cargo run

```

## Instructions to run in docker and to deploy in a cloud

The client and server images need to be built separately and can be ran separately.

```

docker build -f Dockerfile-client -t zkp/client .
docker build -f Dockerfile-server -t zkp/server .

// Create a network
docker network create zkp

// Run server in detached mode in a new terminal
docker run -e SERVER_ADDRESS="0.0.0.0:8080" --network=zkp --name server -p 8080:8080 -d zkp/server

// Run client in interactive mode in a new terminal
docker run -e SERVER_ADDRESS="http://server:8080" --network=zkp --name client -it zkp/client

```

To run these in separate machines in the cloud, the individual docker images can be ran separately on different machines, while setting the env variable for SERVER_ADDRESS in the client accordingly to match the address and the port of the machine where the server is running.