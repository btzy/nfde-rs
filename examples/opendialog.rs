use nfde::*;

fn main() -> Result<(), nfde::Error> {
    let nfd = Nfd::new()?;

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
