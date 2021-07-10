#! /usr/bin/env bash

dir=examples

function do_build
{
    cd $dir
    cargo rustc -- -C link-args='-Wl,-undefined,dynamic_lookup'
    cd -
}

function do_test
{
    php74 -d extension=examples/target/debug/libhelloworld.so vendor/bin/phpunit $*
}

do_build
do_test "$*"