#!/bin/bash

export LD_LIBRARY_PATH="$(pwd)/target/debug/"
echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
cd src/ 
javac -h . HelloWorld.java
javac HelloWorld.java 
java HelloWorld -Djava.library.path="LD_LIBRARY_PATH"
cd ../