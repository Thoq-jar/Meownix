#!/bin/bash

clear

log() {

    [ "$2" ] || return

    name=$1

    {
        set -f
        set +f -- $2
        info=$*
    }

    printf '[%sC' "${ascii_width--1}"

    printf '[3%s;1m%s[m' "${PF_COL1-4}" "$name"

    printf %s "$PF_SEP"

    printf '[%sD[%sC' "${#name}" "${PF_ALIGN-$info_length}"

    printf '[3%sm%s[m\n' "${PF_COL2-7}" "$info"

    info_height=$((${info_height:-0} + 1))
}

get_title() {
    user=${USER:-$(id -un)}

    hostname=${HOSTNAME:-${hostname:-$(hostname)}}
    log "Meowfetch | Hostname: [3${PF_COL3:-1}m${user}${c7}@[3${PF_COL3:-1}m${hostname}" " " >&6
}

get_os() {
    if [ -f /etc/arch-release ]; then
        distro="$HOST"
    else
        distro="$HOST"
    fi

    [ "$distro" ] && {
        log os "$distro" >&6
        return
    }
}

    case $os in
        Linux*)
            if command -v lsb_release; then
                distro=$(lsb_release -sd)

            elif [ -d /system/app ] && [ -d /system/priv-app ]; then
                distro="Android $(getprop ro.build.version.release)"

            else
                while IFS='=' read -r key val; do
                    case $key in
                        PRETTY_NAME) distro=$val ;;
                    esac
                done < /etc/os-release
            fi

            distro=${distro##[\"\']}
            distro=${distro%%[\"\']}

            command -v crux && distro=$(crux)
            command -v guix && distro='Guix System'

            case $PATH in
                */bedrock/cross/*) distro='Bedrock Linux'
            esac

            [ "${kernel%%*-Microsoft}" ] ||
                distro="$distro on Windows 10 [WSL1]"

            distro="${distro}${WSLENV+ on Windows 10 [WSL2]}"
        ;;

        Darwin*)
            while IFS='<>' read -r _ _ line _; do
                case $line in
                    ProductVersion)
                        IFS='<>' read -r _ _ mac_version _
                        break
                    ;;
                esac
            done < /System/Library/CoreServices/SystemVersion.plist

            case $mac_version in
                10.4*)  distro='Mac OS X Tiger' ;;
                10.5*)  distro='Mac OS X Leopard' ;;
                10.6*)  distro='Mac OS X Snow Leopard' ;;
                10.7*)  distro='Mac OS X Lion' ;;
                10.8*)  distro='OS X Mountain Lion' ;;
                10.9*)  distro='OS X Mavericks' ;;
                10.10*) distro='OS X Yosemite' ;;
                10.11*) distro='OS X El Capitan' ;;
                10.12*) distro='macOS Sierra' ;;
                10.13*) distro='macOS High Sierra' ;;
                10.14*) distro='macOS Mojave' ;;
                10.15*) distro='macOS Catalina' ;;
                *)      distro='macOS' ;;
            esac

            distro="$distro $mac_version"
        ;;

        Haiku)
            distro=$(uname -sv)
        ;;

        Minix|DragonFly)
            distro="$os $kernel"

            trap '' EXIT
        ;;

        SunOS)
            IFS='(' read -r distro _ < /etc/release
        ;;

        *)
            distro="$os $kernel"
        ;;
    esac

get_kernel() {
    case $os in
        *BSD*|Haiku|Minix) 
            return 
        ;;
    esac

    log kernel "$kernel" >&6
}

get_host() {
    case $os in
        Linux*)
            read -r name    < /sys/devices/virtual/dmi/id/product_name
            read -r version < /sys/devices/virtual/dmi/id/product_version
            read -r model   < /sys/firmware/devicetree/base/model

            host="$name $version $model"
        ;;

        Darwin*|FreeBSD*|DragonFly*)
            host=$(sysctl -n hw.model)
        ;;

        NetBSD*)
            host=$(sysctl -n machdep.dmi.system-vendor \
                             machdep.dmi.system-product)
        ;;

        *BSD*|Minix)
            host=$(sysctl -n hw.vendor hw.product)
        ;;
    esac

    {
        set -f
        set +f -- $host
        host=
    }

    for word; do
        case $word in
            To      | [Bb]e      | [Ff]illed | [Bb]y  | O.E.M.  | OEM  |\
            Not     | Applicable | Specified | System | Product | Name |\
            Version | Undefined  | Default   | string | INVALID | �    | os )
                continue
            ;;
        esac

        host="$host$word "
    done

    log host "${host:-$arch}" >&6
}

get_uptime() {
    case $os in
        Linux*|Minix*)
            IFS=. read -r s _ < /proc/uptime
        ;;

        Darwin*|*BSD*|DragonFly*)
            s=$(sysctl -n kern.boottime)
            s=${s#*=}
            s=${s%,*}
            s=$(($(date +%s) - s))
        ;;

        Haiku)
            s=$(($(system_time) / 1000000))
        ;;

        SunOS)
            IFS='	.' read -r _ s _ <<-EOF
				$(kstat -p unix:0:system_misc:snaptime)
			EOF
        ;;

        IRIX)
            t=$(LC_ALL=POSIX ps -o etime= -p 1)
            case $t in 
                *-*)   d=${t%%-*} t=${t#*-} ;;
                *:*:*) h=${t%%:*} t=${t#*:} ;;
            esac

            h=${h#0} t=${t#0}
            s=$((${d:-0}*86400 + ${h:-0}*3600 + ${t%%:*}*60 + ${t#*:}))
        ;;
    esac

    d=$((s / 60 / 60 / 24))
    h=$((s / 60 / 60 % 24))
    m=$((s / 60 % 60))

    [ "$d" = 0 ] || uptime="${uptime}${d}d "
    [ "$h" = 0 ] || uptime="${uptime}${h}h "
    [ "$m" = 0 ] || uptime="${uptime}${m}m "

    log uptime "${uptime:-0m}" >&6
}

get_pkgs() {
    has() { command -v "$1" >/dev/null; }
    packages=`
        case $os in
            Linux*)
                has bonsai     && bonsai list
                has crux       && pkginfo -i
                has pacman-key && pacman -Qq
                has dpkg       && dpkg-query -f '.\n' -W
                has rpm        && rpm -qa
                has xbps-query && xbps-query -l
                has apk        && apk info
                has guix       && guix package --list-installed
                has opkg       && opkg list-installed

                has kiss       && printf '%s\n' /var/db/kiss/installed/*/
                has brew       && printf '%s\n' "$(brew --cellar)/"*
                has emerge     && printf '%s\n' /var/db/pkg/*/*/
                has pkgtool    && printf '%s\n' /var/log/packages/*
                has eopkg      && printf '%s\n' /var/lib/eopkg/package/*

                has nix-store  && {
                    nix-store -q --requisites /run/current-system/sw
                    nix-store -q --requisites ~.nix-profile
                }
            ;;

            Darwin*)
                has pkgin      && pkgin list

                has brew       && printf '%s\n' /usr/local/Cellar/*

                has port       && {
                    pkg_list=$(port installed)

                    [ "$pkg_list" = "No ports are installed." ] ||
                        printf '%s\n' "$pkg_list"
                }
            ;;

            FreeBSD*|DragonFly*)
                pkg info
            ;;

            OpenBSD*)
                printf '%s\n' /var/db/pkg/*/
            ;;

            NetBSD*)
                pkg_info
            ;;

            Haiku)
                printf '%s\n' /boot/system/package-links/*
            ;;

            Minix)
                printf '%s\n' /usr/pkg/var/db/pkg/*/
            ;;

            SunOS)
                has pkginfo && pkginfo -i
                has pkg     && pkg list
            ;;

            IRIX)
                versions -b
            ;;
        esac | wc -l
    `

    case $os in
        IRIX) packages=$((packages - 3)) ;;
    esac

    [ "$packages" -gt 1 ] && log meows "$packages" >&6
}

get_memory() {
    case $os in
        Linux*)
            while IFS=':k '  read -r key val _; do
                case $key in
                    MemTotal)
                        mem_used=$((mem_used + val))
                        mem_full=$val
                    ;;

                    Shmem)
                        mem_used=$((mem_used + val))
                    ;;

                    MemFree|Buffers|Cached|SReclaimable)
                        mem_used=$((mem_used - val))
                    ;;
                esac
            done < /proc/meminfo

            mem_used=$((mem_used / 1024))
            mem_full=$((mem_full / 1024))
        ;;

        Darwin*)
            mem_full=$(($(sysctl -n hw.memsize) / 1024 / 1024))

            while IFS=:. read -r key val; do
                case $key in
                    *' wired'*|*' active'*|*' occupied'*)
                        mem_used=$((mem_used + ${val:-0}))
                    ;;
                esac

            done <<-EOF
                $(vm_stat)
			EOF

            mem_used=$((mem_used * 4 / 1024))
        ;;

        OpenBSD*)
            mem_full=$(($(sysctl -n hw.physmem) / 1024 / 1024))

            while read -r _ _ line _; do
                mem_used=${line%%M}
            done <<-EOF
                $(vmstat)
			EOF
        ;;

        FreeBSD*|DragonFly*)
            mem_full=$(($(sysctl -n hw.physmem) / 1024 / 1024))

            {
                set -f
                set +f -- $(sysctl -n hw.pagesize \
                                      vm.stats.vm.v_inactive_count \
                                      vm.stats.vm.v_free_count \
                                      vm.stats.vm.v_cache_count)
            }

            mem_used=$((mem_full - (($2 + $3 + $4) * $1 / 1024 / 1024)))
        ;;

        NetBSD*)
            mem_full=$(($(sysctl -n hw.physmem64) / 1024 / 1024))

            while IFS=':k ' read -r key val _; do
                case $key in
                    MemFree)
                        mem_free=$((val / 1024))
                        break
                    ;;
                esac
            done < /proc/meminfo

            mem_used=$((mem_full - mem_free))
        ;;

        Haiku)
            IFS='( )' read -r _ _ _ _ mem_used _ mem_full <<-EOF
                $(sysinfo -mem)
			EOF

            mem_used=$((mem_used / 1024 / 1024))
            mem_full=$((mem_full / 1024 / 1024))
        ;;

        Minix)
            read -r _ mem_full mem_free _ < /proc/meminfo

            mem_used=$(((mem_full - mem_free) / 1024))
            mem_full=$(( mem_full / 1024))
        ;;

        SunOS)
            hw_pagesize=$(pagesize)
            while read -r key val; do
                case $key in
                    *total) pages_full=$val ;;
                    *free)  pages_free=$val ;;
                esac
            done <<-EOF
				$(kstat -p unix:0:system_pages:pagestotal \
                           unix:0:system_pages:pagesfree)
			EOF

            mem_full=$((pages_full * hw_pagesize / 1024 / 1024))
            mem_free=$((pages_free * hw_pagesize / 1024 / 1024))
            mem_used=$((mem_full - mem_free))
        ;;

        IRIX)
            while IFS=' :' read -r label mem_full _ mem_free _; do
                case $label in
                    Memory) 
                        mem_full=${mem_full%M}
                        mem_free=${mem_free%M}
                        break 
                    ;;
                esac
            done <<-EOF
                $(top -n)
			EOF

            mem_used=$((mem_full - mem_free))
        ;;
    esac

    log meowry "${mem_used:-?}M / ${mem_full:-?}M" >&6
}

get_wm() {
    case $os in
        Darwin*) ;;

        *)
            [ "$DISPLAY" ] || return
            command -v xprop && {
                id=$(xprop -root -notype _NET_SUPPORTING_WM_CHECK)
                id=${id##* }
                wm=$(xprop -id "$id" -notype -len 25 -f _NET_WM_NAME 8t)
                case $wm in
                    *'_NET_WM_NAME = '*)
                        wm=${wm##*_NET_WM_NAME = \"}
                        wm=${wm%%\"*}
                    ;;

                    *)
                        wm=$(ps x | grep -o \
                                         -e '[c]atwm' \
                                         -e '[f]vwm' \
                                         -e '[d]wm' \
                                         -e '[2]bwm' \
                                         -e '[m]onsterwm' \
                                         -e '[w]maker' \
                                         -e '[s]owm')
                    ;;
                esac
            }
        ;;
    esac

    log wm "$wm" >&6
}


get_de() {
    log de "${XDG_CURRENT_DESKTOP:-$DESKTOP_SESSION}" >&6
}

get_shell() {
    log shell "${SHELL##*/}" >&6
}

get_editor() {
    log editor "${VISUAL:-$EDITOR}" >&6
}

get_palette() {
    palette="[7m$c1 $c1 $c2 $c2 $c3 $c3 $c4 $c4 $c5 $c5 $c6 $c6 [m"
    printf '\n' >&6
    log "$palette 
        " " " >&6
}

get_ascii() {
    read_ascii() {
        PF_COL1=${PF_COL1:-${1:-7}}
        PF_COL3=${PF_COL3:-$((${1:-7}%8+1))}

        while IFS= read -r line; do
            ascii="$ascii$line
"
        done
    }

    case ${1:-${PF_ASCII:-${distro:-$os}}} in
        *)
            read_ascii 5 <<-EOF
				${c5}---Meownix---
				${c5}--Meowfetch--
			EOF
        ;;

        *)
            [ "$1" ] || {
                get_ascii "$os"
                return
            }

            printf 'error: %s is not currently supported.\n' "$os" >&6
            printf 'error: Open an issue for support to be added.\n' >&6
            exit 1
        ;;
    esac

    while read -r line; do
        ascii_height=$((${ascii_height:-0} + 1))
        [ "${#line}" -gt "${ascii_width:-0}" ] &&
            ascii_width=${#line}
    done <<-EOF
 		$(printf %s "$ascii" | sed 's/\[3.m//g')
	EOF
    ascii_width=$((ascii_width + 4))

    printf '[1m%s[m[%sA' "$ascii" "$ascii_height" >&6
}

main() {
    [ "$1" = -v ] || exec 2>/dev/null

    exec 6>&1 >/dev/null

    . "${PF_SOURCE:-/dev/null}" ||:

    [ -w "${TMPDIR:-/tmp}" ] || export TMPDIR=~

    {
        c1='[31m'; c2='[32m'
        c3='[33m'; c4='[34m'
        c5='[35m'; c6='[36m'
        c7='[37m'; c8='[38m'
    }

    [ "$TERM" = dumb ]   ||
    [ "$TERM" = minix ]  ||
    [ "$TERM" = cons25 ] || {
        printf '[?7l' >&6
        trap 'printf [?7h >&6' EXIT
    }

    read -r os kernel arch <<-EOF
		$(uname -srm)
	EOF
    get_os

    {
        set -f
        set +f ${PF_INFO-ascii title os host kernel uptime pkgs memory}

        for info; do
            command -v "get_$info" >/dev/null || continue

            [ "${#info}" -gt "${info_length:-0}" ] &&
                info_length=${#info}
        done

        info_length=$((info_length + 1))

        for info; do "get_$info"; done
    }

    [ "${info_height:-0}" -lt "${ascii_height:-0}" ] &&
        cursor_pos=$((ascii_height - info_height))

    while [ "${i:=0}" -le "${cursor_pos:-0}" ]; do
        printf '\n'
        i=$((i + 1))
    done >&6
}
main "$@"
