use crate::config::BASE_URL;

pub fn get_captcha_endpoint(phone: &str) -> String {
    format!("{}/captcha/sent?phone={}", BASE_URL, phone)
}

pub fn get_login_endpoint(phone: &str, captcha: &str) -> String {
    format!(
        "{}/login/cellphone?phone={}&captcha={}",
        BASE_URL, phone, captcha
    )
}
