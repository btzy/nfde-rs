use nfde::*;

fn main() -> Result<(), nfde::Error> {
    // Initialize NFD... NFD will be automatically deinitialized when this object is destroyed
    let nfd = Nfd::new()?;

    // Show the dialog...
    // Note: .show() will block until the dialog is closed
    // You can also set a default path using .default_path(Path)
    let res = nfd
        .open_file_multiple()
        .add_filter("Source code", "c,cpp,cc")?
        .add_filter("Headers", "h,hpp")?
        .show();

    match res {
        DialogResult::Ok(pathset_buf) => {
            println!("Success!");
            for (i, path_buf) in pathset_buf.into_iter().enumerate() {
                // Note the '?' after 'path_buf', for Windows the underlying API is allowed to fail spuriously
                // For MacOS and Linux, it will never be an Err unless we run out of memory
                println!("Path {}: {}", i, path_buf?.display());
            }
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
