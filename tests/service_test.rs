use ykls::service::{
    get_cookie, get_playlist, get_playlist_info, get_recommand_songs, get_song_url, send_captcha,
};

#[tokio::test]
async fn test_send_captcha() {
    let res = send_captcha("xxxxxxxx").await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_get_cookie() {
    let res = get_cookie("xxxxxx", "xxxx").await;
    assert_ne!(res.is_ok(), true);
}

#[tokio::test]
async fn test_get_playlist() {
    let res = get_playlist("3291841711").await.unwrap();
    assert!(!res.is_empty())
}

#[tokio::test]
async fn test_get_playlist_info() {
    let res = get_playlist_info("12538996363").await.unwrap();
    assert_eq!(res[0].id, 1294899616);
}

#[tokio::test]
async fn test_get_song_url() {
    let res = get_song_url(1294899616).await.unwrap();
    assert_ne!(res, String::from(""))
}

#[tokio::test]
async fn test_get_recommand_songs() {
    let res = get_recommand_songs("").await.unwrap();
    assert_ne!(res[0].id, 509106775);
}
