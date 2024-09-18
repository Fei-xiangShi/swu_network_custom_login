use std::fs::File;
use std::io::{self, BufRead};
use std::time::Duration;
use std::{env, thread};

use log::{error, info, warn};
use rand::Rng;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use url::{form_urlencoded, Url};

use num_bigint::{BigInt, Sign};
use hex;

fn main() {
    env_logger::init();
    info!("Starting the application...");

    loop {
        let verification_url = Url::parse("http://captive.apple.com").unwrap();
        let (success, redirect_html_string) = verify_network_status(&verification_url);
        if !success && !redirect_html_string.is_empty() {
            let redirect_url = match get_redirect_url(&redirect_html_string) {
                Some(url) => url,
                None => {
                    error!("Failed to get redirect URL");
                    thread::sleep(Duration::from_secs(20));
                    continue;
                }
            };

            let account = get_account();
            let username = account.username;
            let mut password = account.password;

            info!("Trying to login...");
            info!("Username: {}", username);
            info!("Password: {}", password);

            let env_need_encryption = env::var("SWU_NEED_ENCRYPTION").unwrap_or_else(|_| "true".to_string());
            let need_encryption = env_need_encryption == "true" || env_need_encryption.is_empty();

            if need_encryption {
                password = encrypt_password(&redirect_url, &password);
            }

            login(&username, &password, &redirect_url, need_encryption);
        }
        thread::sleep(Duration::from_secs(5));
    }
}

struct Account {
    username: String,
    password: String,
}

fn get_account() -> Account {
    let file = match File::open("accounts.txt") {
        Ok(f) => f,
        Err(e) => {
            error!("Error in opening file accounts.txt: {}", e);
            panic!("{}", e);
        }
    };

    let reader = io::BufReader::new(file);
    let accounts: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    if accounts.is_empty() {
        error!("No accounts found in accounts.txt");
        panic!("No accounts found");
    }

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..accounts.len());

    let account_line = &accounts[random_index];
    let mut fields = account_line.split_whitespace();

    let username = match fields.next() {
        Some(u) => u.to_string(),
        None => {
            error!("Invalid account line: {}", account_line);
            panic!("Invalid account line");
        }
    };

    let password = match fields.next() {
        Some(p) => p.to_string(),
        None => {
            error!("Invalid account line: {}", account_line);
            panic!("Invalid account line");
        }
    };

    Account { username, password }
}

fn login(username: &str, password: &str, redirect_url: &Url, need_encryption: bool) -> bool {
    let host = redirect_url.host_str().unwrap_or("");
    let authentication_url = format!("http://{}/eportal/InterFace.do?method=login", host);

    let query_pairs: Vec<(String, String)> = redirect_url.query_pairs().map(|(k,v)| (k.to_string(), v.to_string())).collect();
    let query_string_encoded = form_urlencoded::Serializer::new(String::new()).extend_pairs(query_pairs).finish();

    let post_data = [
        ("userId", username),
        ("password", password),
        ("service", "%E9%BB%98%E8%AE%A4"),
        ("queryString", &query_string_encoded),
        ("operatorPwd", ""),
        ("operatorUserId", ""),
        ("validcode", ""),
        ("passwordEncrypt", if need_encryption { "true" } else { "false" }),
    ];

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Host", HeaderValue::from_str(host).unwrap());
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Safari/605.1.15"));
    headers.insert("Accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate"));
    headers.insert("Accept-Language", HeaderValue::from_static("zh-CN,zh;q=0.9"));
    headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"));
    headers.insert("Referer", HeaderValue::from_static("http://123.123.123.123/"));
    headers.insert("Origin", HeaderValue::from_str(&format!("http://{}", host)).unwrap());

    let resp = client.post(&authentication_url)
        .headers(headers)
        .form(&post_data)
        .send();

    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to login: {}", e);
            return false;
        }
    };

    let body = match resp.text() {
        Ok(b) => b,
        Err(e) => {
            error!("Failed to read response body: {}", e);
            return false;
        }
    };

    if body.contains("success") {
        info!("Login success");
        true
    } else {
        let result_json: Value = match serde_json::from_str(&body) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to unmarshal JSON: {}", e);
                return false;
            }
        };

        let message = match result_json.get("message") {
            Some(Value::String(s)) => s,
            _ => {
                error!("Fail reason message is missing");
                ""
            }
        };

        error!("Login failed: reason {}", message);
        false
    }
}

