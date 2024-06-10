mod lib;
use crate::lib::RandomEvictionCache;

fn main() {
    let ki = 1024;
    let mi = 1024 * ki;
    let c2: usize = mi;
    let c3 = 96 * mi;
    let data_size = 128 * mi;
    let repeat_time = 2;

    // let mut data = vec![0; repeat_time * (data_size - c2)];
    let mut data = Vec::new();
    lib::fill_sawtooth_trace(repeat_time, data_size, c2, &mut data);
    // lib::fill_cyclic_trace(repeat_time, data_size, &mut data);
    // println!("Data: {:?}", data);
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
    println!(
        "Miss ratio: {}",
        (miss_counter - data_size) as f32 / (data.len() - data_size) as f32
    );
}
