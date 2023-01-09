#![feature(asm)]
use chrono::{DateTime, Utc};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use humansize::{make_format, DECIMAL};
use std::arch::asm;
use std::arch::global_asm;
use std::cell::Cell;
use std::cmp;
use std::cmp::{max, min};
use std::usize;

// This global variable tracks the highest point of the stack
thread_local!(static STACK_END: Cell<usize> = Cell::new(usize::MAX));

#[macro_export]
macro_rules! stack_ptr {
    () => {{
        // Grab a copy of the stack pointer
        let x: usize;
        unsafe {
            // llvm_asm!("mov %rsp, $0" : "=r"(x) ::: "volatile");
            asm!("mov {0}, rsp", out(reg) x);
        }
        x
    }};
}

/// Saves the current position of the stack. Any function
/// being profiled must call this macro.
#[macro_export]
macro_rules! tick {
    () => {{
        // Save the current stack pointer in STACK_END
        let stack_end = stack_ptr!();
        STACK_END.with(|c| {
            // Since the stack grows down, the "tallest"
            // stack must have the least pointer value
            let best = cmp::min(c.get(), stack_end);
            c.set(best);
        });
    }};
}

pub fn tick() {
    tick!()
}

pub fn stack_ptr() -> usize {
    stack_ptr!()
}

pub fn print_pointer(original_rsp: usize, x1: &str) -> usize {
    tick!();
    let new_rsp = stack_ptr!();
    let formatter = make_format(DECIMAL);

    let diff = original_rsp.checked_sub(new_rsp).unwrap_or(0);
    println!(
        "Stack pointer: {} {} ({}). Difference: {} ({})",
        x1,
        new_rsp,
        formatter(new_rsp),
        diff,
        formatter(diff)
    );
    diff
}

/// Runs the given callback, and returns its maximum stack usage
/// as reported by the `tick!()` macro.
pub fn measure<T, F: FnOnce() -> T>(callback: F) -> (T, usize) {
    STACK_END.with(|c| c.set(usize::MAX));
    let stack_start = stack_ptr!();
    let r = callback();
    let stack_end = STACK_END.with(|c| c.get());
    if stack_start < stack_end {
        panic!("tick!() was never called");
    }
    (r, stack_start - stack_end)
}

/// Example recursive function
pub fn fibonacci(n: i64) -> i64 {
    tick!();
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub struct StackInfo {
    pub original_pointer: usize,
    pub current_pointer: usize,
    pub max_pointer: usize,
    pub iteration: usize,
    pub total_iterations: usize,
    pub max_iteration: usize,
    pub depth: usize,
    pub max_depth: usize,
    pub iteration_at_depth: usize,
    pub total_iterations_at_depth: usize,
    pub formatter: Box<dyn Fn(usize) -> String>,
    pub start_time: DateTime<Utc>,
}

impl StackInfo {
    pub fn new() -> StackInfo {
        tick();
        let p = stack_ptr();
        StackInfo {
            original_pointer: p,
            current_pointer: p,
            max_pointer: p,
            iteration: 0,
            total_iterations: 0,
            max_iteration: 0,
            depth: 0,
            max_depth: 0,
            iteration_at_depth: 0,
            total_iterations_at_depth: 0,
            formatter: Box::new(make_format(DECIMAL)),
            start_time: Utc::now(),
        }
    }

    pub fn show(&self, old_pointer: usize, extra: &str) {
        let diff = old_pointer.checked_sub(self.current_pointer).unwrap_or(0);
        let total_diff = self
            .original_pointer
            .checked_sub(self.current_pointer)
            .unwrap_or(0);
        let max_diff = self
            .original_pointer
            .checked_sub(self.max_pointer)
            .unwrap_or(0);
        let duration = (Utc::now() - self.start_time) / self.total_iterations as i32;
        let nice = HumanTime::from(duration);
        println!(
            "{} / {}. Stack pointer: {} ({}). Difference: {} ({}). Largest stack: {} ({}). {} per iteration. {}",
            self.iteration,
            self.total_iterations,
            total_diff,
            (self.formatter)(total_diff),
            diff,
            (self.formatter)(diff),
            max_diff,
            (self.formatter)(max_diff),
            nice.to_text_en(Accuracy::Precise, Tense::Present),
            extra
        );
    }

    pub fn update(&mut self) -> usize {
        tick!();
        let new_pointer = stack_ptr();
        let old = self.current_pointer;
        self.current_pointer = new_pointer;
        self.iteration += 1;
        self.max_iteration = max(self.max_iteration, self.iteration);
        self.max_pointer = min(self.current_pointer, self.current_pointer);
        self.total_iterations += 1;
        old
    }
    pub fn update_and_show(&mut self) {
        let old = self.update();
        self.show(old, "")
    }

    pub fn update_and_show_every(&mut self, every: usize, extra: impl Fn() -> String) {
        let old = self.update();
        if self.total_iterations % every == 0 {
            self.show(old, &extra())
        }
    }

    pub fn update_depth_iterations(&mut self, depth: usize, iteration_at_depth: usize, total_iterations_at_depth: usize) {
        self.iteration += 1;
        self.max_depth = max(self.max_depth, depth);
        self.depth = depth;
        self.iteration_at_depth = iteration_at_depth;
        self.total_iterations_at_depth = total_iterations_at_depth;
    }
}
