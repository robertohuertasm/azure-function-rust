use azure_functions::func;
use azure_functions::bindings::{HttpRequest, HttpResponse};
use serde_derive::Deserialize;

// Struct that will hold information about the body of the request.
#[derive(Deserialize)]
pub struct Body {
  name: String,
}

// The func attribute marks this fn as the function to be used by Azure Functions.
#[func]
// See https://docs.microsoft.com/en-us/azure/azure-functions/functions-triggers-bindings#supported-bindings
#[binding(name="request", auth_level="anonymous")]
// The function will just check for a name parameter in the querystring
// or for a JSON Body structure in the body of the request.
pub fn hello(request: &HttpRequest) -> HttpResponse {
  // Logs the request on the Azure Functions Host.
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
