use worker::js_sys::{ArrayBuffer, Uint8Array};
use worker::wasm_bindgen::JsCast;
use worker::wasm_bindgen_futures::JsFuture;
use worker::worker_sys::{
    WorkerGlobalScope, Response as JsResponse, Request as JsRequest, RequestInit as JsRequestInit, Headers as JsHeaders
};
use worker::*;

pub async fn fetch_by_url(_: Request, _: RouteContext<()>) -> Result<Response> {
    let global: WorkerGlobalScope = js_sys::global().unchecked_into();

    // First we need to call the global fetch function and await it's promise
    let resp_promise = global.fetch_with_str("https://www.cloudflare.com/ips-v4");
    let resp = JsFuture::from(resp_promise).await?;
    let resp: JsResponse = resp.into();

    // Next we need to get the bytes of the response body, which we have to do through a Uint8Array.
    let bytes_promise = resp.array_buffer()?;
    let bytes = JsFuture::from(bytes_promise).await?;
    let array_buffer: ArrayBuffer = bytes.unchecked_into();
    let uint8_array: Uint8Array = Uint8Array::new(&array_buffer);

    // Now we can get those bytes as a traditional Rust `Vec` and return them in our response.
    let rust_bytes = uint8_array.to_vec();
    Response::from_bytes(rust_bytes)
}

pub async fn fetch_by_request(_: Request, _: RouteContext<()>) -> Result<Response> {
    let global: WorkerGlobalScope = js_sys::global().unchecked_into();

    // Because we're fetching by passing a request object to fetch, we need to configure what our
    // request looks like, in this case just a simple URL.
    let request = JsRequest::new_with_str("https://www.cloudflare.com/ips-v4")?;

    // First we need to call the global fetch function and await it's promise
    let resp_promise = global.fetch_with_request(&request);
    let resp = JsFuture::from(resp_promise).await?;
    let resp: JsResponse = resp.into();

    // Next we need to get the bytes of the response body, which we have to do through a
    // Uint8Array.
    let bytes_promise = resp.array_buffer()?;
    let bytes = JsFuture::from(bytes_promise).await?;
    let array_buffer: ArrayBuffer = bytes.unchecked_into();
    let uint8_array: Uint8Array = Uint8Array::new(&array_buffer);

    // Now we can get those bytes as a traditional Rust `Vec` and return them in our response.
    let rust_bytes = uint8_array.to_vec();
    Response::from_bytes(rust_bytes)
}

pub async fn fetch_by_request_with_init(_: Request, _: RouteContext<()>) -> Result<Response> {
    let global: WorkerGlobalScope = js_sys::global().unchecked_into();

    let headers = JsHeaders::new()?;
    headers.append("example-header", "example-value")?;

    let mut init = JsRequestInit::new();

    init.method("GET");
    init.redirect(worker_sys::RequestRedirect::Follow);
    init.headers(&headers);

    // Because we're fetching by passing a request object to fetch, we need to configure what our
    // request looks like, in this case just a simple URL.
    let request = JsRequest::new_with_str_and_init("https://www.cloudflare.com/ips-v4", &init)?;

    // First we need to call the global fetch function and await it's promise
    let resp_promise = global.fetch_with_request(&request);
    let resp = JsFuture::from(resp_promise).await?;
    let resp: JsResponse = resp.into();

    // Next we need to get the bytes of the response body, which we have to do through a
    // Uint8Array.
    let bytes_promise = resp.array_buffer()?;
    let bytes = JsFuture::from(bytes_promise).await?;
    let array_buffer: ArrayBuffer = bytes.unchecked_into();
    let uint8_array: Uint8Array = Uint8Array::new(&array_buffer);

    // Now we can get those bytes as a traditional Rust `Vec` and return them in our response.
    let rust_bytes = uint8_array.to_vec();
    Response::from_bytes(rust_bytes)
}