use libloading::Library;
use std::process;

fn main() {
    // Libloading requires its relavant code to be run in an unsafe block
    unsafe {
        // This is to check if the program is running under Windows
        #[cfg(target_os = "windows")]
        {
            // We load up kernel32.dll. See the next comment.
            let kernel32 = Library::new("kernel32.dll").unwrap();

            // We check if getting the Wine-exclusive function `wine_get_unix_file_name` in kernel32 results in an error or not
            match kernel32.get::<*mut u32>(b"wine_get_unix_file_name") {
                Ok(_) => println!("Detected Wine"),
                Err(_) => println!("Did not detect Wine")
            }

            // We exit here so we do not reach the condition that the program is not running on Windows
            process::exit(0)
        }
    }
    // This is for the case the program is not compiled for Windows, but this could be modified anyway, so...
    println!("This is intended to run on Windows");
    process::exit(1);
}
