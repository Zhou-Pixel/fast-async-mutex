#[cfg(test)]
mod tests {
    use futures::StreamExt;
    use test::Bencher;
    use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

    #[bench]
    fn create(b: &mut Bencher) {
        b.iter(|| RwLock::new(()));
    }

    #[bench]
    fn concurrency_write(b: &mut Bencher) {
        let mut runtime = tokio::runtime::Builder::new()
            .enable_all()
            .threaded_scheduler()
            .build()
            .unwrap();
        b.iter(|| {
            runtime.block_on(async {
                let c = RwLock::new(0);

                futures::stream::iter(0..10000u64)
                    .for_each_concurrent(None, |_| async {
                        let mut co: RwLockWriteGuard<i32> = c.write().await;
                        *co += 1;
                    })
                    .await;
            })
        });
    }

    #[bench]
    fn step_by_step_writing(b: &mut Bencher) {
        let mut runtime = tokio::runtime::Builder::new()
            .enable_all()
            .threaded_scheduler()
            .build()
            .unwrap();
        b.iter(|| {
            runtime.block_on(async {
                let c = RwLock::new(0);

                futures::stream::iter(0..10000i32)
                    .for_each(|_| async {
                        let mut co: RwLockWriteGuard<i32> = c.write().await;
                        *co += 1;
                    })
                    .await;
            })
        });
    }

    #[bench]
    fn concurrency_read(b: &mut Bencher) {
        let mut runtime = tokio::runtime::Builder::new()
            .enable_all()
            .threaded_scheduler()
            .build()
            .unwrap();
        b.iter(|| {
            runtime.block_on(async {
                let c = RwLock::new(0);

                let co: RwLockReadGuard<i32> = c.read().await;

                futures::stream::iter(0..10000u64)
                    .for_each_concurrent(None, |_| async {
                        let co2: RwLockReadGuard<i32> = c.read().await;
                        assert_eq!(*co, *co2)
                    })
                    .await;
            })
        });
    }

    #[bench]
    fn step_by_step_read(b: &mut Bencher) {
        let mut runtime = tokio::runtime::Builder::new()
            .enable_all()
            .threaded_scheduler()
            .build()
            .unwrap();
        b.iter(|| {
            runtime.block_on(async {
                let c = RwLock::new(0);

                let co: RwLockReadGuard<i32> = c.read().await;

                futures::stream::iter(0..10000u64)
                    .for_each(|_| async {
                        let co2: RwLockReadGuard<i32> = c.read().await;
                        assert_eq!(*co, *co2)
                    })
                    .await;
            })
        });
    }
}