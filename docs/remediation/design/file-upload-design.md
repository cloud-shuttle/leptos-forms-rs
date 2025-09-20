# File Upload Component Design

## 🎯 **Overview**

File upload support is currently missing entirely from the library. This design document outlines how to implement comprehensive file upload functionality with validation, progress tracking, and security features.

## 🚨 **Current State**

### **What's Missing**

- No file input components
- No file validation
- No file upload handling
- No progress tracking
- No security features
- No file type restrictions

### **Impact**

- Cannot build forms with file uploads
- Missing essential form functionality
- No way to handle file data
- Security vulnerabilities for file handling

## 🎯 **Design Goals**

1. **File Input Components**: Easy-to-use file input components
2. **File Validation**: Comprehensive file validation (type, size, etc.)
3. **Progress Tracking**: Real-time upload progress
4. **Security**: Secure file handling and validation
5. **Multiple Files**: Support for single and multiple file uploads
6. **Drag & Drop**: Modern drag and drop interface
7. **Preview**: File preview functionality

## 🏗️ **Architecture**

### **Core Components**

1. **FileInput Component**: Basic file input
2. **FileUploadInput Component**: Advanced file upload with progress
3. **FileValidation**: File validation system
4. **FileProgress**: Upload progress tracking
5. **FilePreview**: File preview functionality

### **Data Flow**

```
File Selection -> Validation -> Upload -> Progress Tracking -> Completion
     ^                                                           |
     |                                                           v
File Preview <-- File Data <-- Upload Handler <-- Network Response
```

## 📋 **Implementation Plan**

### **Phase 1: Core File Types (Week 1)**

#### **1.1 File Data Types**

```rust
// leptos-forms-rs/src/core/types/file_data.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileData {
    pub name: String,
    pub size: u64,
    pub mime_type: String,
    pub last_modified: Option<u64>,
    pub data: FileContent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileContent {
    Base64(String),
    BlobUrl(String),
    Binary(Vec<u8>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileConstraints {
    pub max_size: Option<u64>,
    pub min_size: Option<u64>,
    pub allowed_types: Vec<String>,
    pub max_files: Option<usize>,
    pub required: bool,
}

impl Default for FileConstraints {
    fn default() -> Self {
        Self {
            max_size: Some(10 * 1024 * 1024), // 10MB
            min_size: None,
            allowed_types: vec!["*/*".to_string()],
            max_files: Some(1),
            required: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileUploadProgress {
    pub file_name: String,
    pub bytes_uploaded: u64,
    pub total_bytes: u64,
    pub percentage: f64,
    pub status: UploadStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UploadStatus {
    Pending,
    Uploading,
    Completed,
    Failed(String),
    Cancelled,
}

impl FileUploadProgress {
    pub fn new(file_name: String, total_bytes: u64) -> Self {
        Self {
            file_name,
            bytes_uploaded: 0,
            total_bytes,
            percentage: 0.0,
            status: UploadStatus::Pending,
        }
    }

    pub fn update_progress(&mut self, bytes_uploaded: u64) {
        self.bytes_uploaded = bytes_uploaded;
        self.percentage = (bytes_uploaded as f64 / self.total_bytes as f64) * 100.0;
    }

    pub fn mark_completed(&mut self) {
        self.status = UploadStatus::Completed;
        self.bytes_uploaded = self.total_bytes;
        self.percentage = 100.0;
    }

    pub fn mark_failed(&mut self, error: String) {
        self.status = UploadStatus::Failed(error);
    }

    pub fn mark_cancelled(&mut self) {
        self.status = UploadStatus::Cancelled;
    }
}
```

#### **1.2 File Validation**

