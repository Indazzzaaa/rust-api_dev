use crate::{ ctx::Ctx, Error, Result };
use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{ request::Parts, Request },
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::web::ATUH_TOKEN;

pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

// region: -- Ctx Extractor
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        // User the cookies extractor.
        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(ATUH_TOKEN).map(|c| c.value().to_string());

        // parse token
        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;

        // Todo : Token components validation.
        Ok(Ctx::new(user_id))
    }
}

// endregion: -- Ctx Extractor

/// Parse a toekn of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token).ok_or(
        Error::AuthFailTokenWrongFormat
    )?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
