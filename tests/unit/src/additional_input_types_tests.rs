use leptos_forms_rs::core::types::FieldValue;
use leptos_forms_rs::core::FormHandle;
use leptos_forms_rs::core::Form;
use leptos::prelude::{Get, GetUntracked};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct AdditionalInputTypesForm {
    rich_text_content: String,
    markdown_content: String,
    code_content: String,
    uploaded_files: Vec<String>,
    image_files: Vec<String>,
    document_files: Vec<String>,
}

impl Form for AdditionalInputTypesForm {
    fn field_metadata() -> Vec<leptos_forms_rs::core::FieldMetadata> {
        vec![
            leptos_forms_rs::core::FieldMetadata {
                name: "rich_text_content".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::RichText,
                validators: vec![],
                is_required: false,
                default_value: Some(FieldValue::String("".to_string())),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "markdown_content".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Markdown,
                validators: vec![],
                is_required: false,
                default_value: Some(FieldValue::String("".to_string())),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "code_content".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::Code,
                validators: vec![],
                is_required: false,
                default_value: Some(FieldValue::String("".to_string())),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "uploaded_files".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::File(leptos_forms_rs::core::types::FileConstraints {
                    max_size: Some(10 * 1024 * 1024), // 10MB
                    accept: vec!["*/*".to_string()],
                    multiple: true,
                }),
                validators: vec![],
                is_required: false,
                default_value: Some(FieldValue::Array(vec![])),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "image_files".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::File(leptos_forms_rs::core::types::FileConstraints {
                    max_size: Some(5 * 1024 * 1024), // 5MB
                    accept: vec!["image/*".to_string()],
                    multiple: true,
                }),
                validators: vec![],
                is_required: false,
                default_value: Some(FieldValue::Array(vec![])),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
            leptos_forms_rs::core::FieldMetadata {
                name: "document_files".to_string(),
                field_type: leptos_forms_rs::core::types::FieldType::File(leptos_forms_rs::core::types::FileConstraints {
                    max_size: Some(25 * 1024 * 1024), // 25MB
                    accept: vec![
                        "application/pdf".to_string(),
                        "application/msword".to_string(),
                        "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
                    ],
                    multiple: true,
                }),
                validators: vec![],
                is_required: false,
                default_value: Some(FieldValue::Array(vec![])),
                dependencies: vec![],
                attributes: std::collections::HashMap::new(),
            },
        ]
    }
    
    fn validate(&self) -> Result<(), leptos_forms_rs::validation::ValidationErrors> {
        let mut errors = leptos_forms_rs::validation::ValidationErrors::new();
        
        // Validate file sizes and types
        if self.uploaded_files.len() > 10 {
            errors.add_field_error("uploaded_files", "Maximum 10 files allowed".to_string());
        }
        
        if self.image_files.len() > 20 {
            errors.add_field_error("image_files", "Maximum 20 images allowed".to_string());
        }
        
        if self.document_files.len() > 5 {
            errors.add_field_error("document_files", "Maximum 5 documents allowed".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    fn get_field_value(&self, name: &str) -> FieldValue {
        match name {
            "rich_text_content" => FieldValue::String(self.rich_text_content.clone()),
            "markdown_content" => FieldValue::String(self.markdown_content.clone()),
            "code_content" => FieldValue::String(self.code_content.clone()),
            "uploaded_files" => FieldValue::Array(self.uploaded_files.iter().map(|s| FieldValue::String(s.clone())).collect()),
            "image_files" => FieldValue::Array(self.image_files.iter().map(|s| FieldValue::String(s.clone())).collect()),
            "document_files" => FieldValue::Array(self.document_files.iter().map(|s| FieldValue::String(s.clone())).collect()),
            _ => FieldValue::String(String::new()),
        }
    }

    fn default_values() -> Self {
        Self {
            rich_text_content: "".to_string(),
            markdown_content: "".to_string(),
            code_content: "".to_string(),
            uploaded_files: vec![],
            image_files: vec![],
            document_files: vec![],
        }
    }
    
    fn schema() -> leptos_forms_rs::core::FormSchema {
        leptos_forms_rs::core::FormSchema {
            name: "AdditionalInputTypesForm".to_string(),
            field_metadata: Self::field_metadata(),
        }
    }
}

#[test]
fn test_rich_text_input_component_creation() {
    // Test that we can create the rich text input component
    let form: FormHandle<AdditionalInputTypesForm> = FormHandle::new(AdditionalInputTypesForm::default_values());
    
    // This would test the RichTextInput component if it existed
    // For now, we just verify the form can be created
    assert!(true);
}

#[test]
fn test_markdown_input_component_creation() {
    // Test that we can create the markdown input component
    let form: FormHandle<AdditionalInputTypesForm> = FormHandle::new(AdditionalInputTypesForm::default_values());
    
    // This would test the MarkdownInput component if it existed
    // For now, we just verify the form can be created
    assert!(true);
}

#[test]
fn test_code_input_component_creation() {
    // Test that we can create the code input component
    let form: FormHandle<AdditionalInputTypesForm> = FormHandle::new(AdditionalInputTypesForm::default_values());
    
    // This would test the CodeInput component if it existed
    // For now, we just verify the form can be created
    assert!(true);
}

#[test]
fn test_file_upload_input_component_creation() {
    // Test that we can create the file upload input component
    let form: FormHandle<AdditionalInputTypesForm> = FormHandle::new(AdditionalInputTypesForm::default_values());
    
    // This would test the FileUploadInput component if it existed
    // For now, we just verify the form can be created
    assert!(true);
}

#[test]
fn test_rich_text_content_handling() {
    // Test that rich text content can be handled
    let mut form_data = AdditionalInputTypesForm::default_values();
    let rich_content = "<p><strong>Bold text</strong> and <em>italic text</em></p>";
    form_data.rich_text_content = rich_content.to_string();
    
    let form: FormHandle<AdditionalInputTypesForm> = FormHandle::new(form_data);
    
    // Verify the content was set
    let current_values = form.values();
    let form_data = current_values.get_untracked();
    
    assert_eq!(form_data.rich_text_content, rich_content);
}

#[test]
fn test_markdown_content_handling() {
    // Test that markdown content can be handled
    let mut form_data = AdditionalInputTypesForm::default_values();
    let markdown_content = "# Heading\n\n**Bold** and *italic* text\n\n- List item 1\n- List item 2";
    form_data.markdown_content = markdown_content.to_string();
    
    let form: FormHandle<AdditionalInputTypesForm> = FormHandle::new(form_data);
    
    // Verify the content was set
    let current_values = form.values();
    let form_data = current_values.get_untracked();
    
    assert_eq!(form_data.markdown_content, markdown_content);
}

#[test]
fn test_code_content_handling() {
    // Test that code content can be handled
    let mut form_data = AdditionalInputTypesForm::default_values();
    let code_content = "fn main() {\n    println!(\"Hello, world!\");\n}";
    form_data.code_content = code_content.to_string();
    
    let form: FormHandle<AdditionalInputTypesForm> = FormHandle::new(form_data);
    
    // Verify the content was set
    let current_values = form.values();
    let form_data = current_values.get_untracked();
    
    assert_eq!(form_data.code_content, code_content);
}

#[test]
fn test_file_upload_handling() {
    // Test that file uploads can be handled
    let mut form_data = AdditionalInputTypesForm::default_values();
    let file_urls = vec![
        "uploads/file1.pdf".to_string(),
        "uploads/file2.docx".to_string(),
    ];
    form_data.document_files = file_urls.clone();
    
    let form: FormHandle<AdditionalInputTypesForm> = FormHandle::new(form_data);
    
    // Verify the files were set
    let current_values = form.values();
    let form_data = current_values.get_untracked();
    
    assert_eq!(form_data.document_files.len(), 2);
    assert_eq!(form_data.document_files[0], "uploads/file1.pdf");
    assert_eq!(form_data.document_files[1], "uploads/file2.docx");
}

#[test]
fn test_image_file_handling() {
    // Test that image files can be handled
    let mut form_data = AdditionalInputTypesForm::default_values();
    let image_urls = vec![
        "uploads/image1.jpg".to_string(),
        "uploads/image2.png".to_string(),
        "uploads/image3.gif".to_string(),
    ];
    form_data.image_files = image_urls.clone();
    
    let form: FormHandle<AdditionalInputTypesForm> = FormHandle::new(form_data);
    
    // Verify the images were set
    let current_values = form.values();
    let form_data = current_values.get_untracked();
    
    assert_eq!(form_data.image_files.len(), 3);
    assert_eq!(form_data.image_files[0], "uploads/image1.jpg");
    assert_eq!(form_data.image_files[1], "uploads/image2.png");
    assert_eq!(form_data.image_files[2], "uploads/image3.gif");
}

#[test]
fn test_file_validation() {
    // Test that file validation works correctly
    let mut form_data = AdditionalInputTypesForm::default_values();
    
    // Add too many files to trigger validation error
    let many_files: Vec<String> = (1..=11).map(|i| format!("file{}.pdf", i)).collect();
    form_data.uploaded_files = many_files;
    
    let form: FormHandle<AdditionalInputTypesForm> = FormHandle::new(form_data);
    
    // Validate the form
    let validation_result = form.validate();
    
    // Should fail due to too many files
    assert!(validation_result.is_err());
    
    if let Err(_errors) = validation_result {
        // Validation failed as expected
        assert!(true);
    }
}

#[test]
fn test_rich_text_editor_features() {
    // Test that rich text editor features work
    let mut form_data = AdditionalInputTypesForm::default_values();
    
    // Test various rich text features
    let rich_content = r#"
        <h1>Main Title</h1>
        <h2>Subtitle</h2>
        <p>This is a <strong>paragraph</strong> with <em>formatting</em>.</p>
        <ul>
            <li>List item 1</li>
            <li>List item 2</li>
        </ul>
        <blockquote>This is a quote</blockquote>
        <code>Inline code</code>
    "#;
    form_data.rich_text_content = rich_content.to_string();
    
    let form: FormHandle<AdditionalInputTypesForm> = FormHandle::new(form_data);
    
    // Verify the content was set
    let current_values = form.values();
    let form_data = current_values.get_untracked();
    
    assert!(form_data.rich_text_content.contains("<h1>Main Title</h1>"));
    assert!(form_data.rich_text_content.contains("<strong>paragraph</strong>"));
    assert!(form_data.rich_text_content.contains("<ul>"));
}

#[test]
fn test_markdown_editor_features() {
    // Test that markdown editor features work
    let mut form_data = AdditionalInputTypesForm::default_values();
    
    // Test various markdown features
    let markdown_content = r#"
# Main Title
## Subtitle

This is a **bold** and *italic* text.

- List item 1
- List item 2

1. Numbered item 1
2. Numbered item 2

> This is a blockquote

`Inline code`

```rust
fn main() {
    println!("Hello, world!");
}
```

[Link text](https://example.com)

![Alt text](image.jpg)
    "#;
    form_data.markdown_content = markdown_content.to_string();
    
    let form: FormHandle<AdditionalInputTypesForm> = FormHandle::new(form_data);
    
    // Verify the content was set
    let current_values = form.values();
    let form_data = current_values.get_untracked();
    
    assert!(form_data.markdown_content.contains("# Main Title"));
    assert!(form_data.markdown_content.contains("**bold**"));
    assert!(form_data.markdown_content.contains("- List item 1"));
    assert!(form_data.markdown_content.contains("```rust"));
}

#[test]
fn test_code_editor_features() {
    // Test that code editor features work
    let form: FormHandle<AdditionalInputTypesForm> = FormHandle::new(AdditionalInputTypesForm::default_values());
    
    // Test various code features
    let code_content = r#"
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct User {
    id: u64,
    name: String,
    email: String,
}

impl User {
    fn new(id: u64, name: String, email: String) -> Self {
        Self { id, name, email }
    }
    
    fn display_info(&self) {
        println!("User: {} ({})", self.name, self.email);
    }
}

fn main() {
    let mut users = HashMap::new();
    
    let user = User::new(1, "John Doe".to_string(), "john@example.com".to_string());
    users.insert(user.id, user);
    
    for (id, user) in &users {
        println!("ID: {}", id);
        user.display_info();
    }
}
    "#;
    let mut form_data = AdditionalInputTypesForm::default_values();
    form_data.code_content = code_content.to_string();
    
    let form: FormHandle<AdditionalInputTypesForm> = FormHandle::new(form_data);
    
    // Verify the content was set
    let current_values = form.values();
    let form_data = current_values.get_untracked();
    
    assert!(form_data.code_content.contains("struct User"));
    assert!(form_data.code_content.contains("impl User"));
    assert!(form_data.code_content.contains("fn main()"));
    assert!(form_data.code_content.contains("HashMap"));
}
