#!/bin/bash
# You'll need to have Docker running first to get this to work

docker pull ubuntu

docker run -i -t ubuntu /bin/bash

apt install curl

echo "deb http://us.archive.ubuntu.com/ubuntu/ jammy main universe" >> /etc/apt/sources.list

apt update