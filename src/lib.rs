use std::io::{ErrorKind, Read, Result, Write};

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
        (self.write_idx + 1) % SIZE == self.read_idx
    }

    pub fn free_space(&self) -> usize {
        if self.is_empty() {
            SIZE
        } else if self.write_idx < self.read_idx {
            self.read_idx - self.write_idx - 1
        } else {
            SIZE - (self.write_idx - self.read_idx) - 2
        }
    }

    pub fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else if self.write_idx < self.read_idx {
            SIZE - (self.read_idx - self.write_idx)
        } else {
            self.write_idx - self.read_idx
        }
    }

    pub fn clear(&mut self) {
        self.read_idx = 0;
        self.write_idx = 0;
    }
}

impl<const SIZE: usize> RingBuf<u8, SIZE> {
    pub fn source_from<R: Read>(&mut self, reader: &mut R) -> Result<usize> {
        if self.is_full() {
            return Ok(0);
        }

        let mut n = 0usize;
        if self.write_idx < self.read_idx {
            n = reader.read(&mut self.data[self.write_idx..self.read_idx])?;
        } else {
            n += reader.read(&mut self.data[self.write_idx..])?;
            n += reader
                .read(&mut self.data[0..self.read_idx])
                .unwrap_or_default()
        }

        self.write_idx = (self.write_idx + n) % SIZE;
        Ok(n)
    }

    pub fn sink_into<W: Write>(&mut self, writer: &mut W) -> Result<usize> {
        if self.is_empty() {
            return Ok(0);
        }

        let mut n = 0usize;
        if self.read_idx < self.write_idx {
            n = writer.write(&self.data[self.read_idx..self.write_idx])?;
        } else {
            n += writer.write(&self.data[self.read_idx..])?;
            n += writer
                .write(&self.data[0..self.write_idx])
                .unwrap_or_default()
        }

        self.read_idx = (self.read_idx + n) % SIZE;
        Ok(n)
    }
}

impl<const SIZE: usize> Write for RingBuf<u8, SIZE> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = self.free_space().min(buf.len());

        let first_chunk_len = SIZE - self.write_idx;
        if first_chunk_len >= len {
            self.data[self.write_idx..(self.write_idx + len)].copy_from_slice(buf);
        } else {
            self.data[self.write_idx..].copy_from_slice(&buf[..first_chunk_len]);
            self.data[0..(len - first_chunk_len)].copy_from_slice(&buf[first_chunk_len..]);
        }

        self.write_idx = (self.write_idx + len) % SIZE;
        Ok(len)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<const SIZE: usize> Read for RingBuf<u8, SIZE> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = self.len().min(buf.len());

        let first_chunk_len = SIZE - self.read_idx;
        if first_chunk_len >= len {
            buf[0..len].copy_from_slice(&self.data[self.read_idx..(self.read_idx + len)]);
        } else {
            buf[..first_chunk_len].copy_from_slice(&self.data[self.read_idx..]);
            buf[first_chunk_len..len].copy_from_slice(&self.data[..len - first_chunk_len]);
        }

        self.read_idx = (self.read_idx + len) % SIZE;

        Ok(len)
    }
}
