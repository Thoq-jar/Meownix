#!/bin/bash

function show_version() {
    echo "MeowPkg Manager 1.0 R1"
}

function update() {
    echo "Updating MeowPkg..."
    sudo meow install meow
}

function check_args() {
    case $1 in
    "install")
        if [[ -z "$is_user_sudo" ]]; then
            install_program "$2"
            ask_to_run "$2"
        else
            echo "Please run as root."
            exit 1
        fi
        ;;
    "alt")
        if [[ -z "$is_user_sudo" ]]; then
            echo "Using flatpak to install $2 from Flathub..."
            flatpak install flathub "$2"
            echo "Installed $2 from Flathub."
            ask_to_run "$2"
        else
            echo "Please run as root."
            exit 1
        fi
        ;;
    "remove")
        if [[ -z "$is_user_sudo" ]]; then
            remove_program "$2"
        else
            echo "Please run as root."
            exit 1
        fi
        ;;
    *)
        echo "Unknown option: $1"
        exit 1
        ;;
    esac
}

function show_loading() {
    local pid=$1
    local delay=0.1
    local spin='-\|/'
    local i=0

    echo -n "Loading... "
    while ps -p $pid > /dev/null; do
        local char=${spin:i++%${#spin}:1}
        echo -ne "\r$char"
        sleep $delay
    done
    echo ""
}

function install_program() {
    local program_name="$1"
    show_loading $!
    sudo curl -L -o "/usr/local/bin/$program_name" "https://raw.githubusercontent.com/Thoq-jar/Meownix/main/pkgs/$program_name/$program_name" > /dev/null 2>&1
    local curl_exit_code=$?
    if [[ $curl_exit_code -eq 0 ]]; then
        sudo chmod +x "/usr/local/bin/$program_name"
        echo "Installed $program_name from Meow."
        echo "To run, type $program_name into terminal!"
    else
        echo "Failed to download $program_name from Meow."
        exit 1
    fi
}

function remove_program() {
    local program_name="$1"
    if [[ -f "/usr/local/bin/$program_name" ]]; then
        sudo rm "/usr/local/bin/$program_name"
        echo "Removed $program_name."
    else
        echo "$program_name is not installed via Meow!"
    fi
}

function ask_to_run() {
    local program_name="$1"
    read -p "Would you like to run '$program_name' now? [Y/n] " answer
    if [[ $answer =~ ^[Yy]$ ]] || [[ $answer = "" ]]; then
        sudo chmod +x "/usr/local/bin/$program_name"
        "/usr/local/bin/$program_name"
    fi
}

if [[ "$1" == "-v" ]]; then
    show_version
    exit 0
fi

if [[ "$1" == "-u" ]]; then
    if [[ -z "$is_user_sudo" ]]; then
        update
    else
        echo "Please run as root."
        exit 1
    fi
    exit 0
fi

is_user_sudo=$(sudo -v)

if [[ -z "$is_user_sudo" ]]; then
    check_args "$1" "$2"
else
    echo "Please run as root."
    exit 1
fi
