use app::App;

mod api;
mod app;
mod config;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app: App = App::new();
    app.phone = String::from("13803590508");
    let c = app.send_captcha().await?;
    println!("{}", c);
    app.login().await?;
    println!("{}", app.cookie);

    Ok(())
}
