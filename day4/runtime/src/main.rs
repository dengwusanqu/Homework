use std::{
    alloc::System,
    future::{self, Future},
    cell::RefCell,
    task::{Context, Poll, RawWaker, RawWakerVTable, Wake, Waker},
    sync::{Arc, Condvar, Mutex},
};

use futures::future::BoxFuture;
use scoped_tls::scoped_thread_local;
use std::collections::VecDeque;

fn dummy_waker() -> Waker {
    static DATA: () = ();
    unsafe { Waker::from_raw(RawWaker::new(&DATA, &VTABLE))}
}
const VTABLE: RawWakerVTable = RawWakerVTable::new(vtable_clone, vtable_wake, vtable_wake_by_ref, vtable_drop);
unsafe fn vtable_clone(_p: *const ()) -> RawWaker {
    RawWaker::new(_p, &VTABLE)
}
unsafe fn vtable_wake(_p: *const ()) {}
unsafe fn vtable_wake_by_ref(_p: *const ()) {}
unsafe fn vtable_drop(_p: *const ()) {}


struct Signal {
    state: Mutex<State>,
    cond: Condvar,
}

enum State {
    Empty,
    Wating,
    Notified,
}

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        self.notify();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.notify();
    }
}

impl Signal {
    fn new() -> Self {
        Self {
            state: Mutex::new(State::Empty),
            cond: Condvar::new(),
        }
    }
    fn wait(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Empty => {
                *state = State::Wating;
                while let State::Wating = *state {
                    state = self.cond.wait(state).unwrap();
                }
            }
            State::Wating => {
                panic!("cannot wait twice");
            }
            State::Notified => {
                *state = State::Empty;
            }
        }
    }
    fn notify(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Empty => {
                *state = State::Notified;
            }
            State::Wating => {
                *state = State::Empty;
                self.cond.notify_one();
            }
            State::Notified => {
                println!("already notified")
            }
        }
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let mut fut = std::pin::pin!(future);
    let signal = Arc::new(Signal::new());
    let waker = Waker::from(signal.clone());

    let mut cx = Context::from_waker(&waker);

    let runnable = Mutex::new(VecDeque::with_capacity(1024));
    SIGNAL.set(&signal, || {
        RUNNABLE.set(&runnable, || loop {
            if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
                return output;
            }
            while let Some(task) = runnable.lock().unwrap().pop_front() {
                let waker = Waker::from(task.clone());
                let mut cx = Context::from_waker(&waker);
                let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
            }
            signal.wait();
        })
    })
}

async fn demo() {
    let (tx, rx) = async_channel::bounded::<()>(1);
    std::thread::spawn(move || {
        tx.send_blocking(()).unwrap();
    });
    let _ = rx.recv().await;
    println!("Happy1");
}

async fn demo1() {
    let (tx, rx) = async_channel::bounded::<()>(1);
    println!("Happy2");
    spawn(demo2(tx));
    println!("Happy3");
    let _ = rx.recv().await;
}

async fn demo2(tx: async_channel::Sender<()>) {
    println!("Happy4");
    tx.send(()).await.unwrap();
}

fn spawn<F: Future<Output = ()> + 'static + Send>(future: F) {
    std::thread::spawn(move || {
        let signal = Arc::new(Signal::new());
        let waker = Waker::from(signal.clone());
        let task = Arc::new(Task {
            future: RefCell::new(Box::pin(future)),
            signal: signal.clone(),
        });
        let mut cx = Context::from_waker(&waker);

        while let Poll::Pending = task.future.borrow_mut().as_mut().poll(&mut cx) {
            RUNNABLE.with(|runnable| {
                runnable.lock().unwrap().push_back(task.clone());
                signal.notify();
            });
        }
    });
}

struct Task {
    future: RefCell<BoxFuture<'static, ()>>,
    signal: Arc<Signal>,
}
unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        RUNNABLE.with(|runnable| {
            runnable.lock().unwrap().push_back(self.clone());
            self.signal.notify();
        })
    }
}

scoped_thread_local!(static RUNNABLE: Mutex<VecDeque<Arc<Task>>>);
scoped_thread_local!(static SIGNAL: Arc<Signal>);

fn main() {
    block_on(demo());
    block_on(demo1());
}