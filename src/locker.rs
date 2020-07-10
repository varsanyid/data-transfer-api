use crate::{DataTransfer, DataTransferStep, DataTransferRunner};
use std::io::{Result};
use std::fs::File;
use fs2::FileExt;
use std::vec::Vec;

pub fn with_lock<'a>(transfer: &DataTransfer) -> Result<bool> {
    let result = internal::lock(&transfer.steps)
        .and_then(|_| transfer.run())
        .and_then(|_| internal::unlock(&transfer.steps));
    result
}

mod internal {
    use super::*;
    use std::io::ErrorKind;

    pub(super) fn lock(steps: &Vec<DataTransferStep>) -> Result<bool> {
        let is_lock_acquired = steps.iter().all(|step| {
            let lock = File::open(step.from).unwrap().lock_exclusive().is_ok();
            lock
        });
        Ok(is_lock_acquired)
    }

    pub(super) fn unlock(steps: &Vec<DataTransferStep>) -> Result<bool> {
        let unlocked = steps.iter().all(|step| {
            match File::open(step.from) {
                Ok(x) => x.unlock().is_ok(),
                Err(err) => err.kind() == ErrorKind::NotFound
            }
        });
        Ok(unlocked)
    }
}

