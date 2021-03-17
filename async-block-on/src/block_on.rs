
use core::future::Future;
use core::ptr::null_mut;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

unsafe fn waker_clone(p: *const ()) -> RawWaker {
    RawWaker::new(p, &VTABLE)
}

unsafe fn waker_nop(_p: *const ()) {}

static VTABLE: RawWakerVTable = RawWakerVTable::new(waker_clone, waker_nop, waker_nop, waker_nop);

pub fn block_on<F: Future>(future: F) -> F::Output {
    futures::pin_mut!(future);
    let waker = &unsafe { Waker::from_raw(RawWaker::new(null_mut(), &VTABLE)) };
    let mut cx = Context::from_waker(waker);
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut cx) {
            return output;
        }
    }
}