use std::ffi;

type Callback = unsafe extern "C" fn(data: *mut u8, len: usize) -> ffi::c_int;

#[no_mangle]
pub extern "C" fn test(cb: Option<Callback>) {
    let mut vec = vec![];
    vec.shrink_to_fit();
    assert!(vec.len() == vec.capacity());

    if let Some(cb) = cb {
        let res = unsafe {
             cb(vec.as_mut_ptr(), vec.len() as usize)
        };
        println!("{}", res)
    }
}

#[no_mangle]
pub extern "C" fn rust_munchausen_numbers() -> *mut [i32; 4] {
    // Pre-caching the power for all of the digits; 0⁰ is initially in the cache array.
    let mut cache = [0; 10];
    let mut index = 0;
    let mut munchausen_num: [i32; 4] = [0; 4];
    let munchausen_num_ptr: *mut [i32; 4] = &mut munchausen_num;

    for n in 1..=9 {
        cache[n] = (n as i32).pow(n as u32);
    }

    // Searching for Munchausen numbers iterating through a long range containing all of them.
    for n in 0..500000000 {
        if is_munchausen_number(n, &cache) {
            munchausen_num[index] = n;
            index += 1;
        }
    }

    munchausen_num_ptr
}

fn is_munchausen_number(number: i32, cache: &[i32; 10]) -> bool {
    let mut current_number = number;
    let mut sum = 0;

    // The calculation details: Do until we go through all of the digits.
    while current_number > 0 {
        // Take the last digit of a number.
        let digit = current_number % 10;
        // Add the cached power of the digit to the overall sum.
        sum += cache[digit as usize];

        if sum > number {
            return false;
        }
        // "Cut" the last digit
        current_number /= 10;
    }

    number == sum
}
