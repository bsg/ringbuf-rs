use std::io::ErrorKind;

mod tests;

pub struct RingBuf<T, const SIZE: usize> {
    data: [T; SIZE],
    read_idx: usize,
    write_idx: usize,
}

impl<T: Default + Clone + Copy, const SIZE: usize> Default for RingBuf<T, SIZE> {
    fn default() -> Self {
        Self {
            data: [T::default(); SIZE],
            read_idx: 0,
            write_idx: 0,
        }
    }
}

impl<T, const SIZE: usize> RingBuf<T, SIZE> {
    pub fn is_empty(&self) -> bool {
        self.read_idx == self.write_idx
    }

    pub fn is_full(&self) -> bool {
        (self.read_idx + 1) % SIZE == self.write_idx
    }

    pub fn available_space(&self) -> usize {
        if self.is_empty() {
            SIZE
        } else if self.write_idx <= self.read_idx {
            self.read_idx - self.write_idx
        } else {
            SIZE - self.write_idx + self.read_idx
        }
    }

    pub fn clear(&mut self) {
        self.read_idx = 0;
        self.write_idx = 0;
    }
}

impl<const SIZE: usize> std::io::Write for RingBuf<u8, SIZE> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.available_space() < buf.len() {
            return Err(ErrorKind::StorageFull.into());
        }

        let tail_chunk_len = SIZE - self.write_idx;
        if tail_chunk_len >= buf.len() {
            self.data[self.write_idx..(self.write_idx + buf.len())].copy_from_slice(buf);
        } else {
            self.data[self.write_idx..].copy_from_slice(&buf[..tail_chunk_len]);
            self.data[0..(buf.len() - tail_chunk_len)].copy_from_slice(&buf[tail_chunk_len..]);
        }

        self.write_idx = (self.write_idx + buf.len()) % SIZE;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        match self.write(buf) {
            Ok(_) => Ok(()),
            Err(_) => Err(ErrorKind::StorageFull.into()),
        }
    }
}

impl<const SIZE: usize> std::io::Read for RingBuf<u8, SIZE> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if SIZE - self.available_space() < buf.len() {
            return Err(ErrorKind::UnexpectedEof.into());
        }

        let tail_chunk_len = SIZE - self.read_idx;
        if tail_chunk_len >= buf.len() {
            buf.copy_from_slice(&self.data[self.read_idx..(self.read_idx + buf.len())]);
        } else {
            buf[..tail_chunk_len].copy_from_slice(&self.data[self.read_idx..]);
            buf[tail_chunk_len..].copy_from_slice(&self.data[..self.write_idx]);
        }

        self.read_idx = (self.read_idx + buf.len()) % SIZE;

        Ok(buf.len())
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        match self.read(buf) {
            Ok(_) => Ok(()),
            Err(_) => Err(ErrorKind::UnexpectedEof.into()),
        }
    }
}
