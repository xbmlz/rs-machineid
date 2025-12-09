# rs-machineid

Generate a unique machine ID for any host (without admin privileges).  

This crate provides a simple, cross-platform way to obtain a unique identifier for a machine, compatible with physical machines, virtual machines, and Docker containers.  

---

## Features

* Cross-Platform (tested on Win7+, Debian 8+, Ubuntu 14.04+, OS X 10.6+, FreeBSD 11+)
* No admin privileges required
* Hardware independent (no usage of MAC, BIOS or CPU — those are too unreliable, especially in a VM environment)
* IDs are unique<sup>[1](#unique-key-reliability)</sup> to the installed OS

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
println!("Machine ID Hash with optional appid: {}", get_machine_id_hash("my-app"));
```

## Thanks

Special thanks to Denis Brodbeck for his Go package, [`machineid`](https://github.com/denisbrodbeck/machineid).
