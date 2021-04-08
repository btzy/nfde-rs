# nfde-rs

This package contains Rust bindings for [Native File Dialog Extended (NFDe)](https://github.com/btzy/nativefiledialog-extended), a small library for invoking native file dialogs on Windows, MacOS, and Linux.

It supports four kinds of dialogs:
- Open file
- Open multiple files (under development, does not work yet)
- Save file
- Pick folder

This package should be regarded as **experimental** for now — while upstream NFDe is stable, these Rust bindings are still in flux.

For more information and screenshots, please see the upstream [NFDe](https://github.com/btzy/nativefiledialog-extended) repository.

## Dependencies

The following dependencies need to be installed on your machine manually (Cargo will not install it for you):
- CMake
- A decent C/C++ compiler (MSVC, Clang, or GCC are known to work)

You might also need to place CMake on your PATH so that the build script can find it.

## Basic Usage

```rust
use nfde::*;

fn main() -> Result<(), nfde::Error> {
    // Initialize NFD... NFD will be automatically deinitialized when this object is destroyed
    let nfd = Nfd::new()?;

    // Show the dialog...
    // Note: .show() will block until the dialog is closed
    // You can also set a default path using .default_path(Path)
    let res = nfd
        .open_file()
        .add_filter("Source code", "c,cpp,cc")?
        .add_filter("Headers", "h,hpp")?
        .show();

    match res {
        DialogResult::Ok(path_buf) => {
            println!("Success!");
            println!("Path: {}", path_buf.display());
        }
        DialogResult::Cancel => {
            println!("User pressed cancel.");
        }
        DialogResult::Err(error_str) => {
            println!("Error: {}", error_str);
        }
    };

    Ok(())
}
```

See the `/examples` directory for more examples.
