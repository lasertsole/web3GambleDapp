use std::time::{Duration, SystemTime};
use std::fmt;

#[derive(Debug)]
pub enum CBTimesMethod{
    ONCE,
    Multi
}

pub struct Timer {
    now: SystemTime,
    is_running: bool,
    cb_last_step_time: SystemTime,
    cb_duration: Duration,
    cb: &'static mut dyn FnMut()->(),
    cb_times_method: CBTimesMethod,
}

impl Timer {
    pub fn new(cb_duration:Duration, cb: &'static mut dyn FnMut()->(), cb_times_method: CBTimesMethod)-> Self {
        // SystemTime 实现了 Copy, Timer 的 cb_last_time 和 now是两个独立的副本
        let now:SystemTime = SystemTime::now();
        Timer{now, is_running: false, cb_last_step_time: now, cb_duration, cb, cb_times_method}
    }

    // 区块链上无法设置 定时触发器，需要用户请求触发 或 时间预言机触发
    pub fn update_timer(&mut self) ->() {
        if !self.is_running {return;}

        let now:SystemTime = SystemTime::now();
        let duration:Duration = now.duration_since(self.now).unwrap();

        // 理应触发次数
        let cb_times:u32 = (duration.as_secs() / self.cb_duration.as_secs()) as u32;

        if(duration > self.cb_duration){
            //如果cb_times_method为ONCE，则只触发一次，否则触发 cb_times 次
            match self.cb_times_method{
                CBTimesMethod::ONCE => {
                    (self.cb)();
                }
                _=>{
                    for _ in 0..cb_times{
                        (self.cb)();
                    }
                }
            }

            //更新最新的触发时间步
            self.cb_last_step_time += self.cb_duration * cb_times;
        }

        // 更新当前时间
        self.now = now;
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
impl fmt::Debug for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Timer")
            .field("now", &self.now)
            .field("is_running", &self.is_running)
            .field("cb_last_step_time", &self.cb_last_step_time)
            .field("cb_duration", &self.cb_duration)
            .field("cb", &"[FnMut cb]")
            .field("cb_times_method", &self.cb_times_method)
            .finish() // 结束构建并返回 Result
    }
}
