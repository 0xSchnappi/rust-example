use reqwest::{self, StatusCode};

#[tokio::main]
async fn main() {
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
