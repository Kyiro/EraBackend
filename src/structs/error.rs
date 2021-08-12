use serde::{Serialize};

#[derive(Clone, Serialize)]
pub struct EpicError {
    #[serde(rename = "errorCode")]
    error_code: String,
    
    #[serde(rename = "errorMessage")]
    error_message: String,
    
    #[serde(rename = "messageVars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    message_vars: Option<Vec<String>>,
    
    #[serde(rename = "numericErrorCode")]
    numeric_error_code: i32,
    
    #[serde(rename = "originatingService")]
    originating_service: String,
    
    intent: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    error_description: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>
}

impl EpicError {
    pub fn new() -> Self {
        Self {
            error_code: String::new(),
            error_message: String::new(),
            message_vars: None,
            numeric_error_code: 0,
            originating_service: String::new(),
            intent: String::from("prod"),
            error_description: None,
            error: None
        }
    }
    
    pub fn error_code(&mut self, msg: &str) -> Self {
        self.error_code = String::from(msg);
        self.clone()
    }
    
    pub fn error_message(&mut self, msg: &str) -> Self {
        self.error_message = String::from(msg);
        self.clone()
    }
    
    pub fn message_vars(&mut self, msg: Vec<String>) -> Self {
        self.message_vars = Some(msg);
        self.clone()
    }
    
    pub fn numeric_code(&mut self, msg: i32) -> Self {
        self.numeric_error_code = msg;
        self.clone()
    }
    
    pub fn service(&mut self, msg: &str) -> Self {
        self.originating_service = String::from(msg);
        self.clone()
    }
    
    pub fn intent(&mut self, msg: &str) -> Self {
        self.intent = String::from(msg);
        self.clone()
    }
    
    pub fn description(&mut self) -> Self {
        self.error_description = Some(self.error_code.clone());
        self.clone()
    }
    
    pub fn error(&mut self, msg: &str) -> Self {
        self.error = Some(String::from(msg));
        self.clone()
    }
}