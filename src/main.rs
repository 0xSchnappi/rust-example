use std::time::Duration;
use reqwest::{self, StatusCode};
use tokio::time::{sleep, timeout};

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
