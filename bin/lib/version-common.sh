# shellcheck shell=bash

version_is_less_than() {
	ver1="$1"
	ver2="$2"

	older_version=$( (
		echo "$ver1"
		echo "$ver2"
	) | sort --version-sort | head -n1)
	[ "$older_version" = "$ver1" ]
}
