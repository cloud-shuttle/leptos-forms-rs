# Form Submission Design

## 🎯 **Overview**

Form submission is currently a fake implementation that just returns values without actually submitting data. This design document outlines how to implement real form submission functionality with proper error handling, success callbacks, and network integration.

## 🚨 **Current State**

### **What's Broken**

```rust
// In a real implementation, you would send the data here
// For now, just return the values
Ok(new_state.values)  // Just returns values, doesn't actually submit!
```

### **Impact**

- Forms don't actually submit data anywhere
- No network integration
- No error handling for network failures
- No success/failure callbacks
- Forms are essentially non-functional in real applications

## 🎯 **Design Goals**

1. **Real Submission**: Actually submit data to endpoints
2. **Error Handling**: Proper error handling for network failures
3. **Success Callbacks**: Callback system for success/failure
4. **Loading States**: Proper loading state management
5. **Retry Logic**: Automatic retry for failed submissions
6. **Validation**: Pre-submission validation
7. **Security**: Secure data transmission

## 🏗️ **Architecture**

### **Core Components**

1. **Submission Engine**: Core submission logic
2. **HTTP Client**: Network communication
3. **Error Handling**: Error management and recovery
4. **State Management**: Loading and submission states
5. **Callback System**: Success/failure callbacks

### **Data Flow**

```
Form Data -> Validation -> Submission Engine -> HTTP Client -> Server
     ^                                                           |
     |                                                           v
Form State <-- Callback System <-- Response Handler <-- Network Response
```

## 📋 **Implementation Plan**

### **Phase 1: Core Submission Engine (Week 1)**

#### **1.1 Submission Configuration**

```rust
// leptos-forms-rs/src/core/types/submission_config.rs
#[derive(Debug, Clone)]
pub struct SubmissionConfig {
    pub endpoint: String,
    pub method: HttpMethod,
    pub headers: std::collections::HashMap<String, String>,
    pub timeout: Option<std::time::Duration>,
    pub retry_attempts: u32,
    pub retry_delay: std::time::Duration,
}

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl Default for SubmissionConfig {
    fn default() -> Self {
        Self {
            endpoint: String::new(),
            method: HttpMethod::POST,
            headers: std::collections::HashMap::new(),
            timeout: Some(std::time::Duration::from_secs(30)),
            retry_attempts: 3,
            retry_delay: std::time::Duration::from_secs(1),
        }
    }
}
```

#### **1.2 Submission Result Types**

```rust
// leptos-forms-rs/src/core/types/submission_result.rs
#[derive(Debug, Clone)]
pub struct SubmissionResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<SubmissionError>,
    pub response_code: Option<u16>,
    pub response_headers: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum SubmissionError {
    NetworkError(String),
    TimeoutError,
    ValidationError(ValidationErrors),
    ServerError(u16, String),
    ParseError(String),
    RetryExhausted,
}

impl<T> SubmissionResult<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            response_code: Some(200),
            response_headers: std::collections::HashMap::new(),
        }
    }

    pub fn error(error: SubmissionError) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            response_code: None,
            response_headers: std::collections::HashMap::new(),
        }
    }
}
```

### **Phase 2: HTTP Client Implementation (Week 2)**

#### **2.1 HTTP Client Trait**