```rust
// leptos-forms-rs/src/validation/file_validators.rs
use crate::core::types::{FileData, FileConstraints};

pub struct FileValidators;

impl FileValidators {
    pub fn validate_file(file: &FileData, constraints: &FileConstraints) -> Result<(), String> {
        // Check file size
        if let Some(max_size) = constraints.max_size {
            if file.size > max_size {
                return Err(format!("File size exceeds maximum allowed size of {} bytes", max_size));
            }
        }

        if let Some(min_size) = constraints.min_size {
            if file.size < min_size {
                return Err(format!("File size is below minimum required size of {} bytes", min_size));
            }
        }

        // Check file type
        if !constraints.allowed_types.contains(&"*/*".to_string()) {
            let file_type = &file.mime_type;
            let is_allowed = constraints.allowed_types.iter().any(|allowed_type| {
                if allowed_type.ends_with("/*") {
                    let base_type = allowed_type.trim_end_matches("/*");
                    file_type.starts_with(base_type)
                } else {
                    file_type == allowed_type
                }
            });

            if !is_allowed {
                return Err(format!("File type '{}' is not allowed. Allowed types: {:?}", file_type, constraints.allowed_types));
            }
        }

        // Check file name
        if file.name.is_empty() {
            return Err("File name cannot be empty".to_string());
        }

        // Check for dangerous file extensions
        let dangerous_extensions = ["exe", "bat", "cmd", "com", "pif", "scr", "vbs", "js", "jar"];
        if let Some(extension) = file.name.split('.').last() {
            if dangerous_extensions.contains(&extension.to_lowercase().as_str()) {
                return Err(format!("File extension '{}' is not allowed for security reasons", extension));
            }
        }

        Ok(())
    }

    pub fn validate_multiple_files(files: &[FileData], constraints: &FileConstraints) -> Result<(), String> {
        // Check number of files
        if let Some(max_files) = constraints.max_files {
            if files.len() > max_files {
                return Err(format!("Too many files. Maximum allowed: {}", max_files));
            }
        }

        if constraints.required && files.is_empty() {
            return Err("At least one file is required".to_string());
        }

        // Validate each file
        for file in files {
            Self::validate_file(file, constraints)?;
        }

        Ok(())
    }

    pub fn get_file_type_category(mime_type: &str) -> FileTypeCategory {
        if mime_type.starts_with("image/") {
            FileTypeCategory::Image
        } else if mime_type.starts_with("video/") {
            FileTypeCategory::Video
        } else if mime_type.starts_with("audio/") {
            FileTypeCategory::Audio
        } else if mime_type.starts_with("text/") {
            FileTypeCategory::Text
        } else if mime_type.contains("pdf") {
            FileTypeCategory::Document
        } else if mime_type.contains("zip") || mime_type.contains("rar") || mime_type.contains("7z") {
            FileTypeCategory::Archive
        } else {
            FileTypeCategory::Other
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileTypeCategory {
    Image,
    Video,
    Audio,
    Text,
    Document,
    Archive,
    Other,
}
```

### **Phase 2: File Input Components (Week 2)**

#### **2.1 Basic FileInput Component**

```rust
// leptos-forms-rs/src/components/file_input.rs
use leptos::prelude::*;
use web_sys::{HtmlInputElement, FileList};
use wasm_bindgen::JsCast;

#[component]
pub fn FileInput(
    #[prop(into)] name: String,
    #[prop(optional)] multiple: Option<bool>,
    #[prop(optional)] accept: Option<String>,
    #[prop(optional)] constraints: Option<FileConstraints>,
    #[prop(optional)] on_change: Option<Callback<Vec<FileData>>>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let input_ref = NodeRef::<HtmlInputElement>::new();
    let constraints = constraints.unwrap_or_default();

    let handle_change = move |_| {
        if let Some(input) = input_ref.get() {
            if let Some(file_list) = input.files() {
                let files = convert_file_list_to_file_data(&file_list);

                // Validate files
                if let Err(error) = FileValidators::validate_multiple_files(&files, &constraints) {
                    log::error!("File validation failed: {}", error);
                    // Handle validation error
                    return;
                }

                if let Some(callback) = on_change {
                    callback.call(files);
                }
            }
        }
    };

    let input_class = class.unwrap_or_else(|| "file-input".to_string());

    view! {
        <input
            node_ref=input_ref
            type="file"
            name=name
            multiple=multiple.unwrap_or(false)
            accept=accept.unwrap_or_default()
            class=input_class
            on:change=handle_change
        />
    }
}

fn convert_file_list_to_file_data(file_list: &FileList) -> Vec<FileData> {
    let mut files = Vec::new();

    for i in 0..file_list.length() {
        if let Some(file) = file_list.item(i) {
            let file_data = FileData {
                name: file.name(),
                size: file.size() as u64,
                mime_type: file.type_(),
                last_modified: Some(file.last_modified()),
                data: FileContent::BlobUrl(web_sys::Url::create_object_url_with_blob(&file).unwrap()),
            };
            files.push(file_data);
        }
    }

    files
}
```

