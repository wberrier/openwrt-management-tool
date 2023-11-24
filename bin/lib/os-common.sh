# shellcheck shell=bash
# Intended to be included to provide common functionality

#
# Returns a string containing the current Linux distribution
#
get_dist() {
	dist="nodist"
	if [ -e /etc/fedora-release ]; then
		dist=$(cat /etc/fedora-release | sed -e "s/.*release\s\([0-9]\+\).*/Fedora-\1/")
	elif [ -e /etc/redhat-release ]; then
		dist=$(cat /etc/redhat-release | sed -e "s/.*release\s\([0-9]\+\).*/RedHat-\1/")
	elif [ -e /etc/debian_version ]; then
		# TODO: does this work on olders versions than debian 10?
		dist=$(cat /etc/debian_version | sed -e "s/\([0-9]\+\).*/Debian-\1/")
	elif [ -e /etc/lsb-release ]; then
		id=$(grep DISTRIB_ID /etc/lsb-release | awk -F'=' '{print $2}')
		release=$(grep DISTRIB_RELEASE /etc/lsb-release | awk -F'=' '{print $2}')
		dist="${id}-${release}"
	fi

	arch=$(uname -m)

	# TODO: combine i[356]86?

	echo "$dist-$arch"
}

package_system() {
	case $(get_dist) in
	Fedora* | RedHat*)
		echo "rpm"
		;;
	Debian* | Ubuntu*)
		echo "dpkg"
		;;
	*)
		echo "UNSUPPORTED"
		;;
	esac
}

install_packages() {
	case $(package_system) in
	rpm)
		sudo dnf install -y --skip-broken --allowerasing "$@"
		;;
	dpkg)
		sudo apt-get update
		sudo DEBIAN_FRONTEND=noninteractive apt-get install -yq "$@"
		;;
	*)
		echo "unsupported platform"
		;;
	esac
}

remove_packages() {
	case $(package_system) in
	rpm)
		sudo yum remove -y "$@"
		;;
	dpkg)
		sudo DEBIAN_FRONTEND=noninteractive apt-get remove --purge -yq "$@"
		;;
	*)
		echo "unsupported platform"
		;;
	esac
}

list_package_contents() {
	case $(package_system) in
	rpm)
		rpm -ql "$@"
		;;
	dpkg)
		dpkg -L "$@"
		;;
	*)
		echo "unsupported platform"
		;;
	esac
}

package_installed() {
	case $(package_system) in
	rpm)
		rpm -q "$1" >/dev/null
		;;
	dpkg)
		dpkg -s "$1" 2>/dev/null | grep Status: | grep -q installed
		;;
	*)
		echo "unsupported platform"
		false
		;;
	esac
}

package_version() {
	case $(package_system) in
	rpm)
		if version=$(rpm -q "$1" --queryformat "%{VERSION}\n"); then
			echo "$version"
		else
			echo "0"
			false
		fi
		;;
	dpkg)
		if version=$(dpkg-query --showformat='${source:Upstream-Version}\n' --show "$1" 2>/dev/null); then
			echo "$version"
		else
			echo "0"
			false
		fi
		;;
	*)
		echo "unsupported platform"
		false
		;;
	esac
}

remote_package_version() {
	case $(package_system) in
	rpm)
		if output=$(sudo yum list "$1"); then
			echo "$output" | tail -n1 | awk '{print $2}' | cut -d '-' -f 1
		fi
		;;
	dpkg)
		sudo apt-get update >/dev/null 2>&1
		if output=$(apt-cache show "$1"); then
			echo "$output" | grep Version: | awk '{print $2}'
		fi
		;;
	esac
}

install_rpm_from_url_if_not_installed() {
	url="$1"
	rpm_name="$2"

	if ! package_installed "$rpm_name"; then
		install_packages "$url"
	else
		echo "$2 already installed"
	fi
}

pipx_append_proxy_cert() {
	venv_name="$1"
	cert="http://berrier.org/webbarrier/BerrierServer.crt"

	# If this environment uses cretifi, append the cert
	# NOTE: globs return multiple results, loop over them (even though there's likely one)
	for cert_path in "$HOME/.local/share/pipx/venvs/$venv_name"/lib/python*/site-packages/certifi/cacert.pem ; do
		openssl x509 -in "$cert" -text >> "$cert_path"
	done
}

install_python_app() {
	# Parse some arguments for passing to pipx
	python_options=""
	opts=$(getopt -a -o p: --longoptions python: -- "$@")
	eval set -- "${opts}"
	unset opts
	while true ; do
		case $1 in
		-p | --python)
			python_options="--python $2"
			shift 2
			continue
			;;
		--)
			shift
			break
			;;
		*)
			echo "Unexpected arg: $1"
			shift
			break
			;;
		esac
		shift
	done

	pipx install --force --include-deps $python_options "$@"

	pipx_append_proxy_cert "$@"
}

add_python_packages() {
	package="$1" ; shift
	pipx inject --force --include-deps --include-apps "$package" "$@"
}

flatpak_package_installed() {
	type flatpak >/dev/null 2>&1 && flatpak info "$1" >/dev/null 2>&1
}

install_flatpaks() {
	# NOTE: running on debian seemed to require sudo
	# Not sure if there's an alternative?
	sudo flatpak install -y --noninteractive --system "$@"
}

add_flatpak_remote() {
    sudo flatpak remote-add --if-not-exists "$1" "$2"
}

set_apt_components() {
    sources_list="$1"
    shift
    components="$@"

    # TODO: this only supports the new deb822 style...
    sudo sed -i "s/^Components:/Components: $components/g" "$sources_list"
}
