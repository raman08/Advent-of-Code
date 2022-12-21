#!/bin/bash

echo "Creating day $1"

if [ -d day_$1 ]
then
	echo "Directory for day $1 already exist!!!!"
	exit 0
fi

mkdir day_$1

cp ./base.cpp day_$1/$1_1.cpp
cp ./base.cpp day_$1/$1_2.cpp
touch day_$1/input-ex.txt
touch day_$1/input.txt