#### **2.2 Advanced FileUploadInput Component**

```rust
// leptos-forms-rs/src/components/file_upload_input.rs
#[component]
pub fn FileUploadInput(
    #[prop(into)] name: String,
    #[prop(optional)] multiple: Option<bool>,
    #[prop(optional)] accept: Option<String>,
    #[prop(optional)] constraints: Option<FileConstraints>,
    #[prop(optional)] upload_url: Option<String>,
    #[prop(optional)] on_upload_progress: Option<Callback<FileUploadProgress>>,
    #[prop(optional)] on_upload_complete: Option<Callback<Vec<FileData>>>,
    #[prop(optional)] on_upload_error: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let (files, set_files) = create_signal(Vec::<FileData>::new());
    let (upload_progress, set_upload_progress) = create_signal(Vec::<FileUploadProgress>::new());
    let (is_uploading, set_is_uploading) = create_signal(false);

    let constraints = constraints.unwrap_or_default();

    let handle_file_change = move |new_files: Vec<FileData>| {
        set_files.set(new_files);
    };

    let handle_upload = move |_| {
        let files = files.get();
        if files.is_empty() {
            return;
        }

        set_is_uploading.set(true);

        // Initialize progress tracking
        let progress_items: Vec<FileUploadProgress> = files.iter()
            .map(|file| FileUploadProgress::new(file.name.clone(), file.size))
            .collect();
        set_upload_progress.set(progress_items);

        // Start upload process
        if let Some(upload_url) = upload_url.clone() {
            spawn_local(async move {
                let mut completed_files = Vec::new();
                let mut errors = Vec::new();

                for (index, file) in files.iter().enumerate() {
                    match upload_single_file(file, &upload_url, index, set_upload_progress).await {
                        Ok(uploaded_file) => completed_files.push(uploaded_file),
                        Err(error) => errors.push(error),
                    }
                }

                set_is_uploading.set(false);

                if errors.is_empty() {
                    if let Some(callback) = on_upload_complete {
                        callback.call(completed_files);
                    }
                } else {
                    if let Some(callback) = on_upload_error {
                        callback.call(errors.join(", "));
                    }
                }
            });
        }
    };

    let handle_remove_file = move |index: usize| {
        let mut current_files = files.get();
        if index < current_files.len() {
            current_files.remove(index);
            set_files.set(current_files);
        }
    };

    let component_class = class.unwrap_or_else(|| "file-upload-input".to_string());

    view! {
        <div class=component_class>
            <FileInput
                name=name.clone()
                multiple=multiple
                accept=accept
                constraints=constraints.clone()
                on_change=Callback::new(handle_file_change)
            />

            <div class="file-list">
                {move || {
                    files.get().iter().enumerate().map(|(index, file)| {
                        let file_index = index;
                        let file_name = file.name.clone();
                        let file_size = file.size;
                        let file_type = file.mime_type.clone();

                        view! {
                            <div class="file-item">
                                <div class="file-info">
                                    <span class="file-name">{file_name}</span>
                                    <span class="file-size">{format_file_size(file_size)}</span>
                                    <span class="file-type">{file_type}</span>
                                </div>

                                <button
                                    type="button"
                                    class="remove-file-btn"
                                    on:click=move |_| handle_remove_file(file_index)
                                >
                                    "Remove"
                                </button>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>

            <div class="upload-progress">
                {move || {
                    upload_progress.get().iter().map(|progress| {
                        let file_name = progress.file_name.clone();
                        let percentage = progress.percentage;
                        let status = progress.status.clone();

                        view! {
                            <div class="progress-item">
                                <div class="progress-info">
                                    <span class="file-name">{file_name}</span>
                                    <span class="progress-percentage">{format!("{:.1}%", percentage)}</span>
                                </div>

                                <div class="progress-bar">
                                    <div
                                        class="progress-fill"
                                        style=format!("width: {}%", percentage)
                                    ></div>
                                </div>

                                <div class="progress-status">
                                    {match status {
                                        UploadStatus::Pending => "Pending",
                                        UploadStatus::Uploading => "Uploading",
                                        UploadStatus::Completed => "Completed",
                                        UploadStatus::Failed(ref error) => error,
                                        UploadStatus::Cancelled => "Cancelled",
                                    }}
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>

            <button
                type="button"
                class="upload-btn"
                disabled=move || is_uploading.get() || files.get().is_empty()
                on:click=handle_upload
            >
                {move || {
                    if is_uploading.get() {
                        "Uploading..."
                    } else {
                        "Upload Files"
                    }
                }}
            </button>
        </div>
    }
}

async fn upload_single_file(
    file: &FileData,
    upload_url: &str,
    index: usize,
    set_progress: WriteSignal<Vec<FileUploadProgress>>,
) -> Result<FileData, String> {
    // Implementation for single file upload
    // This would use the HTTP client from the form submission design
    todo!("Implement single file upload")
}

fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}
```

