use poll_promise::Promise;

use crate::prelude::*;

pub enum IoTaskResult<T: std::marker::Send + 'static> {
    Idle,
    Working(Promise<AppResult<T>>),
    Failed(AppError),
}

impl<T: std::fmt::Debug + std::marker::Send + 'static> IoTaskResult<T> {
    pub fn is_working(&self) -> bool {
        matches!(self, Self::Working(_))
    }

    pub fn try_fetch_from_io_task(&mut self) -> Option<T> {
        if self.is_done() {
            let to_take = std::mem::take(self);
            if let Self::Working(promise) = to_take {
                let result = promise.block_and_take();
                match result {
                    Ok(value) => {
                        *self = IoTaskResult::Idle;
                        return Some(value);
                    }
                    Err(error) => {
                        *self = IoTaskResult::Failed(error);
                        return None;
                    }
                }
            }
        }
        None
    }

    fn is_done(&self) -> bool {
        if let Self::Working(promise) = self {
            if promise.ready().is_some() {
                return true;
            }
        }

        false
    }
}

impl<T: std::fmt::Debug + std::marker::Send + 'static> Default for IoTaskResult<T> {
    fn default() -> Self {
        Self::Idle
    }
}
