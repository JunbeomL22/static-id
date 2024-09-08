#[cfg(test)]
mod tests {
    use static_id::static_id::*;
    use std::thread;
    use std::sync::{Arc, Mutex};
    use rustc_hash::FxHashMap;
    
    #[test]
    fn test_multi_thread() {
        thread::sleep(std::time::Duration::from_secs(5));
        
        let id_32x32 = StaticId32x32::from_str("AAPL", "NASDAQ");
        let id_64x64 = StaticId::from_str("AAPL", "NASDAQ");

        let map_32x32 = Arc::new(Mutex::new(FxHashMap::default()));
        map_32x32.lock().unwrap().insert(id_32x32, 100);
        let arc_id_32x32 = Arc::new(id_32x32);

        let mut threads = Vec::new();
        for _ in 0..10 {
            let id_clone = arc_id_32x32.clone();
            let map_clone = map_32x32.clone();
            threads.push(thread::spawn(move || {
                for _ in 0..100_000 {
                    let id_thd = StaticId32x32::from_str("AAPL", "NASDAQ");
                    assert_eq!(*id_clone, id_thd);

                    let map_locked = map_clone.lock().unwrap();
                    let x = map_locked.get(&id_thd).unwrap();
                    assert_eq!(*x, 100);
                }
            }));
        }

        let map_64x64 = Arc::new(Mutex::new(FxHashMap::default()));
        map_64x64.lock().unwrap().insert(id_64x64, 200);
        let arc_id_64x64 = Arc::new(id_64x64);

        for _ in 0..10 {
            let id_clone = arc_id_64x64.clone();
            let map_clone = map_64x64.clone();
            threads.push(thread::spawn(move || {
                for _ in 0..100_000 {
                    let id_thd = StaticId::from_str("AAPL", "NASDAQ");
                    assert_eq!(*id_clone, id_thd);

                    let map_locked = map_clone.lock().unwrap();
                    let x = map_locked.get(&id_thd).unwrap();
                    assert_eq!(*x, 200);
                }
            }));
        }

        for t in threads {
            t.join().unwrap();
        }
    }
}