use libloading::Library;
use std::process;

fn main() {
    unsafe {
        #[cfg(target_os = "windows")]
        {
            let kernel32 = Library::new("kernel32.dll").unwrap();

            match kernel32.get::<*mut u32>(b"wine_get_unix_file_name") {
                Ok(_) => println!("Detected Wine"),
                Err(_) => println!("Did not detect Wine")
            }

            process::exit(0)
        }
        println!("This is intended to run on Windows");
        process::exit(1);
    }
}
