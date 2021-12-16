fn main() {

    let mut t =  libc::timespec { tv_sec: 0, tv_nsec: 0 } ;

    // we silently fail because we pass the wrong clock id
    // There is error handling missing. A user will receive
    // the wrong values but not encounter an error. This is even worse than a crash
    // If using external functions that do not follow the rust conventions (how could they?)
    // make sure to put error handling at the same encapsulation layer, where the unsafe access occurs.
    // See 04_syscall_better for a better example
    let _ = unsafe { libc::clock_gettime(99, &mut t) };   
    println!("wrong: {} {}", t.tv_sec, t.tv_nsec);
    
}