#[macro_export]
macro_rules! test_path {

    () => {
        {
            use std::env;

            let mut pb = env::temp_dir();
            for part in module_path!().split("::") {
                pb.push(part);
            }

            pb
        }
    }
}


#[macro_export]
macro_rules! test_with_fs {

    ($name:ident $closure:expr) => {
        #[test]
        fn $name () {
            let testpathbuf = {
                use std::{env,fs};

                let mut it = module_path!().split("::");
                let mut pb = env::temp_dir();

                {
                    let mut first = String::from("test-");
                    first.push_str(it.next().unwrap());
                    pb.push(first);

                    fs::remove_dir_all(pb.as_path()).unwrap();
                }

                for part in it {
                    pb.push(part);
                }
                fs::create_dir_all(pb.as_path()).unwrap();

                pb.push(stringify!($name));

                pb
            };

            let cl = $closure;

            cl(testpathbuf.as_path());
        }
    }
}