### **Phase 3: Form Integration (Week 3)**

#### **3.1 Form Field Integration**

```rust
// leptos-forms-rs/src/components/core/form_field.rs (enhanced)
// Add file field support to existing FormField component

// In the FormField component, add file field rendering:
let input_type = if let Some(FieldType::Email) = field_type_clone {
    "email"
} else if let Some(FieldType::Password) = field_type_clone {
    "password"
} else if let Some(FieldType::Number(_)) = field_type_clone {
    "number"
} else if let Some(FieldType::Boolean) = field_type_clone {
    "checkbox"
} else if let Some(FieldType::File(constraints)) = field_type_clone {
    // Render file input
    return view! {
        <FileInput
            name=name_clone
            constraints=constraints.clone()
            on_change=Callback::new(move |files| {
                // Update form field value
                form.set_field_value(&name_clone, FieldValue::File(files.first().cloned().unwrap_or_default()));
            })
        />
    };
} else {
    "text"
};
```

#### **3.2 File Field Hook**

```rust
// leptos-forms-rs/src/hooks/field/file_field.rs
use crate::core::traits::Form;
use crate::core::types::FileData;
use crate::core::FormHandle;
use leptos::prelude::*;

pub fn use_file_field<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
) -> Memo<Vec<FileData>> {
    let form_handle = form_handle.clone();
    let field_name = field_name.to_string();

    Memo::new(move |_| {
        form_handle
            .get_field_value(&field_name)
            .and_then(|v| v.as_file())
            .cloned()
            .unwrap_or_default()
    })
}

pub fn use_file_field_single<T: Form + PartialEq + Clone + Send + Sync>(
    form_handle: &FormHandle<T>,
    field_name: &str,
) -> Memo<Option<FileData>> {
    let form_handle = form_handle.clone();
    let field_name = field_name.to_string();

    Memo::new(move |_| {
        form_handle
            .get_field_value(&field_name)
            .and_then(|v| v.as_file())
            .and_then(|files| files.first().cloned())
    })
}
```

### **Phase 4: Testing and Documentation (Week 4)**

#### **4.1 Comprehensive Testing**

