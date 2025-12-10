# rs-machineid

Generate a unique machine ID for any host (without admin privileges).  

---

## Features

* Cross-Platform (tested on Win7+, Debian 8+, Ubuntu 14.04+, OS X 10.6+, FreeBSD 11+)
* No admin privileges required
* Hardware independent (no usage of MAC, BIOS or CPU — those are too unreliable, especially in a VM environment)
* Compatible with Docker containers

## Install

Add `rs-machineid` to your `Cargo.toml`:

```toml
[dependencies]
rs-machineid = "0.2.0"  # replace with latest version
```

## Usage

To obtain the raw GUID of the device, use `MachineId::get()` or `MachineId::get_hashed()`:

```rust
use rs_machineid::{MachineID};

println!("Machine ID: {}", MachineId::get());
println!("Machine ID Hash: {}", MachineId::get_hashed("test_app"));
```

## Thanks

Special thanks to Denis Brodbeck for his Go package, [`machineid`](https://github.com/denisbrodbeck/machineid).
