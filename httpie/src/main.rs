use clap::Parser;
use anyhow::{anyhow, Result};
use colored::Colorize;
use reqwest::Url;
use std::{collections::HashMap, str::FromStr};
use reqwest::{header,Response,Client};
use mime::Mime;


// 定义Httpie的CLI主入口，它包含若干个子命令


/// a native httpie implementation with Rust,can you imagine how easy it is?
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Try Chen <tyr@chen.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令分别对应不同的HTTP方法，目前只支持get / post 
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
    // 我们暂时不支持其他HTTP方法
}

// get 子命令

/// feed get with and url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP请求的URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}


// post 子命令。需要输入一个 URL，和若干个可选的 key=value，用于提供 json body

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser,Debug)]
struct Post {
    /// HTTP请求的URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    /// HTTP请求的body
    #[clap(parse(try_from_str=parse_kv_pair))]
    body: Vec<KvPair>,
}

fn parse_url(s: &str) -> Result<String> {
    //这里我们仅仅检查一下url是否合法
    let _url: Url = s.parse()?;

    Ok(s.into())
}

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v :String,
}


/// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s :&str) -> Result<Self, Self::Err> {
        // 使用 = 进行 split, 这会得到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None 
            // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误 
            k: (split.next().ok_or_else(err)?).to_string(), 
            // 从迭代器中取第二个结果作为 value 
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// 因为我们为 KvPair 实现了 FromStr，这里可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}


async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

// 打印服务器版本号 + 状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n",status);
}

// 打印服务器返回的HTTP header
fn print_headers(resp: &Response) {
    for (name,value) in resp.headers() {
        println!("{}: {:?}",name.to_string().green(),value);
    }
    print!("\n");
}

// 打印服务器返回的HTTP body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}",jsonxf::pretty_print(body).unwrap().cyan());
        }
        _ => println!("{}",body)
    }
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime,&body);
    Ok(())
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers().get(header::CONTENT_TYPE).
        map(|v|v.to_str().unwrap().parse().unwrap())
}


#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT,"Rust Httpie".parse()?);
    // 添加http请求
    let client = Client::builder().default_headers(headers).build()?;
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}

// 仅在 cargo test 时才编译
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err()); 
        assert_eq!( parse_kv_pair("a=1").unwrap(), KvPair {
             k: "a".into(), v: "1".into() 
        }); 
        assert_eq!( parse_kv_pair("b=").unwrap(), KvPair {
             k: "b".into(), v: "".into() 
        } ); 
    }
}