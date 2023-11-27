#!/bin/bash
# You'll need to have Docker running first to get this to work

docker pull ubuntu

docker run -i -t ubuntu /bin/bash

apt install curl

apt install aptitude

apt install llvm-dev

apt install g++ -y

apt install llvm

apt update

docker run -v /Users/caleblitalien/compiler-design:/home/compiler-design -it ubuntu bash

cd /home/compiler-design

g++ -o parser `llvm-config --libs core jit native --cxxflags --ldflags` *.cpp
