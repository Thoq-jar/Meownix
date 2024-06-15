#!/bin/bash

function show_loading() {
    local delay=0.1
    local spin='-\|/'
    local message=$1
    local pid=$2
    local i=0

    while ps -p $pid > /dev/null; do
        local char=${spin:i++%${#spin}:1}
        echo -ne "\r[$char] $message... "
        sleep $delay
    done
    echo ""
}

function show_version() {
    echo "MeowPkg Manager 1.0 R1"
}

function update() {
    echo "Updating MeowPkg..."
    sudo meow install meow > /dev/null 2>&1 &
    local pid=$!
    show_loading "Updating MeowPkg" $pid
    wait $pid
    echo "Installed meow from Meow."
    echo "To run, type meow into terminal!"
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

function install_program() {
    local program_name="$1"
    local download_url="https://raw.githubusercontent.com/Thoq-jar/Meownix/main/pkgs/$program_name/$program_name"
    local install_path="/usr/local/bin/$program_name"

    echo -n "[ ] Downloading $program_name from Meow... "
    sudo curl -fsSL -o "$install_path" "$download_url" > /dev/null 2>&1 &
    local pid=$!

    trap 'echo ""; echo "Download interrupted. Cleaning up..."; sudo kill -9 $pid > /dev/null 2>&1; exit 1' INT

    show_loading "Downloading $program_name from Meow" $pid

    wait $pid
    local curl_exit_code=$?

    trap - INT

    if [[ $curl_exit_code -eq 0 ]]; then
        sudo chmod +x "$install_path"
        echo "Installed $program_name from Meow."
        echo "To run, type $program_name into terminal!"
    else
        echo "Failed to download $program_name from Meow."
        exit 1
    fi
}

function remove_program() {
    local program_name="$1"
    echo -n "[ ] Removing $program_name... "
    sudo rm -f "/usr/local/bin/$program_name" > /dev/null 2>&1 &
    local pid=$!
    show_loading "Removing $program_name" $pid
    wait $pid
    local rm_exit_code=$?
    if [[ $rm_exit_code -eq 0 ]]; then
        echo "Removed $program_name."
    else
        echo "$program_name could not be removed or is not installed via Meow!"
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
