use std::{collections::HashMap, error::Error, io::Write, path::Path};

use bitcode::Encode;
use flate2::{Compression, Crc};
use log::debug;
use serde::Deserialize;
use toml::Table;


type LoaderFn = fn(Table, bool, &Path) -> (Vec<u8>, u32);

pub trait ToBytes {
    fn bytes(self) -> Vec<u8>;
}

impl<T> ToBytes for T where T: Encode {
    fn bytes(self) -> Vec<u8> {
        bitcode::encode(&self)
    }
}


pub trait SourceLoader: for<'a> Deserialize<'a> + 'static {
    type Output: ToBytes;
    type Error: Into<Box<dyn Error + Send + Sync>>;

    fn load(&self, path: &Path) -> Result<Self::Output, Self::Error>;
}

pub struct SourceLoaders {
    loaders: HashMap<String, LoaderFn>,
}

impl SourceLoaders {
    pub fn new() -> Self {
        Self {
            loaders: HashMap::default(),
        }
    }

    pub fn add<L: SourceLoader>(&mut self, kind: &str) {
        debug!("added source loader `{}` for `{}`", std::any::type_name::<L>(), kind);

        let f = |options: Table, compress: bool, path: &Path| {
            let value: L = options.try_into().unwrap();
            let result: Result<L::Output, L::Error> = value.load(path);
            let result = match result {
                Ok(result) => result,
                Err(e) => {
                    let error: Box<dyn Error + Send + Sync> = e.into();
                    panic!("{}", error);
                },
            };
            let bytes = result.bytes();
            let mut crc = Crc::new();
            crc.update(&bytes);
            // let bitcoded = bitcode::encode(&result);
            let data = match compress {
                true => {
                    let mut data = Vec::new();
                    {
                        let mut encoder =
                            flate2::write::DeflateEncoder::new(&mut data, Compression::best());
                        encoder.write(&bytes).unwrap();
                        encoder.flush().unwrap();
                    }
                    data
                }
                false => bytes,
            };
            (data, crc.sum())
        };

        self.loaders.insert(kind.into(), f);
    }

    pub fn load(&self, kind: String, options: Table, compress: bool, path: &Path) -> (Vec<u8>, u32) {
        let loader = self.loaders.get(&kind.to_lowercase()).unwrap();
        debug!("loading `{}` with `{}`", path.to_string_lossy(), kind);
        loader(options, compress, path)
    }
}
