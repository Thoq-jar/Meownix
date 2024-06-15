#!/bin/bash

interrupt_handler() {
    echo -e "\nSIGINT Recieved, Exiting..."
    exit 0
}

trap interrupt_handler SIGINT

while :
do
    top
    sleep 1
done
