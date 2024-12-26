# shellcheck shell=bash

SYSUPGRADE_EXTENSION=".bin"

# This changed to .zst after 23.05
sdk_extension() {
	if [ "$RELEASE" = "snapshot" ] ; then
		echo ".tar.zst"
	else
		echo ".tar.xz"
	fi
}

# NOTE: must load device config first
build_image_builder_url() {
	if [ "$target" = "" ] || [ "$sub_target" = "" ] ; then
		echo "INVALID"
		exit 1
	fi

	if [ "$RELEASE" = "snapshot" ] ; then
		echo "https://downloads.openwrt.org/snapshots/targets/$target/$sub_target/openwrt-imagebuilder-$target-$sub_target.Linux-x86_64$(sdk_extension)"
	else
		echo "https://downloads.openwrt.org/releases/$RELEASE/targets/$target/$sub_target/openwrt-imagebuilder-$RELEASE-$target-$sub_target.Linux-x86_64$(sdk_extension)"
	fi
}

build_sdk_dir() {
	basename "$(build_image_builder_url)" "$(sdk_extension)"
}

build_archive_path() {
	basename "$image_builder_url"
}

build_image_file_path() {
	if [ "$RELEASE" = "snapshot" ] ; then
		image_release=""
	else
		image_release="-$RELEASE"
	fi
	echo "bin/targets/$target/$sub_target/openwrt$image_release-$EXTRA_IMAGE_NAME-${target}-${sub_target}-${PROFILE}-squashfs-sysupgrade${SYSUPGRADE_EXTENSION}"
}
