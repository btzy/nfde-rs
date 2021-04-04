# nfde-rs

This package contains Rust bindings for [Native File Dialog Extended (NFDe)](https://github.com/btzy/nativefiledialog-extended), a small library for invoking native file dialogs on Windows, MacOS, and Linux.

It supports four kinds of dialogs:
- Open file
- Open multiple files
- Save file
- Pick folder (under development)

This package should be regarded as **experimental** for now — while upstream NFDe is stable, these Rust bindings are still in flux.

For more information and screenshots, please see the upstream [NFDe](https://github.com/btzy/nativefiledialog-extended) repository.

## Dependencies

The following dependencies need to be installed on your machine manually (Cargo will not install it for you):
- CMake
- A decent C/C++ compiler (MSVC, Clang, or GCC are known to work)

You might also need to place CMake on your PATH so that the build script can find it.

## Basic Usage

TODO
