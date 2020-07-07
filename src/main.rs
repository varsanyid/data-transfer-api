mod locker;

extern crate fs2;

use std::path::Path;
use std::io::Result;
use std::fs::File;
use fs2::FileExt;
use std::io::{Error, ErrorKind};
use locker::*;

#[derive(PartialEq, Debug)]
enum Operation {
    MOVE,
    COPY,
    MoveAndRemove,
}

#[derive(Debug)]
pub struct DataTransfer<'a> {
    steps: Vec<DataTransferStep<'a>>
}

#[derive(PartialEq, Debug)]
pub struct DataTransferStep<'a> {
    from: &'a Path,
    to: &'a Path,
    operation: Operation,
}

trait DataTransferRunner {
    fn run(&self) -> Result<u64>;
    fn validate(&self) -> Result<bool>;
}

impl<'a> DataTransferRunner for DataTransfer<'a> {
    fn run(&self) -> Result<u64> {
        if self.validate()? {
            let copy_results = self.steps.iter().fold(0, |acc, step| {
                match &step.operation {
                    Operation::COPY => acc + std::fs::copy(step.from, step.to).unwrap(),
                    _ => { unimplemented!("not there yet") }
                }
            });
            return Ok(copy_results);
        }
        let error = Error::new(ErrorKind::NotFound, "Files not found");
        Err(error)
    }

    fn validate(&self) -> Result<bool> {
        Ok(self.steps.iter().all(|step| step.from.exists()))
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
    use crate::locker::locker::with_lock;

    fn build_test_data<'a>() -> DataTransfer<'a> {
        let transfer_step = DataTransferStep {
            from: Path::new("C:\\test.txt"),
            to: Path::new("C:\\test\\test.txt"),
            operation: Operation::COPY,
        };
        let transfer = DataTransfer {
            steps: vec![transfer_step]
        };
        transfer
    }

    #[test]
    fn assert_file_exists() {
        let transfer = build_test_data();
        assert_eq!(transfer.validate(), true)
    }

    #[test]
    fn assert_run_copy() {
        let transfer = build_test_data();
        let _copy = with_lock(&transfer);
        let is_successful = transfer.steps.iter().all(|step| step.to.exists());
        assert!(is_successful)
    }
}


fn main() {}
