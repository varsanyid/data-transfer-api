extern crate fs2;
extern crate boolinator;

use std::path::Path;
use std::io::Result;
use std::fs::File;
use fs2::FileExt;
use boolinator::Boolinator;
use std::io::{Error, ErrorKind};

#[derive(PartialEq, Debug)]
enum Operation {
    MOVE,
    COPY,
    MoveAndRemove,
}

#[derive(Debug)]
struct DataTransfer<'a> {
    steps: Vec<DataTransferStep<'a>>
}

#[derive(PartialEq, Debug)]
struct DataTransferStep<'a> {
    from: &'a Path,
    to: &'a Path,
    operation: Operation,
}

trait DataTransferRunner {
    fn run(&self) -> Result<u64>;
    fn validate(&self) -> Result<bool>;
    fn lock(&self) -> Result<bool>;
    fn unlock(&self) -> Option<&Self>;
}

impl<'a> DataTransferRunner for DataTransfer<'a> {
    fn run(&self) -> Result<u64> {
        if self.validate()? {
            let copy_results = self.steps.iter().fold(0, |acc, step| {
                match &step.operation {
                    Operation::COPY => acc + std::fs::copy(step.from, step.to).unwrap(),
                    _ => { unimplemented!("not there yet")}
                }
            });
            self.unlock();
            return Ok(copy_results);
        }
        let error = Error::new(ErrorKind::NotFound, "Files not found");
        Err(error)
    }

    fn validate(&self) -> Result<bool> {
        Ok(self.steps.iter().all(|step| step.from.exists()))
    }

    fn lock(&self) -> Result<bool> {
        let is_lock_acquired = self.steps.iter().all(|step| {
            let lock = File::open(step.from).unwrap().lock_exclusive().is_ok();
            lock
        });
        Ok(is_lock_acquired)
    }

    fn unlock(&self) -> Option<&Self> {
        let unlocked = self.steps.iter().all(|step| {
            let is_unlocked = File::open(step.from).unwrap().unlock().is_ok();
            is_unlocked
        });
        unlocked.as_some(&self)
    }
}

impl PartialEq for DataTransfer<'_> {
    fn eq(&self, other: &Self) -> bool {
        &self.steps == &other.steps
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn build_test_data<'a>() -> DataTransfer<'a> {
        let transfer_step = DataTransferStep {
            from: Path::new("C:\\test.txt"),
            to: Path::new("C:\\test"),
            operation: Operation::COPY
        };
        let transfer = DataTransfer {
            steps: vec![transfer_step]
        };
        transfer
    }

    #[test]
    fn assert_file_exists() {
        let transfer = build_test_data();
        assert_eq!(transfer.validate().unwrap(), true)
    }

    #[test]
    fn assert_lock_created() {
        let transfer = build_test_data();
        assert_eq!(transfer.lock().unwrap(), true);
    }

    #[test]
    fn assert_unlock_successful() {
        let transfer = build_test_data();
        transfer.lock().unwrap();
        assert_eq!(transfer.unlock().unwrap(), &transfer)
    }
}


fn main() {}