fn verify_network_status(verification_url: &Url) -> (bool, String) {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let resp = client.get(verification_url.as_str())
        .send();

    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to verify network status: {}", e);
            return (false, String::new());
        }
    };

    let body = match resp.text() {
        Ok(b) => b,
        Err(e) => {
            error!("Failed to read response body: {}", e);
            return (false, String::new());
        }
    };

    if body.contains("<script>") {
        warn!("Oops, Network connection is down!!!");
        (false, body)
    } else {
        info!("Network connection is fine");
        (true, String::new())
    }
}

fn get_redirect_url(redirect_html_string: &str) -> Option<Url> {
    let start = match redirect_html_string.find("href='") {
        Some(i) => i + "href='".len(),
        None => {
            error!("href not found in the HTML string");
            return None;
        }
    };
    let end = match redirect_html_string[start..].find("'") {
        Some(i) => start + i,
        None => {
            error!("Referer end delimiter not found in the HTML string");
            return None;
        }
    };
    let redirect_url_string = &redirect_html_string[start..end];
    let redirect_url = match Url::parse(redirect_url_string) {
        Ok(u) => u,
        Err(e) => {
            error!("Failed to parse redirect URL: {}", e);
            return None;
        }
    };
    Some(redirect_url)
}

fn encrypt_password(redirect_url: &Url, password: &str) -> String {
    let host = redirect_url.host_str().unwrap_or("");
    let page_info_url = format!("http://{}/eportal/InterFace.do?method=pageInfo&queryString=undefined", host);

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Host", HeaderValue::from_str(host).unwrap());
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Safari/605.1.15"));
    headers.insert("Accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate"));
    headers.insert("Accept-Language", HeaderValue::from_static("zh-CN,zh;q=0.9"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json; charset=UTF-8"));
    headers.insert("Referer", HeaderValue::from_static("http://123.123.123.123/"));
    headers.insert("Origin", HeaderValue::from_str(&format!("http://{}", host)).unwrap());

    let resp = client.get(&page_info_url)
        .headers(headers)
        .send();

    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to get page info: {}", e);
            return String::new();
        }
    };

    let body = match resp.text() {
        Ok(b) => b,
        Err(e) => {
            error!("Failed to read response body: {}", e);
            return String::new();
        }
    };

    let result_json: Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to unmarshal JSON: {}", e);
            return String::new();
        }
    };

    let public_key_exponent = match result_json.get("publicKeyExponent") {
        Some(Value::String(s)) => s,
        _ => {
            error!("publicKeyExponent is not a string or is missing");
            return String::new();
        }
    };

    let public_key_modulus = match result_json.get("publicKeyModulus") {
        Some(Value::String(s)) => s,
        _ => {
            error!("publicKeyModulus is not a string or is missing");
            return String::new();
        }
    };

    let rsa_e = match BigInt::parse_bytes(public_key_exponent.as_bytes(), 16) {
        Some(b) => b,
        None => {
            error!("Failed to parse publicKeyExponent");
            return String::new();
        }
    };

    let rsa_n = match BigInt::parse_bytes(public_key_modulus.as_bytes(), 16) {
        Some(b) => b,
        None => {
            error!("Failed to parse publicKeyModulus");
            return String::new();
        }
    };

    let mac_addr = redirect_url.query_pairs().find(|(k, _)| k == "mac").map(|(_, v)| v).unwrap_or_else(|| "".into());
    let secret = format!("{}>{}", password, mac_addr);
    let secret_bytes = secret.as_bytes();
    let secret_int = BigInt::from_bytes_be(Sign::Plus, secret_bytes);

    let encrypted_big_int = secret_int.modpow(&rsa_e, &rsa_n);

    let (_, encrypted_bytes) = encrypted_big_int.to_bytes_be();
    hex::encode(encrypted_bytes)
}
