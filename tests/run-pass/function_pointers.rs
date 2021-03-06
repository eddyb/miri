fn f() -> i32 {
    42
}

fn g(i: i32) -> i32 {
    i*42
}

fn return_fn_ptr() -> fn() -> i32 {
    f
}

fn call_fn_ptr() -> i32 {
    return_fn_ptr()()
}

fn indirect<F: Fn() -> i32>(f: F) -> i32 { f() }
fn indirect_mut<F: FnMut() -> i32>(mut f: F) -> i32 { f() }
fn indirect_once<F: FnOnce() -> i32>(f: F) -> i32 { f() }

fn indirect2<F: Fn(i32) -> i32>(f: F) -> i32 { f(10) }
fn indirect_mut2<F: FnMut(i32) -> i32>(mut f: F) -> i32 { f(10) }
fn indirect_once2<F: FnOnce(i32) -> i32>(f: F) -> i32 { f(10) }

fn main() {
    assert_eq!(call_fn_ptr(), 42);
    assert_eq!(indirect(f), 42);
    assert_eq!(indirect_mut(f), 42);
    assert_eq!(indirect_once(f), 42);
    assert_eq!(indirect2(g), 420);
    assert_eq!(indirect_mut2(g), 420);
    assert_eq!(indirect_once2(g), 420);
    assert!(return_fn_ptr() == f);
    assert!(return_fn_ptr() as unsafe fn() -> i32 == f as fn() -> i32 as unsafe fn() -> i32);
}