```rust
// leptos-forms-rs/src/core/http_client.rs
use async_trait::async_trait;

#[async_trait]
pub trait HttpClient {
    async fn submit<T: Serialize + Send + Sync>(
        &self,
        config: &SubmissionConfig,
        data: &T,
    ) -> Result<SubmissionResult<serde_json::Value>, SubmissionError>;

    async fn submit_raw(
        &self,
        config: &SubmissionConfig,
        data: &[u8],
    ) -> Result<SubmissionResult<Vec<u8>>, SubmissionError>;
}

// WASM implementation using web_sys
#[cfg(target_arch = "wasm32")]
pub struct WasmHttpClient;

#[cfg(target_arch = "wasm32")]
#[async_trait]
impl HttpClient for WasmHttpClient {
    async fn submit<T: Serialize + Send + Sync>(
        &self,
        config: &SubmissionConfig,
        data: &T,
    ) -> Result<SubmissionResult<serde_json::Value>, SubmissionError> {
        use web_sys::{Request, RequestInit, RequestMode, RequestCredentials};
        use wasm_bindgen::JsValue;
        use wasm_bindgen_futures::JsFuture;

        let mut request_init = RequestInit::new();
        request_init.method(&config.method.to_string());
        request_init.mode(RequestMode::Cors);
        request_init.credentials(RequestCredentials::SameOrigin);

        // Set headers
        for (key, value) in &config.headers {
            request_init.headers().set(key, value).unwrap();
        }

        // Serialize data
        let json_data = serde_json::to_string(data)
            .map_err(|e| SubmissionError::ParseError(e.to_string()))?;

        request_init.body(Some(&JsValue::from_str(&json_data)));

        // Create request
        let request = Request::new_with_str_and_init(&config.endpoint, &request_init)
            .map_err(|e| SubmissionError::NetworkError(format!("Failed to create request: {:?}", e)))?;

        // Send request
        let window = web_sys::window().unwrap();
        let response = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| SubmissionError::NetworkError(format!("Network error: {:?}", e)))?;

        let response: web_sys::Response = response.dyn_into()
            .map_err(|e| SubmissionError::NetworkError(format!("Invalid response: {:?}", e)))?;

        // Check status
        let status = response.status();
        if !response.ok() {
            let error_text = JsFuture::from(response.text())
                .await
                .map_err(|e| SubmissionError::NetworkError(format!("Failed to read error: {:?}", e)))?
                .as_string()
                .unwrap_or_default();

            return Ok(SubmissionResult::error(SubmissionError::ServerError(status, error_text)));
        }

        // Parse response
        let response_text = JsFuture::from(response.text())
            .await
            .map_err(|e| SubmissionError::NetworkError(format!("Failed to read response: {:?}", e)))?
            .as_string()
            .unwrap_or_default();

        let response_data: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|e| SubmissionError::ParseError(e.to_string()))?;

        Ok(SubmissionResult::success(response_data))
    }

    async fn submit_raw(
        &self,
        config: &SubmissionConfig,
        data: &[u8],
    ) -> Result<SubmissionResult<Vec<u8>>, SubmissionError> {
        // Implementation for raw data submission
        todo!("Implement raw data submission")
    }
}

// Native implementation using reqwest
#[cfg(not(target_arch = "wasm32"))]
pub struct NativeHttpClient;

#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
impl HttpClient for NativeHttpClient {
    async fn submit<T: Serialize + Send + Sync>(
        &self,
        config: &SubmissionConfig,
        data: &T,
    ) -> Result<SubmissionResult<serde_json::Value>, SubmissionError> {
        use reqwest::Client;

        let client = Client::new();
        let mut request = match config.method {
            HttpMethod::GET => client.get(&config.endpoint),
            HttpMethod::POST => client.post(&config.endpoint),
            HttpMethod::PUT => client.put(&config.endpoint),
            HttpMethod::PATCH => client.patch(&config.endpoint),
            HttpMethod::DELETE => client.delete(&config.endpoint),
        };

        // Set headers
        for (key, value) in &config.headers {
            request = request.header(key, value);
        }

        // Set timeout
        if let Some(timeout) = config.timeout {
            request = request.timeout(timeout);
        }

        // Send request
        let response = request.json(data).send().await
            .map_err(|e| SubmissionError::NetworkError(e.to_string()))?;

        let status = response.status().as_u16();

        if !response.status().is_success() {
            let error_text = response.text().await
                .map_err(|e| SubmissionError::NetworkError(e.to_string()))?;

            return Ok(SubmissionResult::error(SubmissionError::ServerError(status, error_text)));
        }

        let response_data: serde_json::Value = response.json().await
            .map_err(|e| SubmissionError::ParseError(e.to_string()))?;

        Ok(SubmissionResult::success(response_data))
    }

    async fn submit_raw(
        &self,
        config: &SubmissionConfig,
        data: &[u8],
    ) -> Result<SubmissionResult<Vec<u8>>, SubmissionError> {
        // Implementation for raw data submission
        todo!("Implement raw data submission")
    }
}
```

### **Phase 3: Form Handle Integration (Week 3)**

#### **3.1 Enhanced Form Handle**

