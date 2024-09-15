use crate::service;

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
        let r: bool = service::send_captcha(&self.phone).await?;

        Ok(r)
    }

    pub async fn login(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let cookie: String = service::get_cookie(&self.phone, &self.captcha).await?;

        self.cookie = cookie;

        Ok(())
    }
}
