use std::any::Any;
use std::time::{Duration, SystemTime};
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub enum CBTimesMethod{
    ONCE,
    Multi,
    Fixed(u8)
}

pub struct Timer<T : Any> {
    now: SystemTime,
    is_running: bool,
    cb_last_step_time: SystemTime,
    cb_duration: Option<Duration>,
    cb_params: Option<Arc<T>>,
    cb: Box<dyn FnMut(Option<Arc<T>>)->()>,
    cb_times_method: CBTimesMethod,
}

impl<T> Timer<T> where T : Any {
    pub fn new(cb_duration:Option<Duration>, cb_params: Option<Arc<T>>, cb: Box<dyn FnMut(Option<Arc<T>>)->()>, cb_times_method: CBTimesMethod)-> Self {
        // SystemTime 实现了 Copy, Timer 的 cb_last_time 和 now是两个独立的副本
        let now:SystemTime = SystemTime::now();
        Timer{now, is_running: false, cb_last_step_time: now, cb_duration, cb_params, cb, cb_times_method}
    }

    // 区块链上无法设置 定时触发器，需要用户请求触发 或 时间预言机触发
    pub fn update_timer(&mut self) ->() {
        if !self.is_running {return;}

        if let Some(cb_duration) = self.cb_duration {
            let now:SystemTime = SystemTime::now();
            let duration:Duration;

            match now.duration_since(self.now){
                Ok(res) => {
                    duration = res;
                }
                Err(error) => {
                    eprintln!("无法计算持续时间，时钟错误: {:?}", error);
                    return;
                }
            }

            // 理应触发次数
            let cb_times:u8= (duration.as_secs() / cb_duration.as_secs()) as u8;

            if(duration > cb_duration){
                //如果cb_times_method为ONCE，则只触发一次，否则触发 cb_times 次
                match self.cb_times_method{
                    CBTimesMethod::ONCE => {
                        (self.cb)(self.cb_params.clone());
                    }
                    CBTimesMethod::Fixed(fixed_times)=>{
                        let actual_times = if(fixed_times>cb_times){cb_times}else{cb_times};
                        for _ in 0..actual_times{
                            (self.cb)(self.cb_params.clone());
                        }
                    }
                    _=>{
                        for _ in 0..cb_times{
                            (self.cb)(self.cb_params.clone());
                        }
                    }
                }

                //更新最新的触发时间步
                self.cb_last_step_time += cb_duration * cb_times as u32;
            }

            // 更新当前时间
            self.now = now;
        }
    }

    pub fn get_now(&self) -> SystemTime {
        self.now
    }

    pub fn get_is_running(&self) -> bool{
        self.is_running
    }

    pub fn set_is_running(&mut self, is_running: bool) {
        self.is_running = is_running;
    }
}

// 手动实现 Debug trait
impl<T> fmt::Debug for Timer<T> where T : Any + fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Timer")
            .field("now", &self.now)
            .field("is_running", &self.is_running)
            .field("cb_last_step_time", &self.cb_last_step_time)
            .field("cb_duration", &self.cb_duration)
            .field("cb", &"Box[FnMut cb]")
            .field("cb_times_method", &self.cb_times_method)
            .finish() // 结束构建并返回 Result
    }
}
