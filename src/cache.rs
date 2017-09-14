pub mod bflru {
    pub struct Cache4<T> {
        contents: [T; 7], // 3 extra slots as padding - actual cache is in 3..7
    }

    impl<T: Copy> Cache4<T> {
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

    impl<T: Default> Cache4<T> {
        // Initializes a cache with all elements set to the cached type's default value.
        pub fn new() -> Cache4<T> {
            Cache4 {
                contents: [
                    T::default(), T::default(),
                    T::default(), T::default(),
                    T::default(), T::default(),
                    T::default()
                ],
            }
        }
    }

    impl<T: PartialEq> Cache4<T> {
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
    
    // ------------------------------------------------------------------------
    
    pub struct Cache8<T> {
        contents: [T; 8],
        order: u32,
    }
    
    impl<T> Cache8<T> {
        pub fn get(&mut self, index: usize) -> &T {
            let ix4 = index * 4;
            let slot = (self.order >> ix4) & 0xF;
            
            let moved_mtf = (self.order << 4) + slot;
            let keep_mask = !0xF << ix4;
            self.order = (self.order & keep_mask) | (moved_mtf & !keep_mask);
            
            &self.contents[slot as usize]
        }
        
        pub fn put(&mut self, value: T) {
            let slot = (self.order >> 28) & 0xF;
            self.order = (self.order << 4) + slot;
            self.contents[slot as usize] = value;
        }
    }
    
    impl<T: Default> Cache8<T> {
        pub fn new() -> Cache8<T> {
            Cache8 {
                order: 0x76543210,
                contents: [
                    T::default(), T::default(),
                    T::default(), T::default(),
                    T::default(), T::default(),
                    T::default(), T::default()
                ],
            }
        }
    }
    
    impl<T: PartialEq> Cache8<T> {
        pub fn find(&mut self, value: &T) -> Option<usize> {
            for i in 0..8 {
                let slot = (self.order >> (i * 4)) & 0xF;
                
                if self.contents[slot as usize].eq(value) {
                    return Some(i);
                }
            }
            
            None
        }
    }
    
    // ------------------------------------------------------------------------
    
    pub struct Cache16<T> {
        contents: [T; 16],
        order: u64,
    }
    
    impl<T> Cache16<T> {
        pub fn get(&mut self, index: usize) -> &T {
            let ix4 = index * 4;
            let slot = (self.order >> ix4) & 0xF;
            
            let moved_mtf = (self.order << 4) + slot;
            let keep_mask = !0xF << ix4;
            self.order = (self.order & keep_mask) | (moved_mtf & !keep_mask);
            
            &self.contents[slot as usize]
        }
        
        pub fn put(&mut self, value: T) {
            let slot = (self.order >> 60) & 0xF;
            self.order = (self.order << 4) + slot;
            self.contents[slot as usize] = value;
        }
    }
    
    impl<T: Default> Cache16<T> {
        pub fn new() -> Cache16<T> {
            Cache16 {
                order: 0xFEDCBA9876543210,
                contents: [
                    T::default(), T::default(), T::default(), T::default(),
                    T::default(), T::default(), T::default(), T::default(),
                    T::default(), T::default(), T::default(), T::default(),
                    T::default(), T::default(), T::default(), T::default()
                ],
            }
        }
    }
    
    impl<T: PartialEq> Cache16<T> {
        pub fn find(&mut self, value: &T) -> Option<usize> {
            for i in 0..16 {
                let slot = (self.order >> (i * 4)) & 0xF;
                
                if self.contents[slot as usize].eq(value) {
                    return Some(i);
                }
            }
            
            None
        }
    }
    
    #[cfg(test)]
    mod tests {
        use bflru::Cache4;
        use bflru::Cache8;
        use bflru::Cache16;

        #[test]
        fn cache4_basic_usage() {
            let mut lru = Cache4::<u8>::new();
            
            for i in 0..4 { lru.put(i + 1); }
            
            assert_eq!(*lru.get(0), 4);
            assert!(lru.find(&10).is_none());
            
            for i in 0..4 {
                assert_eq!(*lru.get(3), i + 1);
                assert_eq!(*lru.get(0), i + 1);
            }
            
            assert_eq!(lru.find(&3).unwrap(), 1);
            assert_eq!(*lru.get(0), 4);
            
            for i in 0..3 {
                let expected = (3 - i) as u8;
                assert_eq!(*lru.get(i + 1), expected);
                assert_eq!(*lru.get(0), expected);
            }
        }
        
        #[test]
        fn cache8_basic_usage() {
            let mut lru = Cache8::<u8>::new();
            
            for i in 0..8 { lru.put(i + 1); }
            
            assert_eq!(*lru.get(0), 8);
            assert!(lru.find(&10).is_none());
            
            for i in 0..8 {
                assert_eq!(*lru.get(7), i + 1);
                assert_eq!(*lru.get(0), i + 1);
            }
            
            assert_eq!(lru.find(&7).unwrap(), 1);
            assert_eq!(*lru.get(0), 8);
            
            for i in 0..7 {
                let expected = (7 - i) as u8;
                assert_eq!(*lru.get(i + 1), expected);
                assert_eq!(*lru.get(0), expected);
            }
        }
        
        #[test]
        fn cache16_basic_usage() {
            let mut lru = Cache16::<u8>::new();
            
            for i in 0..16 { lru.put(i + 1); }
            
            assert_eq!(*lru.get(0), 16);
            assert!(lru.find(&20).is_none());
            
            for i in 0..16 {
                assert_eq!(*lru.get(15), i + 1);
                assert_eq!(*lru.get(0), i + 1);
            }
            
            assert_eq!(lru.find(&15).unwrap(), 1);
            assert_eq!(*lru.get(0), 16);
            
            for i in 0..15 {
                let expected = (15 - i) as u8;
                assert_eq!(*lru.get(i + 1), expected);
                assert_eq!(*lru.get(0), expected);
            }
        }
    }
}
