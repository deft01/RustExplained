#![allow(warnings)]

fn main() {
    let target: usize = 0;

    let ref_foo = &target;
    let ref_bar = &target;
    let ref_baz = &target;

    println!("{}", ref_bar);
}


//#[test]
//#[should_panic]
//fn infinite_readonly_refs_on_mutable_target_TRY_WRITE() {
//    let mut target: usize = 0;
//
//    // Uses of raw pointer (*const) instead of normal ref (&)
//    // for bypassing security checks
//    let ref_foo = &target as *const usize; 
//    let ref_bar = &target as *mut usize;
//    let ref_baz = &target as *const usize;
//
//    // Unsafe operation : 
//    //     - Unsafe pointer dereferencement 
//    //     - Send unsafe pointer (mut *const usize ref) accross thread
//    unsafe {
//        // Create another thread to demonstrate race condition 
//        let handle = std::thread::spawn(|| {
//            let mut_ref = &mut ref_bar as *mut usize; // Mutate using raw pointer
//            unsafe { // Unsafe operation : unsafe pointer dereferencement 
//                *mut_ref = 42; // Increment the target
//            }
//        });
//
//        handle.join().unwrap(); // Wait for the spawned thread to finish
//
//        println!("Display the ref content : {}", *ref_baz);
//        if *ref_baz == 42 {
//            panic!();
//        }
//    }
//}
//

//// Define a wrapper type around *mut usize
//struct RawPtrWrapper(*mut usize);
//
//// Implement Send and Sync for the wrapper type
//unsafe impl Send for RawPtrWrapper {}
//unsafe impl Sync for RawPtrWrapper {}
//
//#[test]
//#[should_panic]
//fn infinite_readonly_refs_on_mutable_target_TRY_WRITE() {
//    unsafe {
//        thread::scope(|s| {
//            let mut target: usize = 0;
//
//            // References on target
//            let ref_foo = &target as *const usize;
//            // Without Arc, the compilo see that the ref is destroyed at the end of the thread, and may not
//            // live enough to be printed.
//            // Without the WRAPPING struct the raw ref is not SEND and SYNC and cant be passed to a thread
//            let ref_bar = RawPtrWrapper(&mut target as *mut usize);
//            let ref_baz = &target as *const usize;
//
//            s.spawn(|| {
//                let RawPtrWrapper(mut_ref) = { ref_bar };
//
//                // Unsafe operation : dereference the pointer
//                *mut_ref = 42;
//            });
//
//            let ten_millis = std::time::Duration::from_millis(100);
//            thread::sleep(ten_millis);
//
//            // Unsafe operation : dereference the pointer
//            println!("(3) Target content: {}", *ref_baz);
//            // Unsafe operation : dereference the pointer
//            if *ref_baz == 42 {
//                panic!();
//            }
//        });
//    }
//}
//
//#[test]
//fn infinite_readonly_refs_on_mutable_target_TRY_WRITE_() {
//    use ::std::thread;
//
//    let mut target: usize = 0;
//
//    let ref_bar = &mut target;
//
//    // Move `ref_bar` into the thread (ownership transfer)
//    thread::scope(|spawner| {
//        spawner.spawn(move || {
//            // Dereference the pointer from the wrapper and mutate it
//            *ref_bar = 42;
//        });
//
//        let ten_millis = std::time::Duration::from_millis(100);
//        thread::sleep(ten_millis);
//    });
//    
//    println!("Target content: {}", target);
//
//    // Compare the dereferenced value with 42
//    assert_eq!(target, 42);
//}
//

    // Move `ref_bar` into the thread (ownership transfer)
    //let handle = thread::spawn(move || {
    //    // Dereference the pointer from the wrapper and mutate it
    //    let RawPtrWrapper(mut_ref) = *ref_bar; // Ownership is moved here

    //    // Unsafe operation : dereference the pointer
    //    unsafe {
    //        *mut_ref = 42; // Mutate the target
    //    }
    //});

    //let ten_millis = std::time::Duration::from_millis(100);
    //thread::sleep(ten_millis);

