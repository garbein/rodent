extern crate reqwest;

use log::{info};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use crate::utils::crypt;

pub async fn send_sms(phone:&str, content:&str) -> Result<(),reqwest::Error> {
    let sms_account = std::env::var("SMS_ACCOUT").unwrap();
    let sms_token = crypt::md5(std::env::var("SMS_PASSWORD").unwrap());
    let smsapi = "http://api.smsbao.com/sms"; //短信网关

    const QUERY: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');
    let sendurl = format!("{}?u={}&p={}&m={}&c={}", smsapi, sms_account, sms_token, phone, utf8_percent_encode(content, QUERY));

    let resp = reqwest::get(&sendurl).await?;

    info!("{:#?}", resp);
    Ok(())
}