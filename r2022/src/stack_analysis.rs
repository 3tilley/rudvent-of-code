#![feature(asm)]
use humansize::{make_format, DECIMAL};
use std::arch::asm;
use std::arch::global_asm;
use std::cell::Cell;
use std::cmp;
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
