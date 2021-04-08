use nfde::*;

// Filters for the dialog
const filters: &[(&str, &str)] = &[("Source code", "c,cpp,cc"), ("Headers", "h,hpp")];

fn main() -> Result<(), nfde::Error> {
    // Initialize NFD... NFD will be automatically deinitialized when this object is destroyed
    let nfd = Nfd::new()?;

    // Show the dialog...
    // This example appends an iterator of filters,
    // instead of adding one after another; existing filters will remain present
    let res = nfd
        .open_file()
        .add_filters(filters.into_iter().copied())?
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