```rust
// leptos-forms-rs/src/core/form_handle/submission.rs
use crate::core::http_client::{HttpClient, WasmHttpClient, NativeHttpClient};
use crate::core::types::{SubmissionConfig, SubmissionResult, SubmissionError};

impl<T: Form + Send + Sync + PartialEq> FormHandle<T> {
    /// Submit the form with real HTTP request
    pub async fn submit_with_config(
        &self,
        config: SubmissionConfig,
    ) -> Result<SubmissionResult<serde_json::Value>, SubmissionError> {
        // Validate form first
        if let Err(validation_errors) = self.validate() {
            return Ok(SubmissionResult::error(SubmissionError::ValidationError(validation_errors)));
        }

        // Mark as submitting
        self.mark_submitting();

        // Get form data
        let form_data = self.get_current_values();

        // Create HTTP client
        let client: Box<dyn HttpClient> = if cfg!(target_arch = "wasm32") {
            Box::new(WasmHttpClient)
        } else {
            Box::new(NativeHttpClient)
        };

        // Submit with retry logic
        let mut last_error = None;
        for attempt in 0..config.retry_attempts {
            match client.submit(&config, &form_data).await {
                Ok(result) => {
                    // Mark as not submitting
                    self.mark_not_submitting();
                    return Ok(result);
                }
                Err(error) => {
                    last_error = Some(error);

                    // Wait before retry
                    if attempt < config.retry_attempts - 1 {
                        tokio::time::sleep(config.retry_delay).await;
                    }
                }
            }
        }

        // Mark as not submitting
        self.mark_not_submitting();

        // Return last error
        Ok(SubmissionResult::error(
            last_error.unwrap_or(SubmissionError::RetryExhausted)
        ))
    }

    /// Submit the form with default configuration
    pub async fn submit_to_endpoint(
        &self,
        endpoint: &str,
    ) -> Result<SubmissionResult<serde_json::Value>, SubmissionError> {
        let config = SubmissionConfig {
            endpoint: endpoint.to_string(),
            ..Default::default()
        };

        self.submit_with_config(config).await
    }

    /// Submit the form with custom callback
    pub async fn submit_with_callback<F>(
        &self,
        config: SubmissionConfig,
        callback: F,
    ) -> Result<SubmissionResult<serde_json::Value>, SubmissionError>
    where
        F: FnOnce(SubmissionResult<serde_json::Value>) + Send + Sync,
    {
        let result = self.submit_with_config(config).await;
        callback(result.clone());
        result
    }

    /// Mark form as submitting
    pub fn mark_submitting(&self) {
        let state = self.state.get_untracked();
        let new_state = state.mark_submitting();
        self.state.set(new_state);
    }

    /// Mark form as not submitting
    pub fn mark_not_submitting(&self) {
        let state = self.state.get_untracked();
        let new_state = state.mark_not_submitting();
        self.state.set(new_state);
    }
}
```

### **Phase 4: Component Integration (Week 4)**

#### **4.1 Enhanced FormSubmit Component**

```rust
// leptos-forms-rs/src/components/core/form_submit.rs
#[component]
pub fn FormSubmit<T: Form + PartialEq + Clone + Send + Sync>(
    form: FormHandle<T>,
    #[prop(optional)] endpoint: Option<String>,
    #[prop(optional)] config: Option<SubmissionConfig>,
    #[prop(optional)] on_success: Option<Callback<SubmissionResult<serde_json::Value>>>,
    #[prop(optional)] on_error: Option<Callback<SubmissionError>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let form_clone = form.clone();
    let endpoint_clone = endpoint.clone();
    let config_clone = config.clone();
    let on_success_clone = on_success.clone();
    let on_error_clone = on_error.clone();

    let submit_handler = move |_| {
        let form = form_clone.clone();
        let endpoint = endpoint_clone.clone();
        let config = config_clone.clone();
        let on_success = on_success_clone.clone();
        let on_error = on_error_clone.clone();

        spawn_local(async move {
            let result = if let Some(config) = config {
                form.submit_with_config(config).await
            } else if let Some(endpoint) = endpoint {
                form.submit_to_endpoint(&endpoint).await
            } else {
                // Fallback to old behavior
                match form.submit() {
                    Ok(data) => Ok(SubmissionResult::success(serde_json::to_value(data).unwrap())),
                    Err(error) => Ok(SubmissionResult::error(SubmissionError::ValidationError(error))),
                }
            };

            match result {
                Ok(submission_result) => {
                    if submission_result.success {
                        if let Some(callback) = on_success {
                            callback.call(submission_result);
                        }
                    } else {
                        if let Some(callback) = on_error {
                            if let Some(error) = submission_result.error {
                                callback.call(error);
                            }
                        }
                    }
                }
                Err(error) => {
                    if let Some(callback) = on_error {
                        callback.call(error);
                    }
                }
            }
        });
    };

    let is_submitting = form.is_submitting();
    let is_valid = form.is_valid();

    view! {
        <button
            type="button"
            class="form-submit-btn"
            disabled=move || is_submitting.get() || !is_valid.get()
            on:click=submit_handler
        >
            {move || {
                if is_submitting.get() {
                    "Submitting..."
                } else {
                    if let Some(children) = children {
                        children.call(())
                    } else {
                        "Submit"
                    }
                }
            }}
        </button>
    }
}
```