//    unsafe {
//        // Unsafe operation : dereference the pointer
//        println!("Target content: {}", *ref_baz);
//
//        // Unsafe operation : dereference the pointer
//        // Compare the dereferenced value with 42
//        if *ref_baz == 42 {
//            panic!(); // If the target was modified, the test should panic
//        }
//    }
//}


//// Define a wrapper type around *mut usize
//struct RawPtrWrapper(*mut usize);
//
//// Implement Send and Sync for the wrapper type (unsafe because we bypass Rust's thread safety)
//unsafe impl Send for RawPtrWrapper {}
//unsafe impl Sync for RawPtrWrapper {}
//
//#[test]
//#[should_panic] // We expect a panic to show a race condition has occurred
//fn infinite_readonly_refs_on_mutable_target_TRY_WRITE() {
//    let mut target: usize = 0;
//
//    // Use raw pointers to access the same target from multiple locations
//    let ref_foo = &target as *const usize;
//    let ref_bar = RawPtrWrapper(&mut target as *mut usize); // Wrapped mutable pointer
//    let ref_baz = &target as *const usize;
//
//    unsafe {
//        // Move `ref_bar` into the thread (ownership transfer)
//        let handle = thread::spawn(move || {
//            // Dereference the pointer from the wrapper and mutate it
//            let RawPtrWrapper(mut_ref) = ref_bar; // Ownership is moved here
//            unsafe {
//                *mut_ref = 42; // Mutate the target
//            }
//        });
//
//        handle.join().unwrap(); // Wait for the spawned thread to finish
//
//        // Dereference the pointer to compare with the value
//        let ref_baz_value = *ref_baz;
//        println!("Display the ref content: {}", ref_baz_value);
//
//        // Compare the dereferenced value with 42
//        if ref_baz_value == 42 {
//            panic!(); // If the target was modified, the test should panic
//        }
//    }
//}


//// Define a wrapper type around *mut usize
//struct RawPtrWrapper(UnsafeCell<usize>);
//
//// Implement Send and Sync for the wrapper type (unsafe because we bypass Rust's thread safety)
//unsafe impl Send for RawPtrWrapper {}
//unsafe impl Sync for RawPtrWrapper {}
//
//#[test]
//#[should_panic] // We expect a panic to show a race condition has occurred
//fn infinite_readonly_refs_on_mutable_target_TRY_WRITE() {
//    let target = RawPtrWrapper(UnsafeCell::new(0));
//
//    // Use raw pointers to access the same target from multiple locations
//    let ref_foo = &target.0 as *const UnsafeCell<usize>;
//    let ref_baz = &target.0 as *const UnsafeCell<usize>; // Pointer to the shared target
//
//    unsafe {
//        // Move `target` into the thread (ownership transfer)
//        let handle = thread::spawn(move || {
//            // Dereference the pointer and mutate the value inside `UnsafeCell`
//            *ref_baz = 42; // Mutate the target
//        });
//
//        handle.join().unwrap(); // Wait for the spawned thread to finish
//
//        // Dereference the pointer to compare with the value
//        let ref_foo_value = *ref_baz;
//        println!("Display the ref content: {}", ref_foo_value);
//
//        // Compare the dereferenced value with 42
//        if ref_foo_value == 42 {
//            panic!(); // If the target was modified, the test should panic
//        }
//    }
//}


//#[test]
//#[should_panic] // We expect a panic to show a race condition has occurred
//fn mytest() {
//    pub fn cell(target: &Cell<u32>) {
//        target.set(123);
//        // do some slow calculation (assume no side effects)...
//        let ten_millis = std::time::Duration::from_millis(1000);
//        thread::sleep(ten_millis);
//        target.set(321);
//    }
//
//    pub fn ref_mut(target: &mut u32) {
//        *target = 456;
//        // do some slow calculation (assume no side effects)...
//        *target = 654;
//    }
//
//    pub fn atomic(target: &AtomicU32) {
//        target.store(789, Relaxed);
//        // do some slow calculation (assume no side effects)...
//        target.store(987, Relaxed);
//    }
//
//    let target = Cell::new(0);
//
//    cell(&target);
//
//    println!("Content of cell {}", target.get());
//}
