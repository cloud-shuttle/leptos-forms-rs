//! CSS styles for the form wizard example
//!
//! This module provides the CSS styles used in the form wizard example.

/// CSS styles for the form wizard example
pub const FORM_WIZARD_EXAMPLE_CSS: &str = r#"
.form-wizard-example {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
}

.form-wizard-example h1 {
    text-align: center;
    color: #1e293b;
    margin-bottom: 1rem;
}

.form-wizard-example > p {
    text-align: center;
    color: #64748b;
    margin-bottom: 3rem;
    font-size: 1.1rem;
}

.example-info {
    margin-top: 3rem;
    padding: 2rem;
    background: #f8fafc;
    border-radius: 8px;
    border: 1px solid #e2e8f0;
}

.example-info h3 {
    color: #1e293b;
    margin-bottom: 1rem;
}

.example-info ul {
    margin-bottom: 2rem;
    padding-left: 1.5rem;
}

.example-info li {
    color: #475569;
    margin-bottom: 0.5rem;
    line-height: 1.5;
}

.current-step-info {
    background: #eff6ff;
    border: 1px solid #bfdbfe;
    border-radius: 6px;
    padding: 1rem;
}

.current-step-info h4 {
    color: #1e40af;
    margin: 0 0 0.5rem 0;
}

.current-step-info p {
    color: #1e40af;
    margin: 0;
    font-weight: 500;
}

/* Step-specific styles */
.step-personal-info,
.step-preferences,
.step-confirmation {
    padding: 1rem;
}

.step-personal-info h3,
.step-preferences h3,
.step-confirmation h3 {
    color: #1e293b;
    margin-bottom: 0.5rem;
}

.step-personal-info p,
.step-preferences p,
.step-confirmation p {
    color: #64748b;
    margin-bottom: 1.5rem;
}

.form-group {
    margin-bottom: 1.5rem;
}

.form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #374151;
}

.form-input,
.form-select {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-size: 0.875rem;
    transition: border-color 0.2s ease;
}

.form-input:focus,
.form-select:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    font-weight: 500;
    color: #374151;
}

.checkbox-label input[type="checkbox"] {
    width: 1.25rem;
    height: 1.25rem;
    accent-color: #3b82f6;
}

.summary {
    background: #f9fafb;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    padding: 1rem;
    margin-bottom: 1.5rem;
}

.summary h4 {
    color: #1f2937;
    margin: 0 0 1rem 0;
    font-size: 1rem;
}

.summary-item {
    display: flex;
    justify-content: space-between;
    padding: 0.5rem 0;
    border-bottom: 1px solid #e5e7eb;
}

.summary-item:last-child {
    border-bottom: none;
}

.summary-item strong {
    color: #374151;
    font-weight: 600;
}

.summary-item span {
    color: #6b7280;
}

/* Responsive adjustments */
@media (max-width: 768px) {
    .form-wizard-example {
        padding: 1rem;
    }

    .example-info {
        margin-top: 2rem;
        padding: 1rem;
    }

    .summary-item {
        flex-direction: column;
        gap: 0.25rem;
    }
}
"#;
