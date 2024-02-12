pub struct MemoryLogger {
    buffer: std::sync::Mutex<Vec<u8>>,
}

impl MemoryLogger {
    pub fn new() -> MemoryLogger {
        MemoryLogger {
            buffer: std::sync::Mutex::new(Vec::new()),
        }
    }
}

impl std::io::Write for MemoryLogger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut buffer = self.buffer.lock().unwrap();
        println!("{}", std::str::from_utf8(buf).unwrap());
        buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
