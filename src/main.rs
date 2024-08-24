use futures::future::join_all;
use reqwest::{self, StatusCode};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use tokio::{
    runtime::Runtime,
    time::{sleep, timeout},
};

#[tokio::main]
async fn main() {
    // 案例1：异步HTTP请求
    let mut urls = vec![];
    urls.push("https://www.baidu.com"); // 百度
    urls.push("https://www.qq.com"); // 腾讯
    urls.push("https://www.alibaba.com"); // 阿里巴巴
    urls.push("https://www.taobao.com"); // 淘宝
    urls.push("https://www.jd.com"); // 京东
    urls.push("https://www.sina.com.cn"); // 新浪
    urls.push("https://www.163.com"); // 网易
    urls.push("https://www.sohu.com"); // 搜狐
    urls.push("https://baike.baidu.com"); // 百度百科
    urls.push("https://www.toutiao.com"); // 今日头条

    for url in urls.iter() {
        let status_code = get_http_status(url).await;
        println!("url:{}\tstatus code:{}", url, status_code);
    }

    // 案例2：异步任务超时
    let timeout_duration = Duration::from_secs(2);

    // 启动超时任务
    for i in 0..5 {
        match timeout(timeout_duration, timeout_async(Duration::from_secs(i))).await {
            Ok(_) => println!("Task [{}] completed successfully", i),
            Err(_) => println!("Task [{}] timed out", i),
        }
    }

    // 案例3：多线程数据共享
    let counters = Arc::new(Mutex::new(0_u32));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counters);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter:{}", counters.lock().unwrap());

    // 案例4：异步和多线程混合
    let mut handles = vec![];
    for i in 0..10 {
        let handle = thread::spawn(move || {
            // 在每个线程中创建一个 tokio 运行时
            let rt = Runtime::new().unwrap();

            // 使用运行时来执行异步任务
            rt.block_on(async {
                // 模拟I/O操作
                sleep(Duration::from_secs(3)).await;
                // 输出线程编写和任务完成的消息
                println!("Thread {}: Task completed", i);
            });
        });

        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads have completed.");

    let mut tasks = vec![];

    let start = Instant::now();

    // 启动多个异步任务
    for i in 0..10 {
        let task_id = i;
        let task = tokio::spawn(async move { compute_task(task_id).await });
        tasks.push(task);
    }

    // 等待所有任务完成
    let results = join_all(tasks).await;

    // 输出每个任务的结果
    for result in results {
        match result {
            Ok(message) => println!("{}", message),
            Err(e) => eprintln!("Task failed: {:?}", e),
        }
    }

    let duration = start.elapsed();
    println!("All tasks completed in {:?}", duration);
}

async fn get_http_status(url: &str) -> StatusCode {
    let status_code = match reqwest::get(url).await {
        Ok(resp) => resp.status(),
        Err(e) => {
            println!("error: {}", e);
            e.status().unwrap()
        }
    };
    return status_code;
}

async fn timeout_async(time_date: Duration) {
    sleep(time_date).await;
}

// 模拟计算密集型任务的异步函数
async fn compute_task(task_id: u32) -> String {
    // 模拟计算密集型操作，例如执行一个长时间计算
    let result = long_compytation(task_id).await;
    format!("Task {} completed with result: {}", task_id, result)
}

// 模拟长时间计算的异步
async fn long_compytation(task_id: u32) -> u64 {
    let mut sum = 0;
    for i in 0..1_000_000 {
        sum += i * task_id as u64;
    }
    return sum;
}
