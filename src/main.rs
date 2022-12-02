use windows::{
    core::Result,
    Win32::{
        Storage::FileSystem::GetFileType,
        System::Console::{GetConsoleMode, GetStdHandle, CONSOLE_MODE, STD_OUTPUT_HANDLE},
    },
};

fn main() -> Result<()> {
    let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) }?;
    let ty = unsafe { GetFileType(stdout) };
    eprintln!("HANDLE = 0x{:08x}", stdout.0);
    eprintln!("FileType = {}", ty);
    if ty == 2 {
        let mut mode = CONSOLE_MODE::default();
        unsafe { GetConsoleMode(stdout, &mut mode) }.ok()?;
        eprintln!("ConsoleMode = {}", mode.0);
    }
    Ok(())
}
