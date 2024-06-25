use std::cell::RefCell;
use std::future::Future;

use super::executor::LocalPool;
use super::executor::LocalSpawner;
use super::executor::ThreadNotifyRef;
use futures::task::LocalSpawnExt;
use futures::task::SpawnError;
use once_cell::unsync::Lazy;

thread_local! {
    static LOCAL_POOL: Lazy<RefCell<LocalPool>> = Lazy::default();
    static SPAWNER: Lazy<LocalSpawner> = Lazy::new(|| LOCAL_POOL.with(|ex| ex.borrow().spawner()));
    static FUTURES_COUNT: Lazy<RefCell<usize>> = Lazy::default();
}

pub struct LocalRuntime;

impl LocalRuntime {
    pub fn futures_count() -> usize {
        Self::count()
    }

    pub fn queue_future(future: impl Future<Output = ()> + 'static) -> Result<(), SpawnError> {
        Self::increment();
        SPAWNER.with(move |ls| ls.spawn_local(async move {
            future.await;
            Self::decrement();
        }))
    }

    pub fn run_until_stalled(thread_notify: ThreadNotifyRef) -> bool {
        LOCAL_POOL.with(move |lp| lp.borrow_mut().run_until_stalled(thread_notify));
        if Self::count() == 0 {
            true
        } else {
            false
        }
    }

    fn count() -> usize {
        FUTURES_COUNT.with(|c| *c.borrow_mut())
    }

    fn increment() {
        FUTURES_COUNT.with(|c| *c.borrow_mut() += 1);
    }

    fn decrement() {
        FUTURES_COUNT.with(|c| *c.borrow_mut() -= 1);
    }
}