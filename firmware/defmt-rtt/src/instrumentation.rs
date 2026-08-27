//! Records the byte size of every defmt frame written to RTT, so it can be
//! compared against the size a plain `log`/`core::fmt` message would have
//! taken for the same data.
//!
//! Drain the queue from your own task with [`try_pop`].

use core::cell::RefCell;

use critical_section::Mutex;
use heapless::spsc::Queue;

/// Number of frame sizes the queue can hold before entries are dropped.
const QUEUE_CAPACITY: usize = 32;

static FRAME_SIZES: Mutex<RefCell<Queue<usize, QUEUE_CAPACITY>>> =
    Mutex::new(RefCell::new(Queue::new()));

/// Push one frame's byte size onto the queue. Silently drops the entry if
/// the queue is full.
pub(crate) fn record_frame_size(size: usize) {
    critical_section::with(|cs| {
        FRAME_SIZES.borrow(cs).borrow_mut().enqueue(size).ok();
    });
}

/// Pop the oldest recorded frame size, if any.
pub fn try_pop() -> Option<usize> {
    critical_section::with(|cs| FRAME_SIZES.borrow(cs).borrow_mut().dequeue())
}
