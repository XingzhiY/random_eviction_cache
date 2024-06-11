use indexmap::IndexSet;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use rand::Rng;
pub struct RandomEvictionCache {
    set: IndexSet<i32>,
    capacity: usize,
}

impl RandomEvictionCache {
    pub fn new(capacity: usize) -> Self {
        RandomEvictionCache {
            set: IndexSet::new(),
            capacity,
        }
    }

    pub fn read_function(&mut self, number: i32) -> Option<i32> {
        if self.set.contains(&number) {
            //O(1)
            Some(number)
        } else {
            if self.set.len() >= self.capacity {
                self.evict_random();
            }
            self.set.insert(number); //O(1)
            None
        }
    }

    pub fn evict_random(&mut self) {
        if !self.set.is_empty() {
            let mut rng = thread_rng();
            let random_index = (0..self.set.len()).choose(&mut rng).unwrap();
            self.set.swap_remove_index(random_index); //O(1)
        }
    }
}

pub fn fill_sawtooth_trace(repeat_time: usize, data_size: usize, c2: usize, data: &mut Vec<i32>) {
    for i in 1..=c2 {
        data.push(i as i32);
    } // 1 to 1
    for _ in 0..repeat_time {
        for i in 1..=data_size {
            if i <= c2 {
                continue;
            }
            data.push(i as i32);
        } // 2 to 128
        for i in 1..=data_size {
            if i <= c2 {
                continue;
            }
            data.push((data_size + 1 - i) as i32);
        } // 127 to 1
    }
}

pub fn fill_cyclic_trace(repeat_time: usize, data_size: usize, data: &mut Vec<i32>) {
    for _ in 0..repeat_time {
        for i in 1..=data_size {
            data.push(i as i32);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::net::UnixDatagram;

    use super::*;

    #[test]
    fn test_basic_cyclic() {
        let capacity = 512;
        let mut cache = RandomEvictionCache::new(capacity);
        let mut misses = 0;
        let total_numbers = 1024;
        let iterations = 100;

        for _ in 0..iterations {
            for number in 1..=total_numbers {
                if cache.read_function(number).is_none() {
                    misses += 1;
                }
            }
        }

        let total_requests = total_numbers * iterations;
        let miss_ratio = misses as f64 / total_requests as f64;
        println!("Miss ratio: {}", miss_ratio);
    }

    #[test]
    fn test_cache_behavior1() {
        for _ in 0..20 {
            let ki = 1024;
            let mi = 1024 * ki;
            let c2: usize = 1;
            let c3 = 96;
            let data_size = 64;
            let repeat_time = 2000;

            // let mut data = vec![0; repeat_time * (data_size - c2)];
            let mut data = Vec::new();
            // fill_sawtooth_trace(repeat_time, data_size, c2, &mut data);
            fill_cyclic_trace(repeat_time, data_size, &mut data);
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
            // println!(
            //     "Hit ratio: {}",
            //     (hit_counter) as f32 / (data.len() - data_size) as f32
            // );
        }
    }
}
// small scale sawtooth sim
// [(data size, miss ratio), ...]
// [(64, 0), (128, 0.35433072), (256, 0.7467787), (512, 0.86972326), (1024, 0.93688035)]

// large scale sawtooth sim
// [(data size, miss ratio), ...]
// [(64mb, 0), (128mb, 0.35528725), (256mb, 0.73603165), (512mb, 0.87168735), (1024mb, 0.93592227)]

// small scale cyclic sim
// [(data size, miss ratio), ...]
// [(64, 0), (128, 0.456), (256, 0.913), (512, 0.995), (1024, 0.9999)]
