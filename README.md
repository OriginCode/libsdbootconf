# libsdbootconf

A systemd-boot configuration and boot entry configuration parser library.

## Usage

```rust
use libsdbootconf::{SystemdBootConf, entry::{Entry, Token}};

let mut systemd_boot_conf = SystemdBootConf::new("/efi/loader");

systemd_boot_conf.config.default = "5.12.0-aosc-main".to_owned();
systemd_boot_conf.config.timeout = 5;

systemd_boot_conf.entries.push(Entry::new(
    "5.12.0-aosc-main",
    vec![Token::Title("AOSC OS (5.12.0-aosc-main)".to_owned())]
));

systemd_boot_conf.write_all().unwrap();
```