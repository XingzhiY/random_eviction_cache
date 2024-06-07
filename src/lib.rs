use rand::Rng;
use indexmap::IndexSet;
use rand::seq::IteratorRandom;
use rand::thread_rng;


struct RandomEvictionCache {
    set: IndexSet<i32>,
    capacity: usize,
}

impl RandomEvictionCache {
    fn new(capacity: usize) -> Self {
        RandomEvictionCache {
            set: IndexSet::new(),
            capacity,
        }
    }

    fn read_function(&mut self, number: i32) -> Option<i32> {
        if self.set.contains(&number) {//O(1)
            Some(number)
        } else {
            if self.set.len() >= self.capacity {
                self.evict_random();
            }
            self.set.insert(number);//O(1)
            None
        }
    }

    fn evict_random(&mut self) {
        if !self.set.is_empty() {
            let mut rng = thread_rng();
            let random_index = (0..self.set.len()).choose(&mut rng).unwrap();
            self.set.swap_remove_index(random_index); //O(1)
        }
    }
}

#[cfg(test)]
mod tests {
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
        let mut cache = RandomEvictionCache::new(3);

        // Reading numbers and demonstrating the cache behavior
        for &number in &[1, 2, 3, 4, 5, 1, 6, 2] {
            println!("cache state: {:?}", cache.set);
            match cache.read_function(number) {
                Some(num) => println!("Hit: {}", num),
                None => println!("Miss: {}", number),
            }
        }

        println!("Final cache state: {:?}", cache.set);
    }
}
