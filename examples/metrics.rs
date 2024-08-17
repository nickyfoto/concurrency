use anyhow::Result;
use concurrency::{
    // Metrics,
    MetricsDashMap,
    // MetricsRwLock,
};
use rand::Rng;
use std::{thread, time::Duration};

const N: usize = 2;
const M: usize = 4;

fn task_worker(idx: usize, metrics: MetricsDashMap) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc(format!("call.task_worker{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(metrics: MetricsDashMap) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..5);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn main() -> Result<()> {
    // let metrics = Metrics::new();
    // let metrics = MetricsRwLock::default();
    let metrics = MetricsDashMap::default();

    // start N workers and M requesters

    // println!("{:?}", metrics.snapshot());
    println!("{:?}", metrics);

    for idx in 0..N {
        task_worker(idx, metrics.clone())?; // Metrics {data: Arc::clone(&metrics.data)}
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        // println!("{:?}", metrics.snapshot());
        println!("{:?}", metrics);
    }
}
