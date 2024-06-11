mod lib;
use crate::lib::RandomEvictionCache;

fn main() {
    let ki = 1024;
    let mi = 1024 * ki;
    let c2: usize = 1;
    let c3 = 96;
    // let data_size = 64;
    let repeat_time = 2000;
    let sawtooth = true;
    let cyclic = false;
    let test_data_sizes = vec![64, 128, 256, 512, 1024];
    let num_run = 20;

    for data_size in test_data_sizes {
        // println!("Data size: {}", data_size);
        // let mut data = vec![0; repeat_time * (data_size - c2)];
        let mut data = Vec::new();
        if sawtooth {
            lib::fill_sawtooth_trace(repeat_time, data_size, c2, &mut data);
        } else if cyclic {
            lib::fill_cyclic_trace(repeat_time, data_size, &mut data);
        }
        let mut total_mr = 0.0;
        for _ in 0..num_run {
            let mut miss_counter = 0;
            let mut hit_counter = 0;
            let mut cache = RandomEvictionCache::new(c3);

            // Reading numbers and demonstrating the cache behavior
            for &number in &data {
                // println!("cache state: {:?}", cache.set);
                match cache.read_function(number) {
                    Some(num) => {
                        // println!("Hit: {}", num)
                        hit_counter += 1;
                    }
                    None => {
                        // println!("Miss: {}", number);
                        miss_counter += 1;
                    }
                }
            }
            // println!("Final cache state: {:?}", cache.set);
            // println!("Hit: {}", hit_counter);
            // println!("Miss: {}", miss_counter);
            // println!("trace size: {}", data.len());
            // println!("data size: {}", data_size);
            // println!("since the first pass of the data is not counted as a miss, the miss ratio is calculated as (miss_counter - data_size) / (data.len() - data_size) ");
            // println!("effective trace size: {}", data.len() - data_size);
            // println!("effective miss: {}", miss_counter - data_size);
            if sawtooth {
                let tmp_sawtooth_mr = (miss_counter - data_size) as f32
                    / (data.len() - data_size + 2 * repeat_time * c2 - c2) as f32;
                // println!("Miss ratio: {}", tmp_sawtooth_mr);
                total_mr += tmp_sawtooth_mr;
            } else if cyclic {
                let tmp_cyclic_mr =
                    (miss_counter - data_size) as f32 / (data.len() - data_size) as f32;
                // println!("Miss ratio: {}", tmp_cyclic_mr);
                total_mr += tmp_cyclic_mr;
            }
        }
        println!(
            "Average miss ratio for data size equals to {}: {}",
            data_size,
            total_mr / num_run as f32
        );
    }
}

// small scale sawtooth sim
// [(data size, miss ratio), ...]
// [(64, 0), (128, 0.34439498), (256, 0.7310089), (512, 0.86947805), (1024, 0.9347599)]
