use reqwest::Client;
use serde::Deserialize;

#[derive(Debug)]
pub struct MinecraftAuth {
    pub access_token: String,
    pub expires_in: u32,
    pub name: String,
    pub uuid: String,
    pub skins: Vec<Skins>,
    pub capes: Vec<Capes>
}

#[tracing::instrument(skip(access_token))]
pub async fn auth_minecraft(access_token: &str) -> crate::Result<MinecraftAuth> {

    let client = Client::new();

    let xbl_auth_token = get_xbl_token(&client, access_token).await?;
    let xsts_auth_token = get_xsts_token(&client, &xbl_auth_token.token).await?;
    let minecraft_auth_token = get_minecraft_auth_token(&client, &xsts_auth_token.display_claims.xui[0].uhs, &xsts_auth_token.token).await?;
    let minecraft_buy = check_minecraft_store(&client, &minecraft_auth_token.access_token).await?;

    if !minecraft_buy {
        return Err(crate::ErrorKind::MinecraftAuthError("你沒有購買《Minecraft》！ 請使用其他 Microsoft 帳戶或購買 Minecraft。".to_owned()).as_error());
    }

    let minecraft_profile = get_minecraft_profile(&client, &minecraft_auth_token.access_token).await?;

    Ok(MinecraftAuth {
        access_token: minecraft_auth_token.access_token,
        expires_in: minecraft_auth_token.expires_in,
        name: minecraft_profile.name,
        uuid: minecraft_profile.id,
        skins: minecraft_profile.skins,
        capes: minecraft_profile.capes
    })
}

#[derive(Debug, Deserialize)]
pub struct Skins {
    pub id: String,
    pub state: String,
    #[serde(rename = "textureKey")]
    pub texture_key: String,
    pub url: String,
    pub variant: String,
}

#[derive(Debug, Deserialize)]
pub struct Capes {
    pub alias: String,
    pub id: String,
    pub state: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
struct MinecraftProfile {
    id: String,
    name: String,
    #[serde(rename = "profileActions")]
    profile_actions: serde_json::Value,
    skins: Vec<Skins>,
    capes: Vec<Capes>
}

async fn get_minecraft_profile(client: &Client, access_token: &str) -> crate::Result<MinecraftProfile> {

    let minecraft_profile_response = client
        .get("https://api.minecraftservices.com/minecraft/profile")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if minecraft_profile_response.status().is_success() {
        let text = minecraft_profile_response.text().await?;
        let minecraft_profile = serde_json::from_str::<MinecraftProfile>(&text)?;
        return Ok(minecraft_profile);
    }

    Err(crate::ErrorKind::MinecraftAuthError("Get minecraft profile error".to_owned()).as_error())
}

#[derive(Debug, Deserialize)]
struct CheckGameOwnershipItem {
    name: String,
    signature: String
}

#[derive(Debug, Deserialize)]
struct CheckGameOwnership {
    items: Vec<CheckGameOwnershipItem>
}

#[tracing::instrument(skip(client, access_token))]
async fn check_minecraft_store(client: &Client, access_token: &str) -> crate::Result<bool> {

    let check_game_ownership_response = client
        .get("https://api.minecraftservices.com/entitlements/mcstore")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if check_game_ownership_response.status().is_success() {

        let text = check_game_ownership_response.text().await?;
        let check_game_ownership = serde_json::from_str::<CheckGameOwnership>(&text)?;

        for item in check_game_ownership.items.iter() {
            if item.name == "product_minecraft" {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

#[derive(Debug, Deserialize)]
struct MinecraftAuthToken {
    username: String,
    roles: Vec<serde_json::Value>,
    access_token: String,
    token_type: String,
    expires_in: u32
}

#[tracing::instrument(skip(client, xsts_token))]
async fn get_minecraft_auth_token(client: &Client, uhs: &str, xsts_token: &str) -> crate::Result<MinecraftAuthToken> {

    let payload = serde_json::json!({
        "identityToken": format!("XBL3.0 x={};{}", uhs, xsts_token)
    });

    let minecraft_response = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .json(&payload)
        .send()
        .await?;

    if minecraft_response.status().is_success() {
        let text = minecraft_response.text().await?;
        let minecraft_auth = serde_json::from_str::<MinecraftAuthToken>(&text)?;
        return Ok(minecraft_auth);
    }
    
    Err(crate::ErrorKind::MinecraftAuthError("Get minecraft auth token error.".to_owned()).as_error())
}

#[derive(Debug, Deserialize)]
struct Xui {
    uhs: String
}

#[derive(Debug, Deserialize)]
struct DisplayClaims {
    xui: Vec<Xui>
}

#[derive(Debug, Deserialize)]
struct XSTSXBLAuth {
    #[serde(rename = "IssueInstant")]
    issue_instant: String,
    #[serde(rename = "NotAfter")]
    not_after: String,
    #[serde(rename = "Token")]
    token: String,
    #[serde(rename = "DisplayClaims")]
    display_claims: DisplayClaims,
}

#[tracing::instrument(skip(client, access_token))]
async fn get_xbl_token(client: &Client, access_token: &str) -> crate::Result<XSTSXBLAuth> {

    let payload = serde_json::json!({
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT",
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": format!("d={}", access_token)
        }
    });

    let xbl_response = client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .json(&payload)
        .send()
        .await?;

    if xbl_response.status().is_success() {
        let text = xbl_response.text().await?;
        let xbl_auth = serde_json::from_str::<XSTSXBLAuth>(&text)?;
        return Ok(xbl_auth);
    }

    Err(crate::ErrorKind::MinecraftAuthError("Get xbl token error.".to_owned()).as_error())
}

#[derive(Debug, Deserialize)]
struct XSTSAuthError {
    #[serde(rename = "Identity")]
    identity: String,
    #[serde(rename = "XErr")]
    x_err: u32,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Redirect")]
    redirect: String,
}

#[tracing::instrument(skip(client, token))]
async fn get_xsts_token(client: &Client, token: &str) -> crate::Result<XSTSXBLAuth> {

    let payload = serde_json::json!({
        "RelyingParty": "rp://api.minecraftservices.com/",
        "TokenType": "JWT",
        "Properties": {
            "SandboxId": "RETAIL",
            "UserTokens": [
                token
            ]
        }
    });

    let xsts_response = client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .json(&payload)
        .send()
        .await?;

    if xsts_response.status().is_success() {
        let text = xsts_response.text().await?;
        let xsts_auth = serde_json::from_str::<XSTSXBLAuth>(&text)?;
        return Ok(xsts_auth);
    }

    let text = xsts_response.text().await?;
    let xsts_auth_error = serde_json::from_str::<XSTSAuthError>(&text)?;

    match xsts_auth_error.x_err {
        2148916233 => {
            return Err(crate::ErrorKind::MinecraftAuthError("您的 Microsoft 帳戶未連線至 Xbox 帳戶。請創建一個。".to_owned()).as_error());
        },
        2148916238 => {
            return Err(crate::ErrorKind::MinecraftAuthError("由於您尚未滿 18 歲，成年人必須將您添加到家庭中才能使用".to_owned()).as_error());
        },
        _ => {
            return Err(crate::ErrorKind::MinecraftAuthError(text).as_error());
        }
    }
}