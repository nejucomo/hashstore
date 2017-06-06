use std::path::Path;
use std::{fs, io};
use hash::{Hash, Hasher};


pub struct HashSpool {
    f: fs::File,
    hasher: Hasher,
}

impl HashSpool {
    pub fn create(path: &Path) -> io::Result<HashSpool> {
        // FIXME: Does this OpenOptions ensure seekable?
        let f = try!(fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path));

        Ok(HashSpool {
            f: f,
            hasher: Hasher::new(),
        })
    }

    pub fn finish(mut self) -> io::Result<(Hash, fs::File)> {
        use std::io::{Seek, SeekFrom, Write};

        try!(self.flush());

        let hash = self.hasher.finalize();
        try!(self.f.seek(SeekFrom::Start(0)));

        Ok((hash, self.f))
    }
}

impl io::Write for HashSpool {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = try!(self.f.write(buf));
        self.hasher.update(&buf[0..s]);
        Ok(s)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.f.flush()
    }
}


#[cfg(test)]
mod tests {
    tests_with_fs! {
        empty_spool |testpath| {
            use hashspool::HashSpool;
            use EMPTY_HASH;

            let spool = res_unwrap!(HashSpool::create(testpath));
            let (hash, _) = res_unwrap!(spool.finish());
            assert_eq!(EMPTY_HASH, hash.encoded());
        }
    }
}
