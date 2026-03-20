use std::{fs::File, io::{BufRead, BufReader, Result}, path::Path};

pub struct LineReader<R: BufRead> {
    reader: R,
    buf: String,
    finished: bool,
}

impl LineReader<BufReader<File>> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        Ok(Self {
            reader,
            buf: String::new(),
            finished: false,
        })
    }
}

impl<R: BufRead> LineReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: String::new(),
            finished: false,
        }
    }
}

impl<R: BufRead> Iterator for LineReader<R> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        loop {
            self.buf.clear();

            match self.reader.read_line(&mut self.buf) {
                Ok(0) => {
                    self.finished = true;
                    return None;
                }
                Ok(_) => {
                    if self.buf.starts_with('#') {
                        continue;
                    }

                    if self.buf.ends_with('\n') {
                        self.buf.pop();
                        if self.buf.ends_with('\r') {
                            self.buf.pop();
                        }
                    }

                    return Some(Ok(self.buf.clone()));
                }
                Err(e) => {
                    self.finished = true;
                    return Some(Err(e));
                }
            }
        }
    }
}