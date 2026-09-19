use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ApiRequest {
    pub extension_id: String,
    pub api: String,
    pub action: String,
    pub payload: String,
}

#[derive(Debug, Clone)]
pub struct ApiResponse {
    pub success: bool,
    pub data: String,
}

pub struct ApiBridge {
    handlers: HashMap<String, fn(ApiRequest) -> ApiResponse>,
}

impl ApiBridge {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        api: &str,
        handler: fn(ApiRequest) -> ApiResponse,
    ) {
        self.handlers.insert(
            api.to_string(),
            handler,
        );
    }

    pub fn execute(
        &self,
        request: ApiRequest,
    ) -> ApiResponse {

        if let Some(handler) =
            self.handlers.get(&request.api)
        {
            return handler(request);
        }

        ApiResponse {
            success: false,
            data: "API not found".into(),
        }
    }
}
