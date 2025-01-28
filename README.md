# OpenWrt Management Tool

This tool helps maintain OpenWrt routers.  The install, backup,
upgrade, and set-wifi subcommands require ssh authentication (although
it will also prompt for the password).

It is designed to easily upgrade to new releases without having to
reconfigure the device, as well as track configuration in source
control.

## Usage

```
# omt --help
Tool to help automate openwrt management

Usage: omt --names <NAMES> <COMMAND>

Commands:
  build-image       build firmware image
  install-image     install firmware image
  create-backup     create backup configuration
  restore-backup    restore backup configuration
  upgrade-packages  upgrade packages
  set-wifi          set wifi
  help              Print this message or the help of the given subcommand(s)

Options:
  -n, --names <NAMES>
  -h, --help           Print help
```

## Configuration

The base config is shared across all devices.

`conf/base.yml`:

```yaml
---
release: 23.05.5
disabled_services:
- odhcpd
sysupgrade_extension: .bin
packages:
- wpad-mesh-mbedtls

# Packages that come in the openwrt published image, but must be specified manually with the image builder
- luci
- luci-ssl
- iwinfo

extra_packages:
- tcpdump

package_removals:
- wpad-basic-mbedtls
```

And then each device will need it's own configuration, and can
optionally include configuration from the base config.

`conf/image-my-router.yml`:

```yaml
---
target: mediatek
sub_target: filogic
profile: bananapi_bpi-r4
extra_image_name: my-router
release: 24.10.0-rc6
sysupgrade_extension: .itb
packages:
- lm-sensors
# Stuff to include from base.yml
includes:
  packages: true
  extra_packages: true
  package_removals: true
  disabled_services: true
```
