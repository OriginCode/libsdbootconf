# libsdbootconf

[![crates.io](https://img.shields.io/crates/v/libsdbootconf.svg)](https://crates.io/crates/libsdbootconf)
[![docs.rs](https://docs.rs/libsdbootconf/badge.svg)](https://docs.rs/libsdbootconf/)
[![MIT licensed](https://img.shields.io/crates/l/libsdbootconf.svg)](./LICENSE)

A systemd-boot configuration and boot entry configuration parser library.

## Usage

```rust
use libsdbootconf::{config::ConfigBuilder, entry::EntryBuilder, SystemdBootConfBuilder};

let systemd_boot_conf = SystemdBootConfBuilder::new("/efi/loader")
    .config(ConfigBuilder::new()
        .default("5.12.0-aosc-main")
        .timeout(5u32)
        .build())
    .entry(EntryBuilder::new("5.12.0-aosc-main")
        .title("AOSC OS x86_64 (5.12.0-aosc-main)")
        .version("5.12.0-aosc-main")
        .build())
    .build();

systemd_boot_conf.write_all().unwrap();
```