```rust
// tests/unit/src/file_upload_tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_validation() {
        let constraints = FileConstraints {
            max_size: Some(1024 * 1024), // 1MB
            allowed_types: vec!["image/*".to_string()],
            ..Default::default()
        };

        let valid_file = FileData {
            name: "test.jpg".to_string(),
            size: 512 * 1024, // 512KB
            mime_type: "image/jpeg".to_string(),
            last_modified: None,
            data: FileContent::Base64("test".to_string()),
        };

        assert!(FileValidators::validate_file(&valid_file, &constraints).is_ok());

        let invalid_file = FileData {
            name: "test.exe".to_string(),
            size: 2 * 1024 * 1024, // 2MB
            mime_type: "application/x-executable".to_string(),
            last_modified: None,
            data: FileContent::Base64("test".to_string()),
        };

        assert!(FileValidators::validate_file(&invalid_file, &constraints).is_err());
    }

    #[test]
    fn test_file_size_formatting() {
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1024 * 1024), "1.0 MB");
        assert_eq!(format_file_size(1024 * 1024 * 1024), "1.0 GB");
    }

    #[test]
    fn test_file_type_categorization() {
        assert_eq!(FileValidators::get_file_type_category("image/jpeg"), FileTypeCategory::Image);
        assert_eq!(FileValidators::get_file_type_category("video/mp4"), FileTypeCategory::Video);
        assert_eq!(FileValidators::get_file_type_category("audio/mp3"), FileTypeCategory::Audio);
        assert_eq!(FileValidators::get_file_type_category("text/plain"), FileTypeCategory::Text);
        assert_eq!(FileValidators::get_file_type_category("application/pdf"), FileTypeCategory::Document);
    }
}
```

## 🎯 **Usage Examples**

### **Basic File Upload**

```rust
#[component]
fn BasicFileUpload() -> impl IntoView {
    let (form, _submit_callback, _reset_callback) = use_form(FileUploadForm::default_values());

    view! {
        <div class="form">
            <FormField
                form=form.clone()
                name="avatar".to_string()
                label="Profile Picture".to_string()
                field_type=FieldType::File(FileConstraints {
                    max_size: Some(5 * 1024 * 1024), // 5MB
                    allowed_types: vec!["image/*".to_string()],
                    max_files: Some(1),
                    required: true,
                })
            />

            <FormSubmit form=form endpoint="/api/upload".to_string() />
        </div>
    }
}
```

### **Multiple File Upload**

```rust
#[component]
fn MultipleFileUpload() -> impl IntoView {
    let (form, _submit_callback, _reset_callback) = use_form(MultipleFileForm::default_values());

    view! {
        <div class="form">
            <FormField
                form=form.clone()
                name="documents".to_string()
                label="Documents".to_string()
                field_type=FieldType::File(FileConstraints {
                    max_size: Some(10 * 1024 * 1024), // 10MB
                    allowed_types: vec!["application/pdf".to_string(), "image/*".to_string()],
                    max_files: Some(5),
                    required: false,
                })
            />

            <FormSubmit form=form endpoint="/api/upload-multiple".to_string() />
        </div>
    }
}
```

## 🎯 **Success Criteria**

### **Functional Requirements**

- ✅ File input components
- ✅ File validation
- ✅ Upload progress tracking
- ✅ Multiple file support
- ✅ Drag and drop interface
- ✅ File preview
- ✅ Security validation

### **Performance Requirements**

- ✅ Handle files up to 100MB
- ✅ Progress updates in real-time
- ✅ Memory efficient
- ✅ No memory leaks

### **Quality Requirements**

- ✅ 100% test coverage
- ✅ No panics or crashes
- ✅ Proper error handling
- ✅ Security considerations

## 📊 **Implementation Timeline**

- **Week 1**: Core file types and validation
- **Week 2**: File input components
- **Week 3**: Form integration
- **Week 4**: Testing and documentation

## 🚀 **Next Steps**

1. **Approve Design**: Review and approve this design
2. **Set Up Development**: Create feature branch
3. **Implement Phase 1**: Core file types
4. **Implement Phase 2**: File input components
5. **Implement Phase 3**: Form integration
6. **Implement Phase 4**: Testing and documentation
7. **Security Review**: Security audit
8. **Documentation**: Update docs and examples

This design will add comprehensive file upload functionality to the library, making it suitable for modern web applications that need to handle file uploads.
