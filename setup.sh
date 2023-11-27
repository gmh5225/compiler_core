#!/bin/bash
# You'll need to have Docker running first to get this to work

docker pull ubuntu

docker run -i -t ubuntu /bin/bash

apt install curl -y

apt install aptitude -y

apt install llvm-dev -y

apt install g++ -y

apt install llvm -y

apt install bison -y

apt install flex -y

apt update

docker run -v /Users/caleblitalien/compiler-design:/home/compiler-design -it ubuntu bash

cd /home/compiler-design

g++ -o parser `llvm-config --libs core jit native --cxxflags --ldflags` *.cpp
