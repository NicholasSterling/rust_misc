use std::fs::File;
use std::panic;
use std::path::Path;
use backtrace::Backtrace;
use std::io::prelude::*;
use std::panic::PanicInfo;


fn save_backtrace(panic_info: &PanicInfo) -> Result<(), std::io::Error> {
    let backtrace = Backtrace::new();
    let path = Path::new("/Users/akash/rust/panic/src/rust_crash_backtrace.txt");
    let mut file = File::create(&path)?;
    write!(&mut file, "{}", panic_info)?;
    write!(&mut file, "##############################\n")?;
    write!(&mut file, "{:?}", backtrace)?;
    Ok(())
}
fn main()  {
    println!("Hello, world!");
    panic::set_hook(Box::new(|panic_info| {
        save_backtrace(panic_info).unwrap_or(());
    }));
    panic!("panic called");
}