## 🎯 **Usage Examples**

### **Basic Form Submission**

```rust
#[component]
fn LoginForm() -> impl IntoView {
    let (form, _submit_callback, _reset_callback) = use_form(LoginForm::default_values());

    let on_success = Callback::new(|result: SubmissionResult<serde_json::Value>| {
        log::info!("Form submitted successfully: {:?}", result.data);
        // Handle success (redirect, show message, etc.)
    });

    let on_error = Callback::new(|error: SubmissionError| {
        log::error!("Form submission failed: {:?}", error);
        // Handle error (show error message, etc.)
    });

    view! {
        <div class="form">
            <FormField form=form.clone() name="email".to_string() />
            <FormField form=form.clone() name="password".to_string() />

            <FormSubmit
                form=form
                endpoint="/api/login".to_string()
                on_success=on_success
                on_error=on_error
            />
        </div>
    }
}
```

### **Advanced Form Submission**

```rust
#[component]
fn AdvancedForm() -> impl IntoView {
    let (form, _submit_callback, _reset_callback) = use_form(AdvancedForm::default_values());

    let config = SubmissionConfig {
        endpoint: "/api/advanced".to_string(),
        method: HttpMethod::POST,
        headers: {
            let mut headers = std::collections::HashMap::new();
            headers.insert("Content-Type".to_string(), "application/json".to_string());
            headers.insert("Authorization".to_string(), "Bearer token".to_string());
            headers
        },
        timeout: Some(std::time::Duration::from_secs(60)),
        retry_attempts: 5,
        retry_delay: std::time::Duration::from_secs(2),
    };

    view! {
        <div class="form">
            <FormField form=form.clone() name="field1".to_string() />
            <FormField form=form.clone() name="field2".to_string() />

            <FormSubmit
                form=form
                config=config
                on_success=Callback::new(|result| {
                    // Handle success
                })
                on_error=Callback::new(|error| {
                    // Handle error
                })
            />
        </div>
    }
}
```

## 🎯 **Success Criteria**

### **Functional Requirements**

- ✅ Real HTTP submission
- ✅ Proper error handling
- ✅ Success/failure callbacks
- ✅ Loading state management
- ✅ Retry logic
- ✅ Timeout handling
- ✅ Header support

### **Performance Requirements**

- ✅ Submission in < 5 seconds
- ✅ Proper timeout handling
- ✅ Efficient retry logic
- ✅ Memory efficient

### **Quality Requirements**

- ✅ 100% test coverage
- ✅ No panics or crashes
- ✅ Proper error messages
- ✅ Security considerations

## 📊 **Implementation Timeline**

- **Week 1**: Core submission engine
- **Week 2**: HTTP client implementation
- **Week 3**: Form handle integration
- **Week 4**: Component integration and testing

## 🚀 **Next Steps**

1. **Approve Design**: Review and approve this design
2. **Set Up Development**: Create feature branch
3. **Implement Phase 1**: Core submission engine
4. **Implement Phase 2**: HTTP client
5. **Implement Phase 3**: Form handle integration
6. **Implement Phase 4**: Component integration
7. **Testing**: Comprehensive testing
8. **Documentation**: Update docs and examples

This design will transform form submission from a fake implementation to a fully functional, production-ready feature with proper error handling, retry logic, and network integration.
