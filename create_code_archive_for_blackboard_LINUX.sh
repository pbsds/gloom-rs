#!/bin/sh
if test -f source.zip; then
	rm -v source.zip
fi

if [ -z "$1" ]; then
	echo "No folder specified"
	exit
fi 

DIR=$(echo $1 | sed 's:/*$::') # Remove trailing slash in path

zip -r source.zip \
	$DIR/Cargo.lock \
	$DIR/Cargo.toml \
	$DIR/src \
	$DIR/shaders \
	/resources/* \
	-x"resources/helicopter.obj" \
	-x"resources/lunarsurface.obj" \
	-x"resources/.gitkeep"

mv source.zip $DIR
