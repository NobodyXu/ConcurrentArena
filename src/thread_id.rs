use parking_lot::{lock_api::GetThreadId, RawThreadId};

/// Return a non zero thread id
pub(crate) fn get_thread_id() -> usize {
    RawThreadId::INIT.nonzero_thread_id().get()
}
