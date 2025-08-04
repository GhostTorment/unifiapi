# UnifiApi

## Disclaimer

This project is not affiliated with, endorsed by, or sponsored by Ubiquiti Inc. (UniFi®).
All trademarks and copyrights belong to their respective owners.

## Crates

- [`sitemanager`](./sitemanager): Interface for Unifi Site Manager endpoint

## Usage

Each crate can be imported independently into your Rust project.
To use all of them together, you can include them as a workspace in your own project.

## Development

```bash
cargo build
cargo test
```

## Features
### Implemented
#### SiteManager API
- [x] List Sites
- [x] List Hosts
- [x] List SD-WAN Configs
- [x] Get ISP Metrics
- [x] Query ISP Metrics

### Not Yet Implemented
#### SiteManager API
- [ ] Get Host by ID
- [ ] List Devices
- [ ] Get SD-WAN Config by ID
- [ ] Get SD-WAN Config Status
#### Network API
##### About Application
- [ ] Get Application Info
##### Sites
- [ ] List Local Sites
##### Unifi Devices
- [ ] Execute Port Action
- [ ] Execute Device Action
- [ ] List Devices
- [ ] Get Device Details
- [ ] Get Latest Device Statistics
##### Clients
- [ ] Execute Client Action
- [ ] List Connected Clients
- [ ] Get Connected Client Details
##### Hotspot Vouchers
- [ ] List Vouchers
- [ ] Generate Vouchers
- [ ] Delete Vouchers
- [ ] Get Voucher Details
- [ ] Delete Voucher
