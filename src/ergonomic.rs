use std::iter::FromIterator;

use worker::*;

pub async fn fetch_by_url(_: Request, _: RouteContext<()>) -> Result<Response> {
    let url = Url::parse("https://www.cloudflare.com/ips-v4").expect("invalid url");
    let mut resp_from_url = Fetch::Url(url).send().await?;

    Response::from_bytes(resp_from_url.bytes().await?)
}

pub async fn fetch_by_request(_: Request, _: RouteContext<()>) -> Result<Response> {
    let request = Request::new("https://www.cloudflare.com/ips-v4", Method::Get)?;
    let mut resp_from_request = Fetch::Request(request).send().await?;

    Response::from_bytes(resp_from_request.bytes().await?)
}

pub async fn fetch_by_request_with_init(_: Request, _: RouteContext<()>) -> Result<Response> {
    let mut request_init = RequestInit::new();

    request_init
        .with_headers(Headers::from_iter([("example-header", "example-value")]))
        .with_method(Method::Get)
        .with_redirect(RequestRedirect::Follow);

    let request = Request::new_with_init("https://www.cloudflare.com/ips-v4", &request_init)?;
    let mut resp_from_init = Fetch::Request(request).send().await?;

    Response::from_bytes(resp_from_init.bytes().await?)
}