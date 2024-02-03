# FundSimulatorRS
FundSimulatorRS is a versatile Rust-based program for simulating the behaviour of index funds through easy-to-configure settings.


## Usage
The application can run in two modes: server and CLI. Both modes use a JSON file as input. It must include the following information:
- `deposit`: Integer representing the initial deposit. 
- `years`: Integer representing the number of years for the simulation.
- `return_rates`: Float or list of floats. If a single float, then the same return rate is applied for all the years. Otherwise, the number of elements in the list must be equal to the number of years.
- `annual_contributions`: Float or list of floats. If a single float, then the same annual contribution is applied for all the years. Otherwise, the number of elements in the list must be equal to the number of years.

### CLI mode
```
cargo run -- --mode cli --config-file example.json
```

### Server mode
Run the following command or `docker compose run` to start the server.
```
cargo run -- --mode server
```
It will be listening on port 3000 by default.
The endpoint is `/simulate` and you need to pass the config json in the payload.
