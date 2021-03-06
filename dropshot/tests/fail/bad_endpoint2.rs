// Copyright 2020 Oxide Computer Company

use dropshot::endpoint;
use dropshot::HttpError;
use dropshot::HttpResponseOk;

#[endpoint {
    method = GET,
    path = "/test",
}]
async fn bad_endpoint(self) -> Result<HttpResponseOk<()>, HttpError> {
    Ok(HttpResponseOk(()))
}

fn main() {}
