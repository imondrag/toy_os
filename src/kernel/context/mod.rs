//! # Context management
//!
//! For resources on contexts, please consult [wikipedia](https://en.wikipedia.org/wiki/Context_switch) and  [osdev](https://wiki.osdev.org/Context_Switching)

mod context;

pub fn init() {
    // let mut contexts = contexts_mut();
    // let context_lock = contexts
    //     .new_context()
    //     .expect("could not initialize first context");

    // let mut context = context_lock.write();
    // context.status = Status::Runnable;
    // context.running = true;
    // context.cpu_id = Some(crate::cpu_id());
    // CONTEXT_ID.store(context.id, Ordering::SeqCst);
}
