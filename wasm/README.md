currently a bit stuck on handling string based operations

Basically the method is to give and return (i32, i32) and put the string into memory in the host, then you get the return of the string back

```rs
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_void};

#[no_mangle]
pub extern fn allocate(size: usize) -> *mut c_void {
  let mut buffer = Vec::with_capacity(size);
  let pointer = buffer.as_mut_ptr();
  mem::forget(buffer);

  pointer as *mut c_void
}

#[no_mangle]
pub extern fn deallocate(pointer: *mut c_void, capacity: usize) {
  unsafe {
    let _ = Vec::from_raw_parts(pointer, 0, capacity);
  }
}

#[no_mangle]
pub extern fn greet(subject: *mut c_char) -> *mut c_char {
  let subject = unsafe { CStr::from_ptr(subject).to_bytes().to_vec() };
  let mut output = b"Hello, ".to_vec();
  output.extend(&subject);
  output.extend(&[b'!']);

  unsafe { CString::from_vec_unchecked(output) }.into_raw()
}
```

`cargo build --target wasm32-wasi`

Or with the wasm-bindgen:
