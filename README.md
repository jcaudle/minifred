# Minifred -- some software for the heart of a machine

This is a toy operating system built following the guides provided on [the second edition of Phil Opp's OS blog](https://os.phil-opp.com). It is a successor to the other toy OS I started on called [winnie](https://github.com/jcaudle/winnie). It is designed to be built on a Mac.

## Prerequisites

This software can be built with the `cargo-xbuild` crate, rust source code, and `bootimage` 0.5.x tool installed.

## Build instructions

This software depends on the nightly build of Rust and Cargo. It can be built with the following command:

    bootimage build

## Running in QEMU

Assuming that QEMU is installed, the OS can be run with `bootimage run`.
