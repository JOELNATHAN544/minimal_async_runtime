use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
    thread,
    sync::{Arc, Mutex},
    collections::VecDeque,
};

// A simple future that yields control back to the runtime
struct YieldNow {
    yielded: bool,
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn yield_now() -> YieldNow {
    YieldNow { yielded: false }
}

// A minimal async runtime
struct MiniRuntime {
    tasks: Arc<Mutex<VecDeque<Pin<Box<dyn Future<Output = ()> + Send>>>>>,
}

impl MiniRuntime {
    fn new() -> Self {
        MiniRuntime {
            tasks: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + 'static + Send,
    {
        self.tasks.lock().unwrap().push_back(Box::pin(future));
    }

    fn block_on<F: Future>(&mut self, future: F) -> F::Output {
        let mut future = Box::pin(future);
        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        
        loop {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(result) => return result,
                Poll::Pending => {
                    // Process any spawned tasks
                    while let Some(mut task) = self.tasks.lock().unwrap().pop_front() {
                        let _ = task.as_mut().poll(&mut cx);
                    }
                    // Yield to the OS
                    thread::yield_now();
                }
            }
        }
    }
}

async fn task_one() {
    println!("task one: start");
    thread::sleep(Duration::from_secs(1));
    println!("task one: done");
}

async fn task_two() {
    println!("task two: start");
    thread::sleep(Duration::from_secs(2));
    println!("task two: done");
}

fn main() {
    let rt = Arc::new(Mutex::new(MiniRuntime::new()));
    
    // Spawn tasks first
    {
        let rt = rt.lock().unwrap();
        rt.spawn(task_one());
        rt.spawn(task_two());
    }
    
    // Then run the main future
    let mut rt_guard = rt.lock().unwrap();
    rt_guard.block_on(async {
        println!("Runtime started...");
        yield_now().await;
        thread::sleep(Duration::from_secs(3));
    });
}