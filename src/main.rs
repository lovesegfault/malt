const PROGRAM: &[u8] = &[
    // mov eax, 42 (0x2a)
    0xb8, 0x2a, 0x00, 0x00, 0x00, //
    // ret
    0xc3,
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut mmap = memmap::MmapOptions::new().len(PROGRAM.len()).map_anon()?;
    mmap.copy_from_slice(PROGRAM);
    let mmap = mmap.make_exec()?;
    let function: fn() -> u8 = unsafe { std::mem::transmute(mmap.as_ptr()) };
    let ret: u8 = function();
    assert_eq!(ret, 42);
    Ok(())
}
