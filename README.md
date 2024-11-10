# Otus Smart Home

## TCP

### Client

To run client:

```sh
cargo run --bin socket-tcp-client <command>
```

Where `<command>` is one of:

- `status` - get status of the device
- `switch` - switch the device on/off

### Server

To run server:

```sh
cargo run --bin socket-tcp-server
```
