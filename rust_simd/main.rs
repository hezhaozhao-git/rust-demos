use std::{arch::x86_64::*, mem};

fn main() {
    // 创建一个包含8个32位整数的SIMD向量
    let size = 1000000;
    let a = vec![1 as i32; size];

    for chunk in a.chunks(8) {
        let aa:  [i32; 8]  = chunk.to_owned().try_into().expect("Chunk size mismatch");
        let a_simd: __m256i = unsafe { mem::transmute( aa) };
        unsafe {
            // 创建一个包含8个32位整数的SIMD向量，每个元素都是5
            let b = _mm256_set1_epi32(5);
    
            // 将a_simd和b中对应位置的元素相加
            let result = _mm256_add_epi32(a_simd, b);
    
            // 将结果转换回整数数组
            let result_array: [i32; 8] =  mem::transmute(result);
    
            // 将结果打印出来
            println!("{:?}", result_array);
        }
    }
    

   
}
