// costs Approx 140CUs
// Direct syscall to Solana's native SHA256 implementation
#[cfg(any(target_os = "solana", target_arch = "bpf"))]
unsafe extern "C" {
    pub unsafe fn sol_sha256(vals: *const u8, val_len: u64, hash_result: *mut u8) -> u64;
}

#[cfg(not(any(target_os = "solana", target_arch = "bpf")))]
pub unsafe fn sol_sha256(vals: *const u8, val_len: u64, hash_result: *mut u8) {
    #[cfg(test)]
    {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();

        // Cast the pointer back to our SolBytes struct array
        let slices = core::slice::from_raw_parts(vals as *const SolBytes, val_len as usize);

        for slice in slices {
            // Hash the actual data the slice points to
            let data = core::slice::from_raw_parts(slice.ptr, slice.len as usize);
            hasher.update(data);
        }

        let hash = hasher.finalize();
        core::ptr::copy_nonoverlapping(hash.as_ptr(), hash_result, 32);
    }
}

#[repr(C)]
pub struct SolBytes {
    pub ptr: *const u8,
    pub len: u64,
}
