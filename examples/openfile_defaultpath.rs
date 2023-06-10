use nfde::*;

fn main() -> Result<(), nfde::Error> {
    // Initialize NFD... NFD will be automatically deinitialized when this object is destroyed
    let nfd = Nfd::new()?;

    let current_exe_path = std::env::current_exe().map_err(|_| "Cannot get current exe path")?;

    // Show the dialog...
    // Note: .show() will block until the dialog is closed
    let res = nfd
        .open_file()
        .default_path(&(current_exe_path.parent().ok_or("Cannot get parent dir")?))?
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
