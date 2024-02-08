mod tokeniser;
mod expr;
mod parser;
mod api_definition;
mod http_request;
mod api_request_route_resolver;
mod resolved_variables;
mod worker_request_executor;
mod worker;
mod app_config;
mod worker_request;
mod evaluator;
mod value_typed;
mod worker_bridge_reponse;
mod api;
mod register;

pub trait UriBackConversion {
    fn as_http_02(&self) -> http_02::Uri;
}

impl UriBackConversion for http::Uri {
    fn as_http_02(&self) -> http_02::Uri {
        self.to_string().parse().unwrap()
    }
}
