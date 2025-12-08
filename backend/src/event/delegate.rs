use std::any::Any;
use std::fmt;
use std::ops::AddAssign;
use std::sync::{Arc, Mutex};

pub struct Delegate{
    listeners: Vec<Arc<Mutex<dyn FnMut(Arc<dyn Any + Send + Sync>)->() + Send + Sync>>>
}

impl Delegate{
    pub fn new() -> Self{
        Delegate{listeners:Vec::new()}
    }

    // 接受一个实现了 EventListener Trait 的对象，并将其安全地存储起来。
    fn add_listener(&mut self, listener: Arc<Mutex<dyn FnMut(Arc<dyn Any + Send + Sync>)->() + Send + Sync>>) ->() {
        self.listeners.push(listener);
    }

    pub fn trigger_event(&mut self, data: Arc<dyn Any + Send + Sync>) ->() {
        for listener_arc_mutex in &mut self.listeners {
            if let Ok(mut listener) = listener_arc_mutex.lock() {
                // 克隆 Arc，将克隆的所有权副本传递给闭包
                listener(Arc::clone(&data));
            }
        }
    }
}

// 手动实现 Debug Trait
impl fmt::Debug for Delegate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Delegate")
            .field("listeners", &format!("[{} listeners]", self.listeners.len()))
            .finish()
    }
}

// 实现 add_assign 方法
impl AddAssign<Arc<Mutex<dyn FnMut(Arc<dyn Any + Send + Sync>)->() + Send + Sync>>> for Delegate {

    fn add_assign(&mut self, rhs: Arc<Mutex<dyn FnMut(Arc<dyn Any + Send + Sync>)->() + Send + Sync>>) {
        self.listeners.push(rhs);
    }
}