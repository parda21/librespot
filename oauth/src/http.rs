//! oauth2's HTTP client, on reqwest 0.13.

use oauth2::{HttpRequest, HttpResponse};
use reqwest::{StatusCode, header::HeaderMap, redirect::Policy};

// No redirects, as oauth2 asks of its clients.
pub(crate) fn send(request: HttpRequest) -> Result<HttpResponse, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .redirect(Policy::none())
        .build()?;
    let response = client.execute(request.try_into()?)?;
    let (status, headers) = (response.status(), response.headers().clone());
    Ok(into_response(status, headers, response.bytes()?.to_vec()))
}

pub(crate) async fn send_async(request: HttpRequest) -> Result<HttpResponse, reqwest::Error> {
    let client = reqwest::Client::builder()
        .redirect(Policy::none())
        .build()?;
    let response = client.execute(request.try_into()?).await?;
    let (status, headers) = (response.status(), response.headers().clone());
    Ok(into_response(
        status,
        headers,
        response.bytes().await?.to_vec(),
    ))
}

fn into_response(status: StatusCode, headers: HeaderMap, body: Vec<u8>) -> HttpResponse {
    let mut response = HttpResponse::new(body);
    *response.status_mut() = status;
    *response.headers_mut() = headers;
    response
}
