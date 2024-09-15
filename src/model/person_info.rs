use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonInfo {
    #[serde(rename = "loginType")]
    pub login_type: u8,
    pub code: u16,
    pub account: Account,
    pub token: String,
    pub profile: Profile,
    pub bindings: Vec<Binding>,
    pub cookie: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: u64,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "type")]
    pub types: u8,
    pub status: u8,
    #[serde(rename = "whitelistAuthority")]
    pub whitelist_authority: u8,
    #[serde(rename = "createTime")]
    pub create_time: u64,
    pub salt: String,
    #[serde(rename = "tokenVersion")]
    pub token_version: u8,
    pub ban: u8,
    #[serde(rename = "baoyueVersion")]
    pub baoyue_version: i8,
    #[serde(rename = "donateVersion")]
    pub donate_version: i8,
    #[serde(rename = "vipType")]
    pub vip_type: u8,
    #[serde(rename = "viptypeVersion")]
    pub viptype_version: u64,
    #[serde(rename = "anonimousUser")]
    pub anonimous_user: bool,
    pub uninitialized: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
    #[serde(rename = "vipType")]
    pub vip_type: u8,
    #[serde(rename = "authStatus")]
    pub auth_status: u8,
    #[serde(rename = "djStatus")]
    pub dj_status: u8,
    #[serde(rename = "detailDescription")]
    pub detail_description: String,
    pub experts: Experts,
    #[serde(rename = "experTages")]
    pub exper_tages: Option<String>,
    #[serde(rename = "accountStatus")]
    pub account_status: u8,
    pub nickname: String,
    pub birthday: u64,
    pub gender: u8,
    pub province: u32,
    pub city: u32,
    #[serde(rename = "avatarImgId")]
    pub avatar_img_id: u64,
    #[serde(rename = "backgroundImgId")]
    pub background_img_id: u64,
    #[serde(rename = "userType")]
    pub user_type: u8,
    #[serde(rename = "defaultAvatar")]
    pub default_avatar: bool,
    pub mutual: bool,
    #[serde(rename = "remarkName")]
    pub remark_name: Option<String>,
    pub followed: bool,
    #[serde(rename = "backgroundUrl")]
    pub background_url: String,
    #[serde(rename = "avatarImgIdStr")]
    pub avatar_img_id_str: String,
    #[serde(rename = "backgroundImgIdStr")]
    pub background_img_id_str: String,
    pub description: String,
    #[serde(rename = "userId")]
    pub user_id: u32,
    pub signature: String,
    pub authority: u8,
    pub followeds: u32,
    #[serde(rename = "eventCount")]
    pub event_count: u32,
    #[serde(rename = "avatarDetail")]
    pub avatar_detail: Option<String>,
    #[serde(rename = "playlistCount")]
    pub playlist_count: u32,
    #[serde(rename = "playlistBeSubscribedCount")]
    pub playlst_be_subscribed_count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Experts {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Binding {
    #[serde(rename = "bindingTime")]
    pub binding_time: u64,
    #[serde(rename = "refreshTime")]
    pub refresh_time: u64,
    #[serde(rename = "expiresIn")]
    pub expires_in: u64,
    #[serde(rename = "tokenJsonStr")]
    pub token_json_str: String,
    pub url: String,
    pub expired: bool,
    #[serde(rename = "userId")]
    pub user_id: u64,
    pub id: u64,
    #[serde(rename = "type")]
    pub types: u8,
}
