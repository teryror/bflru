mod bflru {
    pub struct Cache<T> {
        contents: [T; 7], // 3 extra slots as padding - actual cache is in 3..7
    }

    impl<T: Copy> Cache<T> {
        // Gets an element from the cache as specified by the `index` parameter,
        // where 0 is the most recently used element, and 3 is the least recently used.
        pub fn get(&mut self, index: usize) -> &T {
            let result = self.contents[index + 3];
            self.contents[index + 3] = self.contents[index + 2];
            self.contents[index + 2] = self.contents[index + 1];
            self.contents[index + 1] = self.contents[index];
            self.contents[3] = result;
            
            &self.contents[3]
        }

        // Evicts the least recently used element from the cache and inserts
        // the specified value at index = 0, making it the most recently used
        // element.
        pub fn put(&mut self, value: T) {
            let index: usize = 3;
            self.contents[index + 3] = self.contents[index + 2];
            self.contents[index + 2] = self.contents[index + 1];
            self.contents[index + 1] = self.contents[index];
            self.contents[3] = value;
        }
    }

    impl<T: Default> Cache<T> {
        // Initializes a cache with all elements set to the cached type's default value.
        pub fn new() -> Cache<T> {
            Cache {
                contents: [
                    T::default(),
                    T::default(),
                    T::default(),
                    T::default(),
                    T::default(),
                    T::default(),
                    T::default()
                ],
            }
        }
    }

    impl<T: PartialEq> Cache<T> {
        // Searches the cache for the specified value, and returns its
        // Some(index) if it was found, None otherwise.
        // This does not move the value to the front, which might be more
        // useful when implementing an optimizing LZ parser.
        pub fn find(&mut self, value: &T) -> Option<usize> {
            for i in 0..4 {
                if self.contents[i + 3].eq(value) {
                    return Some(i);
                }
            }
            
            None
        }
    }
    
    #[cfg(test)]
    mod tests {
        use bflru::Cache;

        #[test]
        fn basic_usage() {
            let mut test_cache = Cache::<u8>::new();

            test_cache.put(1);
            test_cache.put(2);
            test_cache.put(3);
            test_cache.put(4);

            assert!(test_cache.find(&5).is_none());
            
            assert_eq!(*test_cache.get(0), 4);

            assert_eq!(*test_cache.get(3), 1);
            assert_eq!(*test_cache.get(0), 1);

            assert_eq!(*test_cache.get(3), 2);
            assert_eq!(*test_cache.get(0), 2);

            assert_eq!(*test_cache.get(3), 3);
            assert_eq!(*test_cache.get(0), 3);

            assert_eq!(*test_cache.get(3), 4);
            assert_eq!(*test_cache.get(0), 4);
            
            assert_eq!(test_cache.find(&3).unwrap(), 1);
            assert_eq!(*test_cache.get(0), 4);
            
            assert_eq!(*test_cache.get(1), 3);
            assert_eq!(*test_cache.get(0), 3);
            
            assert_eq!(*test_cache.get(2), 2);
            assert_eq!(*test_cache.get(0), 2);
            
            assert_eq!(*test_cache.get(3), 1);
            assert_eq!(*test_cache.get(0), 1);
        }
    }
}
