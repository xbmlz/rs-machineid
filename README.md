# rs-machineid

Generate a unique machine ID for any host (without admin privileges).  

This crate provides a simple, cross-platform way to obtain a unique identifier for a machine, compatible with physical machines, virtual machines, and Docker containers.  

---

## Install

Add `rs-machineid` to your `Cargo.toml`:

```toml
[dependencies]
rs-machineid = "0.1"  # replace with latest version
```

## Usage

To obtain the raw GUID of the device, use `get_machine_id()`:

```rust
use rs_machineid::{get_machine_id, get_machine_id_hash};

println!("Machine ID: {}", get_machine_id());
println!("Machine ID Hash: {}", get_machine_id_hash());
```

## Thanks

Special thanks to Denis Brodbeck for his Go package, [`machineid`](https://github.com/denisbrodbeck/machineid).
