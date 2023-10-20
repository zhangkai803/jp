#[warn(non_snake_case)]
use std::{time::{SystemTime, Duration}, env, collections::HashMap};
use clap::Parser;
use reqwest::{RequestBuilder, header::{HeaderMap, HeaderValue}};
use serde::{Deserialize, Serialize};
use serde_json;

/// Simple program to translate your input to Japanese
#[derive(Parser, Debug)]
struct Args {
    /// Input to be translated
    #[arg(short, long)]
    input: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    SourceText: String,
    Source: String,
    Target: String,
    ProjectId: isize
}

impl Request {
    fn new(source_text: String) -> Request {
        Request{
            SourceText: source_text,
            Source: "auto".to_string(),
            Target: "ja".to_string(),
            ProjectId: 0,
        }
    }
}

// #[derive(Serialize, Deserialize, Debug, Default)]
// struct ErrorBody {
//     Code: String,
//     Message: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct SuccResponse {
//     Source: String,
//     Target: String,
//     TargetText: String,
//     RequestId: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct ErrorResponse {
//     Error: ErrorBody,
//     RequestId: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// enum ResponseEnum {
//     Error(ErrorResponse),
//     Succ(SuccResponse)
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct TransResponse {
//     Response: ResponseEnum
// }

fn getCanonicalRequest(payload: String) -> String {
    let HTTPRequestMethod = "POST";
    let CanonicalURI = "/";
    let CanonicalQueryString = "";
    let CanonicalHeaders = format!("content-type:application/json; charset=utf-8\nhost:tmt.tencentcloudapi.com\nx-tc-action:TextTranslate\n");
    let SignedHeaders = "content-type;host;x-tc-action".to_string();
    let HashedRequestPayload = payload;
    format!("{HTTPRequestMethod}\n{CanonicalURI}\n{CanonicalQueryString}\n{CanonicalHeaders}\n{SignedHeaders}\n{HashedRequestPayload}")
}

fn getStringToSign() -> String {
    let Algorithm: &str = "TC3-HMAC-SHA256";
    let RequestTimestamp = "";
    let CredentialScope = "//tc3_request";
    let HashedCanonicalRequest = "";
    format!("{Algorithm}\n{RequestTimestamp}\n{CredentialScope}\n{HashedCanonicalRequest}")
}

fn signature(payload: Request, now: Duration) -> String {
    let secret_id = env::var("TENCENTCLOUD_SECRET_ID").unwrap();
    let secret_key = env::var("TENCENTCLOUD_SECRET_KEY").unwrap();

    let HTTPRequestMethod = "POST";
    let CanonicalURI = "/";
    let CanonicalQueryString = "";
    let CanonicalHeaders = format!("content-type:application/json; charset=utf-8\nhost:tmt.tencentcloudapi.com\nx-tc-action:TextTranslate\n");
    let SignedHeaders = "content-type;host;x-tc-action".to_string();
    let HashedRequestPayload = serde_json::to_string(&payload).unwrap();
    let CanonicalRequest = format!("{HTTPRequestMethod}\n{CanonicalURI}\n{CanonicalQueryString}\n{CanonicalHeaders}\n{SignedHeaders}\n{HashedRequestPayload}");

    let Algorithm: &str = "TC3-HMAC-SHA256";
    let RequestTimestamp = "";
    let CredentialScope = "2023-10-11/tmt/tc3_request";
    let HashedCanonicalRequest = "";
    let StringToSign = format!("{Algorithm}\n{RequestTimestamp}\n{CredentialScope}\n{HashedCanonicalRequest}");
    let Signature = "";

    format!("{Algorithm} Credential={secret_id}/{CredentialScope}, SignedHeaders={SignedHeaders}, Signature={Signature}")
}

fn biuld_request(_input: &str) -> RequestBuilder {
    let payload = Request::new(_input.to_string());
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("Time went backwards");
    let sign = signature(payload, now);

    let mut headers = HeaderMap::new();
    headers.insert("X-TC-Action", HeaderValue::from_str("TextTranslate").unwrap());
    headers.insert("X-TC-Version", HeaderValue::from_str("2018-03-21").unwrap());
    headers.insert("X-TC-Region", HeaderValue::from_str("ap-guangzhou").unwrap());
    headers.insert("X-TC-Timestamp", HeaderValue::from_str(now.as_secs().to_string().as_str()).unwrap());
    headers.insert("Authorization", HeaderValue::from_str(sign.as_str()).unwrap());

    let client: reqwest::Client = reqwest::Client::new();
    let req = client.post("https://tmt.tencentcloudapi.com").headers(headers);

    req
}

async fn translate(_input: &str) -> Result<isize, Box<dyn std::error::Error>> {
    let req = biuld_request(_input);

    let rsp = req.send().await?;

    println!("{:?}", rsp);

    println!("{:?}", rsp.bytes().await?);

    // let rsp_json = rsp.json::<TransResponse>().await?;

    // println!("{:?}", rsp_json);

    Ok(1)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Hello {}!", args.input);

    let translated = translate(&args.input).await.unwrap();
    println!("{:?}", translated);

    Ok(())
}
