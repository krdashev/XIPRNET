//! Secure memory management and zeroization
//! 
//! Provides secure memory wiping and cryptographic erase functionality

use zeroize::Zeroize;

/// Securely zeroize a byte slice
pub fn zeroize_bytes(bytes: &mut [u8]) {
    bytes.zeroize();
}

/// Securely zeroize a string
pub fn zeroize_string(s: &mut String) {
    unsafe {
        s.as_mut_vec().zeroize();
    }
    s.clear();
}

/// Securely zeroize a vector
pub fn zeroize_vec<T: Zeroize>(vec: &mut Vec<T>) {
    for item in vec.iter_mut() {
        item.zeroize();
    }
    vec.clear();
}

/// Cryptographic erase trait for sensitive data
pub trait CryptographicErase {
    fn cryptographically_erase(&mut self);
}

impl<T: Zeroize> CryptographicErase for T {
    fn cryptographically_erase(&mut self) {
        self.zeroize();
    }
}

/// Secure memory allocator wrapper
pub struct SecureMemory<T: Zeroize> {
    data: T,
}

impl<T: Zeroize> SecureMemory<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
    
    pub fn get(&self) -> &T {
        &self.data
    }
    
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T: Zeroize> Drop for SecureMemory<T> {
    fn drop(&mut self) {
        self.data.zeroize();
    }
}
