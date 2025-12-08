use std::time::{Duration, SystemTime};
use std::fmt;

#[derive(Debug)]
pub enum CBTimesMethod{
    ONCE,
    Multi
}

pub struct Timer {
    now: SystemTime,
    cb_last_time_step: SystemTime,
    cb_duration: Duration,
    cb: Box<dyn FnMut()->()>,
    cb_times_method: CBTimesMethod
}

impl Timer {
    fn new(now: SystemTime, cb_duration:Duration, cb: Box<dyn FnMut()->()>, cb_times_method: CBTimesMethod)-> Self {
        // SystemTime 实现了 Copy, Timer 的 cb_last_time 和 now是两个独立的副本
        Timer{now, cb_last_time_step: now, cb_duration, cb, cb_times_method}
    }

    // 区块链上无法设置 定时触发器，需要用户请求触发 或 时间预言机触发
    fn update_timer(&mut self) ->() {
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
            self.cb_last_time_step += self.cb_duration * cb_times;
        }

        // 更新当前时间
        self.now = now;
    }

    fn get_now(&self) -> SystemTime {
        self.now
    }
}

// 手动实现 Debug trait
impl fmt::Debug for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Timer")
            .field("now", &self.now)
            .field("cb_last_time_step", &self.cb_last_time_step)
            .field("cb_duration", &self.cb_duration)
            .field("cb", &"Box<dyn FnMut()->()>")
            .field("cb_times_method", &self.cb_times_method)
            .finish() // 结束构建并返回 Result
    }
}