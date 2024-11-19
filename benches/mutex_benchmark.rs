use criterion::{criterion_main, BenchmarkId, Criterion};
use parking_lot::{Mutex as ParkingLotMutex, RwLock as ParkingLotRwLock};
use std::sync::Arc;
use std::thread;

use mutex_comp::my_mutex::MyMutex;
use mutex_comp::my_rw_lock::MyRWLock;
use mutex_comp::atomics_rw_lock::RwLock as AtomicsRWLock;
use mutex_comp::c_mutex::CMutex;
use mutex_comp::c_rw_lock::CRWLock;
use std::sync::{Mutex as StdMutex, RwLock as StdRwLock};
use std::time::Duration;

fn bench_mutex(c: &mut Criterion) {
    // let plot_config = PlotConfiguration::default()
    //     .summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("Mutex");
    // group.plot_config(plot_config);
    for i in 0..3 {
        group.bench_with_input(BenchmarkId::new("MyMutex", i), &i, |b, _| {
            b.iter(|| {
                let mutex = Arc::new(MyMutex::new());
                let mut handles = vec![];
                for _ in 0..10 {
                    let mutex_clone = Arc::clone(&mutex);
                    let handle = thread::spawn(move || {
                        for _ in 0..1000 {
                            mutex_clone.lock();
                            criterion::black_box(&mutex_clone);
                            mutex_clone.unlock();
                        }
                    });
                    handles.push(handle);
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            })
        });
        group.bench_with_input(BenchmarkId::new("CMutex", i), &i, |b, _| {
            b.iter(|| {
                let mutex = Arc::new(CMutex::new());
                let mut handles = vec![];
                for _ in 0..10 {
                    let mutex_clone = Arc::clone(&mutex);
                    let handle = thread::spawn(move || {
                        for _ in 0..1000 {
                            mutex_clone.lock();
                            criterion::black_box(&mutex_clone);
                            mutex_clone.unlock();
                        }
                    });
                    handles.push(handle);
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("ParkingLotMutex", i), &i, |b, _| {
            b.iter(|| {
                let mutex = Arc::new(ParkingLotMutex::new(()));
                let mut handles = vec![];
                for _ in 0..10 {
                    let mutex = Arc::clone(&mutex);
                    let handle = thread::spawn(move || {
                        for _ in 0..1000 {
                            let _lock = mutex.lock();
                            criterion::black_box(&_lock);
                        }
                    });
                    handles.push(handle);
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("StdMutex", i), &i, |b, _| {
            b.iter(|| {
                let mutex = Arc::new(StdMutex::new(()));
                let mut handles = vec![];
                for _ in 0..10 {
                    let mutex = Arc::clone(&mutex);
                    let handle = thread::spawn(move || {
                        for _ in 0..1000 {
                            let _lock = mutex.lock().unwrap();
                            criterion::black_box(&_lock);
                        }
                    });
                    handles.push(handle);
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            })
        });
    }
    group.finish();
}

fn bench_rwlock(c: &mut Criterion) {
    // let plot_config = PlotConfiguration::default()
    //     .summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("RwLock");
    // group.plot_config(plot_config);

    for i in 0..3 {
        group.bench_with_input(BenchmarkId::new("CRWLock", i), &i, |b, _| {
            b.iter(|| {
                let rwlock = Arc::new(CRWLock::new());
                let mut handles = vec![];
                for _ in 0..8 {
                    let rwlock = Arc::clone(&rwlock);
                    let handle = thread::spawn(move || {
                        for _ in 0..1000 {
                            rwlock.acquire_read();
                            criterion::black_box(&rwlock);
                            rwlock.release();
                        }
                    });
                    handles.push(handle);
                }
                for _ in 0..2 {
                    let rwlock = Arc::clone(&rwlock);
                    let handle = thread::spawn(move || {
                        for _ in 0..100 {
                            rwlock.acquire_write();
                            criterion::black_box(&rwlock);
                            rwlock.release();
                        }
                    });
                    handles.push(handle);
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            })
        });
        // group.bench_with_input(BenchmarkId::new("AtomicsRWLock", i), &i, |b, _| {
        //     b.iter(|| {
        //         let rwlock = Arc::new(AtomicsRWLock::new(()));
        //         let mut handles = vec![];
        //         for _ in 0..8 {
        //             let rwlock = Arc::clone(&rwlock);
        //             let handle = thread::spawn(move || {
        //                 for _ in 0..1000 {
        //                     let guard = rwlock.read();
        //                     criterion::black_box(&rwlock);
        //                     drop(guard);
        //                 }
        //             });
        //             handles.push(handle);
        //         }
        //         for _ in 0..2 {
        //             let rwlock = Arc::clone(&rwlock);
        //             let handle = thread::spawn(move || {
        //                 for _ in 0..100 {
        //                     let guard = rwlock.write();
        //                     criterion::black_box(&rwlock);
        //                     drop(guard)
        //
        //                 }
        //             });
        //             handles.push(handle);
        //         }
        //         for handle in handles {
        //             handle.join().unwrap();
        //         }
        //         drop(rwlock);
        //     })
        // });
        group.bench_with_input(BenchmarkId::new("MyRWLock", i), &i, |b, _| {
            b.iter(|| {
                let rwlock = Arc::new(MyRWLock::new());
                let mut handles = vec![];
                for _ in 0..8 {
                    let rwlock = Arc::clone(&rwlock);
                    let handle = thread::spawn(move || {
                        for _ in 0..1000 {
                            rwlock.acquire_read();
                            criterion::black_box(&rwlock);
                            rwlock.release();
                        }
                    });
                    handles.push(handle);
                }
                for _ in 0..2 {
                    let rwlock = Arc::clone(&rwlock);
                    let handle = thread::spawn(move || {
                        for _ in 0..100 {
                            rwlock.acquire_write();
                            criterion::black_box(&rwlock);
                            rwlock.release();
                        }
                    });
                    handles.push(handle);
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("ParkingLotRwLock", i), &i, |b, _| {
            b.iter(|| {
                let rwlock = Arc::new(ParkingLotRwLock::new(()));
                let mut handles = vec![];
                for _ in 0..8 {
                    let rwlock = Arc::clone(&rwlock);
                    let handle = thread::spawn(move || {
                        for _ in 0..1000 {
                            let _guard = rwlock.read();
                            criterion::black_box(&_guard);
                        }
                    });
                    handles.push(handle);
                }
                for _ in 0..2 {
                    let rwlock = Arc::clone(&rwlock);
                    let handle = thread::spawn(move || {
                        for _ in 0..100 {
                            let _guard = rwlock.write();
                            criterion::black_box(&_guard);
                        }
                    });
                    handles.push(handle);
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("StdRwLock", i), &i, |b, _| {
            b.iter(|| {
                let rwlock = Arc::new(StdRwLock::new(()));
                let mut handles = vec![];
                for _ in 0..8 {
                    let rwlock = Arc::clone(&rwlock);
                    let handle = thread::spawn(move || {
                        for _ in 0..1000 {
                            let _guard = rwlock.read().unwrap();
                            criterion::black_box(&_guard);
                        }
                    });
                    handles.push(handle);
                }
                for _ in 0..2 {
                    let rwlock = Arc::clone(&rwlock);
                    let handle = thread::spawn(move || {
                        for _ in 0..100 {
                            let _guard = rwlock.write().unwrap();
                            criterion::black_box(&_guard);
                        }
                    });
                    handles.push(handle);
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            })
        });
    }
    group.finish();
}

fn custom_criterion_main() {
    let mut criterion = Criterion::default()
        .sample_size(100)
        .measurement_time(Duration::from_secs(6));

    bench_mutex(&mut criterion);
    bench_rwlock(&mut criterion);
}
criterion_main!(custom_criterion_main);
