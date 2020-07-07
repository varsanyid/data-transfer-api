pub mod locker {
    use crate::{DataTransfer, DataTransferStep, DataTransferRunner};
    use std::io::{Result, Error};
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

        pub(super) fn lock(steps: &Vec<DataTransferStep>) -> Result<bool> {
            let is_lock_acquired = steps.iter().all(|step| {
                let lock = File::open(step.from).unwrap().lock_exclusive().is_ok();
                lock
            });
            Ok(is_lock_acquired)
        }

        pub(super) fn unlock(steps: &Vec<DataTransferStep>) -> Result<bool> {
            let unlocked = steps.iter().all(|step| {
                let is_unlocked = File::open(step.from).unwrap().unlock().is_ok();
                is_unlocked
            });
            Ok(unlocked)
        }
    }
}
