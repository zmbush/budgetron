// Copyright 2017 Zachary Bush.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use {
    crate::loading::{
        alliant,
        generic::{Genericize, Transaction},
        logix, mint,
    },
    budgetronlib::error::{BResult, BudgetError},
    csv::Reader,
    log::info,
    serde::de::DeserializeOwned,
    std::{
        cmp::min,
        fmt::Display,
        fs::File,
        io::{self, Read, Seek, Stdin, StdinLock},
        path::Path,
    },
};

fn from_reader<TransactionType, R>(file: &mut R) -> BResult<Vec<Transaction>>
where
    TransactionType: Genericize + DeserializeOwned,
    R: io::Read,
{
    let mut transactions = Vec::new();
    for record in Reader::from_reader(file).deserialize() {
        let record: TransactionType = record?;
        transactions.push(record.genericize()?);
    }
    Ok(transactions)
}

struct StdinSource<'a> {
    buf: Vec<u8>,
    loc: usize,
    stdin: StdinLock<'a>,
}

impl<'a> StdinSource<'a> {
    fn new(stdin: &'a Stdin) -> StdinSource<'a> {
        StdinSource {
            buf: Vec::new(),
            loc: 0,
            stdin: stdin.lock(),
        }
    }
}

enum Source<'a> {
    File(File),
    Stdin(StdinSource<'a>),
}

impl<'a> Seek for Source<'a> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match *self {
            Source::File(ref mut f) => f.seek(pos),
            Source::Stdin(ref mut source) => match pos {
                io::SeekFrom::Start(loc) => {
                    source.loc = loc as usize;
                    Ok(source.loc as u64)
                }
                io::SeekFrom::Current(diff) => {
                    if diff >= 0 {
                        source.loc += diff as usize;
                    } else {
                        source.loc -= (-diff) as usize;
                    }
                    if source.loc >= source.buf.len() {
                        Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "Tried to seek past internal buffer",
                        ))
                    } else {
                        Ok(source.loc as u64)
                    }
                }
                io::SeekFrom::End(_) => Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Stdin has no end",
                )),
            },
        }
    }
}

impl<'a> Read for Source<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Source::File(ref mut f) => f.read(buf),
            Source::Stdin(ref mut source) => {
                if source.loc >= source.buf.len() {
                    let ret = source.stdin.read(buf);
                    if let Ok(size) = ret {
                        source.buf.extend_from_slice(&buf[..size]);
                        source.loc += size;
                        Ok(size)
                    } else {
                        ret
                    }
                } else {
                    let len = buf.len();
                    let start = source.loc;
                    let end = min(start + len, source.buf.len());
                    let readlen = end - start;
                    buf[..readlen].copy_from_slice(&source.buf[start..end]);
                    source.loc = end;
                    Ok(readlen)
                }
            }
        }
    }
}

fn from_file_inferred<P: AsRef<Path> + Copy>(filename: P) -> BResult<Vec<Transaction>> {
    // If the file doesn't exist. Don't bother.
    let stdin = io::stdin();
    let mut reader = match filename.as_ref().to_str() {
        Some("-") => Source::Stdin(StdinSource::new(&stdin)),
        _ => Source::File(File::open(filename)?),
    };

    let mut errors = Vec::new();

    macro_rules! parse_exports {
        ($($type:path),*) => ($(match from_reader::<$type, _>(&mut reader) {
            Ok(result) => return Ok(result),
            Err(e) => {
                errors.push(e);
                reader.seek(io::SeekFrom::Start(0))?;
            }
        })*)
    }
    parse_exports!(
        Transaction,
        mint::MintExport,
        logix::LogixExport,
        alliant::AlliantExport
    );
    Err(BudgetError::Multi(errors))
}

pub fn load_from_files<P: AsRef<Path> + Display, Files: Iterator<Item = P>>(
    filenames: Files,
) -> BResult<Vec<Transaction>> {
    let mut transactions = Vec::new();
    for filename in filenames {
        info!("Opening file: {}", filename);
        transactions.append(&mut from_file_inferred(&filename)?);
    }

    transactions.sort_by(|a, b| a.date.cmp(&b.date));

    Ok(transactions)
}
