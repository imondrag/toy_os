/// The status of a context - used for scheduling
/// See `syscall::process::waitpid` and the `sync` module for examples of usage
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Runnable,
    Blocked,
    Exited(usize),
}

/// A context, which identifies either a process or a thread
#[derive(Debug)]
pub struct Context {
    /// The ID of this context
    pub id: ContextId,
    /// Status of context
    pub status: Status,
    pub status_reason: &'static str,
    /// Context running or not
    pub running: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ContextId(usize);
