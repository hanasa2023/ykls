use std::collections::HashMap;

use reqwest::{Response, StatusCode};

use crate::api::login;
use crate::model;

pub enum CurrentScreen {
    Splash,
    Main,
}

pub struct App {
    pub phone: String,
    pub captcha: String,
    pub cookie: String,
}

impl App {
    pub fn new() -> App {
        App {
            phone: String::new(),
            captcha: String::new(),
            cookie: String::new(),
        }
    }

    pub async fn send_captcha(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let r: Response = reqwest::get(login::get_captcha_endpoint(&self.phone)).await?;
        if r.status() == StatusCode::OK {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let res = reqwest::get(login::get_login_endpoint(&self.phone, &self.captcha))
            .await?
            .json::<model::person_info::PersonInfo>()
            .await?;

        self.cookie = res.cookie;

        Ok(())
    }
}
