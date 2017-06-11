use std::path::{Path, PathBuf};
use std::{fs, io};
use hash::Hash;
use hashspool::HashSpool;


pub struct HashStore {
    dir: PathBuf,
}

impl HashStore {
    pub fn create(dir: &Path) -> io::Result<HashStore> {
        match fs::create_dir(dir) {
            Ok(()) => {}
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::AlreadyExists => {
                        // Fine, no problem.
                    }
                    _ => return Err(e),
                }
            }
        }
        HashStore::open(dir)
    }

    pub fn open(dir: &Path) -> io::Result<HashStore> {
        match fs::read_dir(dir) {
            Err(e) => Err(e),
            Ok(_) => Ok(HashStore { dir: dir.to_owned() }),
        }
    }

    pub fn open_inserter(&self) -> io::Result<HashInserter> {
        HashInserter::init(self.dir.clone())
    }

    pub fn open_reader(&self, hash: &Hash) -> io::Result<fs::File> {
        let mut pb = self.dir.clone();
        pb.push(hash.encoded());
        fs::File::open(pb)
    }

    pub fn has_hash(&self, hash: &Hash) -> io::Result<bool> {
        use std::io::ErrorKind;

        match self.open_reader(hash) {
            Ok(_) => Ok(true),
            Err(ref e) if e.kind() == ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e),
        }
    }
}


pub struct HashInserter {
    dir: PathBuf,
    inpath: PathBuf,
    spool: HashSpool,
}

impl HashInserter {
    fn init(dir: PathBuf) -> io::Result<HashInserter> {
        use unival::UniqueValue;

        let mut pb = PathBuf::new();
        pb.push(dir.as_path());
        pb.push(format!("in.{}", try!(UniqueValue::generate()).encoded()));

        let spool = try!(HashSpool::create(pb.as_path()));

        Ok(HashInserter {
            dir: dir,
            inpath: pb,
            spool: spool,
        })
    }

    pub fn commit(self) -> io::Result<Hash> {
        let (hash, _) = try!(self.spool.finish());

        let mut outpath = PathBuf::new();
        outpath.push(self.dir.as_path());
        outpath.push(hash.encoded());

        try!(fs::rename(self.inpath.as_path(), outpath.as_path()));

        Ok(hash)
    }
}

impl io::Write for HashInserter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.spool.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.spool.flush()
    }
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    tests_with_fs! {
        create_new_dir |path: &Path| {
            use std::{fs, io};
            use hashstore::HashStore;

            let exists_as_dir = |p| {
                match fs::metadata(p) {
                    Ok(md) => {
                        md.is_dir()
                    }
                    Err(e) => {
                        assert_eq!(e.kind(), io::ErrorKind::NotFound);
                        false
                    }
                }
            };

            assert!(!exists_as_dir(path));

            res_unwrap!(HashStore::create(path));

            assert!(exists_as_dir(path));
        };

        open_non_existent_dir |path: &Path| {
            use std::io;
            use hashstore::HashStore;

            let res = HashStore::open(path);

            assert!(res.is_err());
            assert!(res.err().unwrap().kind() == io::ErrorKind::NotFound);
        };

        insert_empty |path: &Path| {
            use std::fs;
            use std::io::Read;
            use hashstore::HashStore;
            use EMPTY_HASH;

            let hs = res_unwrap!(HashStore::create(path));
            let ins = res_unwrap!(hs.open_inserter());
            let hash = res_unwrap!(ins.commit());
            assert_eq!(EMPTY_HASH, hash.encoded());

            let mut pb = path.to_path_buf();
            pb.push(EMPTY_HASH);
            assert!(res_unwrap!(fs::metadata(pb)).is_file());

            let mut f = res_unwrap!(hs.open_reader(&hash));
            let mut contents = String::new();
            let readlen = res_unwrap!(f.read_to_string(&mut contents));
            assert_eq!(0, readlen);
            assert_eq!("", contents);

            assert!(res_unwrap!(hs.has_hash(&hash)));
        }
    }
}
