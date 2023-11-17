# shellcheck shell=bash

# NOTE: must load device config first
build_image_builder_url() {
	if [ "$target" = "" ] || [ "$sub_target" = "" ] ; then
		echo "INVALID"
		exit 1
	fi

	if [ "$RELEASE" = "snapshot" ] ; then
		echo "https://downloads.openwrt.org/snapshots/targets/$target/$sub_target/openwrt-imagebuilder-$target-$sub_target.Linux-x86_64.tar.xz"
	else
		echo "https://downloads.openwrt.org/releases/$RELEASE/targets/$target/$sub_target/openwrt-imagebuilder-$RELEASE-$target-$sub_target.Linux-x86_64.tar.xz"
	fi
}

build_sdk_dir() {
	basename "$(build_image_builder_url)" .tar.xz
}

build_archive_path() {
	basename "$image_builder_url"
}

build_image_file_path() {
	echo "bin/targets/$target/$sub_target/openwrt-$RELEASE-$EXTRA_IMAGE_NAME-${target}-${sub_target}-${PROFILE}-squashfs-sysupgrade.bin"
}
