use std::io;
#[cfg(windows)] use winres::WindowsResource;

fn main() -> io::Result<()> {
    #[cfg(windows)] {
        // Add app logo to binaries built on windows.
        WindowsResource::new()
            .set_icon("resources/icons/logo.ico")
            .compile()?;
    }
    Ok(())
}

