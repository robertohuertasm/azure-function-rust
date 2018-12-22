use azure_functions::func;
use azure_functions::bindings::{HttpRequest, HttpResponse};
use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Body {
  name: String,
}


#[func]
#[binding(name="request", auth_level="anonymous")]
pub fn hello(request: &HttpRequest) -> HttpResponse {
  info!("Request: {:?}", request);

  // checking the query string
  if let Some(name) = request.query_params().get("name") {
    return format!("Hello from Rust, my dear {}! (qs)", name).into();
  }

  // checking the body
  if let Ok(body) = request.body().as_json::<Body>() {
    return format!("Hello from Rust, my dear {}! (body)", body.name).into();
  }

  "Hello from Rust, my dear default user with no params!".into()
}