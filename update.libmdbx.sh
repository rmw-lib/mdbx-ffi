#!/usr/bin/env bash

DIR=$(cd "$(dirname "$0")"; pwd)
set -ex
cd $DIR/..

if [ ! -d "libmdbx" ] ; then
git clone git@github.com:rmw-lib/libmdbx.git
cd libmdbx
git pull git@github.com:erthink/libmdbx.git
else
cd libmdbx
git pull git@github.com:erthink/libmdbx.git
fi

make dist

libmdbx=$DIR/libmdbx
rm -rf $libmdbx
cp -R dist $libmdbx
