//! Debounce utility to prevent rapid execution of heavy tasks (e.g., searching).

use fltk::app;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicUsize, Ordering};
//use std::sync::{Arc, Mutex};

// 全局计数器，用来记录防抖的“代号”
static DEBOUNCE_COUNTER: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(0));

pub struct Debouncer;

impl Debouncer {
    /// 延迟运行。如果在这段延迟时间内再次调用了 `run`，上一次的回调将被取消。
    pub fn run<F>(delay: f64, callback: F)
    where
        F: FnOnce() + 'static,
    {
        // 每次调用，计数器 +1，当前的值代表这一次调用的唯一代号
        let current_id = DEBOUNCE_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
        
        // 包装 callback，使其可以在定时器闭包里被消费
        let mut callback = Some(callback);
        
        app::add_timeout3(delay, move |_| {
            // 检查这 delay 秒内，是否又发生了新的防抖调用
            // 如果计数器的值 == 当前的 ID，说明期间没有新调用，可以安全执行
            if DEBOUNCE_COUNTER.load(Ordering::SeqCst) == current_id {
                if let Some(cb) = callback.take() {
                    cb();
                }
            }
        });
    }
}