use std::any::Any;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt;
use crate::event::delegate::Delegate;

pub struct Bus{
    listeners: Mutex<HashMap<String, Vec<Box<dyn FnMut(Arc<dyn Any + Send + Sync>)->() + Send + Sync>>>>
}

impl Bus {
    pub fn new() -> Self {
        Bus{listeners: Mutex::new(HashMap::new())}
    }

    /// 注册一个特定事件类型 T 的监听器
    pub fn subscribe(& mut self, event: &str, listener: Box<dyn FnMut(Arc<dyn Any + Send + Sync>)->() + Send + Sync>) -> ()
    {
        let mut listeners_guard = self.listeners.lock().unwrap();
        listeners_guard.entry(event.to_string())
            .or_insert_with(Vec::new)
            .push(listener);
    }

    /// 发布一个事件，所有订阅了该事件类型的监听器都会收到
    pub fn publish(&self, event:  &str, data: Arc<dyn Any + Send + Sync>)
    {
        let mut listeners_guard = self.listeners.lock().unwrap();
        if let Some(listener_vec)  = listeners_guard.get_mut(event) {
            for listener in listener_vec {
                listener(Arc::clone(&data));
            }
        }
    }
}

// 修正：为 Bus 结构体实现 Debug Trait
impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 1. 锁定 Mutex 以安全地读取内部数据
        let listeners_guard = self.listeners.lock().unwrap();

        // 2. 获取所有事件类型（HashMap 的键）和总的监听器数量
        let total_listeners: usize = listeners_guard.values().map(|vec| vec.len()).sum();

        // 3. 格式化输出
        f.debug_struct("Bus")
            .field("total_listeners", &total_listeners)
            .finish()
    }
}