use std::path::{Path, PathBuf};
use std::{fs, io};
use super::hash::Hash;
use super::hashspool::HashSpool;


pub struct HashStore {
    dir: PathBuf,
}

impl HashStore {
    pub fn create(dir: &Path) -> io::Result<HashStore>
    {
        match fs::create_dir(dir) {
            Ok(()) => {}
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::AlreadyExists => {
                        /* Fine, no problem. */
                    }
                    _ => {
                        return Err(e)
                    }
                }
            }
        }
        HashStore::open(dir)
    }

    pub fn open(dir: &Path) -> io::Result<HashStore>
    {
        match fs::read_dir(dir) {
            Err(e) => Err(e),
            Ok(_) => Ok(HashStore { dir: dir.to_owned() }),
        }
    }

    pub fn hash_inserter(&self) -> io::Result<HashInserter>
    {
        HashInserter::init(self.dir.as_path())
    }
}


pub struct HashInserter<'a> {
    dir: &'a Path,
    inpath: PathBuf,
    spool: HashSpool,
}

impl<'a> HashInserter<'a> {
    fn init(dir: &'a Path) -> io::Result<HashInserter>
    {
        use super::unival::UniqueValue;

        let mut pb = PathBuf::new();
        pb.push(dir);
        pb.push(format!("in.{}", try!(UniqueValue::generate()).encoded()));

        let spool = try!(HashSpool::create(pb.as_path()));

        Ok(HashInserter {
            dir: dir,
            inpath: pb,
            spool: spool,
        })
    }

    pub fn commit(self) -> io::Result<Hash>
    {
        let (hash, _) = try!(self.spool.finish());

        let mut outpath = PathBuf::new();
        outpath.push(self.dir);
        outpath.push(hash.encoded());

        try!(fs::rename(self.inpath.as_path(), outpath.as_path()));

        Ok(hash)
    }
}

impl<'a> io::Write for HashInserter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>
    {
        self.spool.write(buf)
    }

    fn flush(&mut self) -> io::Result<()>
    {
        self.spool.flush()
    }
}
