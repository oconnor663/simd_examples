pub fn sum_portable(nums: &[u64]) -> u64 {
    let mut sum = 0u64;
    for &n in nums {
        sum = sum.wrapping_add(n);
    }
    sum
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn sum_avx2(nums: &[u64]) -> u64 {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let mut i = 0;
    let mut sums_vec = _mm256_set1_epi64x(0i64);
    let batch_end = nums.len() - (nums.len() % 4);
    while i < batch_end {
        let ptr = nums.as_ptr().add(i) as *const __m256i;
        let loaded_vec = _mm256_loadu_si256(ptr);
        sums_vec = _mm256_add_epi64(sums_vec, loaded_vec);
        i += 4;
    }
    let sums_array: [u64; 4] = std::mem::transmute(sums_vec);
    let mut sum: u64 = sums_array[0]
        .wrapping_add(sums_array[1])
        .wrapping_add(sums_array[2])
        .wrapping_add(sums_array[3]);
    while i < nums.len() {
        sum = sum.wrapping_add(nums[i]);
        i += 1
    }
    sum
}

pub fn sum_avx2_static(nums: &[u64]) -> u64 {
    #[cfg(target_feature = "avx2")]
    unsafe {
        return sum_avx2(nums);
    }
    #[allow(unreachable_code)]
    sum_portable(nums)
}

pub fn sum_avx2_dynamic(nums: &[u64]) -> u64 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                return sum_avx2(nums);
            }
        }
    }
    sum_portable(nums)
}

#[cfg(test)]
mod test {
    use super::*;

    const NUMS: &[u64] = &[3, 1, 4, 1, 5, 9];
    const EXPECTED: u64 = 23;

    #[test]
    fn test_portable() {
        assert_eq!(EXPECTED, sum_portable(NUMS));
    }

    #[test]
    fn test_static() {
        assert_eq!(EXPECTED, sum_avx2_static(NUMS));
    }

    #[test]
    fn test_dynamic() {
        assert_eq!(EXPECTED, sum_avx2_dynamic(NUMS));
    }
}
