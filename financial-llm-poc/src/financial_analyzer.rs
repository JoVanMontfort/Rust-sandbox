use crate::document_types::{FinancialDocument, ValidationResult};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct LLMRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
    response_format: Option<ResponseFormat>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    r#type: String,
}

#[derive(Debug, Deserialize)]
struct LLMResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

pub struct FinancialAnalyzer {
    client: Client,
    api_key: String,
}

impl FinancialAnalyzer {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn analyze_document(&self, text: &str) -> Result<FinancialDocument> {
        let prompt = self.build_analysis_prompt(text);

        let request = LLMRequest {
            model: "gpt-3.5-turbo".to_string(), // or "gpt-4" for better accuracy
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: r#"You are a financial document analysis expert.
                    Analyze financial documents and extract structured data.
                    Always respond with valid JSON in the specified format.
                    Be accurate and thorough in your analysis."#
                        .to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: 0.1,
            max_tokens: 2000,
            response_format: Some(ResponseFormat {
                r#type: "json_object".to_string(),
            }),
        };

        let response = self.call_llm(request).await?;
        let analysis: FinancialDocument = serde_json::from_str(&response)
            .map_err(|e| anyhow::anyhow!("Failed to parse LLM response: {}", e))?;

        Ok(analysis)
    }

    pub async fn validate_document(
        &self,
        document: &FinancialDocument,
    ) -> Result<ValidationResult> {
        let prompt = self.build_validation_prompt(document);

        let request = LLMRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: r#"You are a financial document validation expert.
                    Validate financial documents for completeness, accuracy, and compliance.
                    Return JSON with validation results."#
                        .to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: 0.1,
            max_tokens: 1000,
            response_format: Some(ResponseFormat {
                r#type: "json_object".to_string(),
            }),
        };

        let response = self.call_llm(request).await?;
        let validation: ValidationResult = serde_json::from_str(&response)
            .map_err(|e| anyhow::anyhow!("Failed to parse validation response: {}", e))?;

        Ok(validation)
    }

    pub async fn convert_to_json(&self, text: &str) -> Result<serde_json::Value> {
        let prompt = format!(
            r#"Convert this financial document into structured JSON format.
            Extract all relevant fields and maintain data relationships.

            DOCUMENT:
            {}

            Return a clean JSON object with all extracted data."#,
            text
        );

        let request = LLMRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "Convert financial documents to structured JSON format.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: 0.1,
            max_tokens: 2000,
            response_format: Some(ResponseFormat {
                r#type: "json_object".to_string(),
            }),
        };

        let response = self.call_llm(request).await?;
        let json_data: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON conversion: {}", e))?;

        Ok(json_data)
    }

    fn build_analysis_prompt(&self, text: &str) -> String {
        format!(
            r#"
            Analyze this financial document and extract structured information.

            DOCUMENT TEXT:
            {}

            Return JSON in this exact format:
            {{
                "document_type": "Invoice|Receipt|BankStatement|TaxForm|Contract|Bill|PaymentConfirmation|Payroll|Unknown",
                "confidence": 0.95,
                "extracted_data": {{
                    "date": "YYYY-MM-DD",
                    "total_amount": "123.45",
                    "vendor": "Company Name",
                    "tax_amount": "12.34",
                    "currency": "USD",
                    "document_number": "INV-001",
                    "payment_terms": "Net 30"
                }},
                "validation_errors": ["Missing invoice number", "Invalid date format"],
                "suggested_categories": ["Office Supplies", "Tax Deductible"],
                "tax_implications": ["VAT applicable", "Business expense"],
                "risk_assessment": "Low|Medium|High|Critical",
                "metadata": {{
                    "document_date": "2024-01-15",
                    "total_amount": 4860.0,
                    "currency": "USD",
                    "parties": [
                        {{"role": "payer", "name": "ABC Corporation"}},
                        {{"role": "payee", "name": "Tech Solutions Inc."}}
                    ],
                    "line_items": [
                        {{"description": "Software License", "quantity": 2, "unit_price": 1500.0, "amount": 3000.0}},
                        {{"description": "Technical Support", "quantity": 10, "unit_price": 100.0, "amount": 1000.0}}
                    ]
                }}
            }}

            Be thorough and accurate in your analysis.
            "#,
            text
        )
    }

    fn build_validation_prompt(&self, document: &FinancialDocument) -> String {
        let doc_json = serde_json::to_string_pretty(document).unwrap();

        format!(
            r#"
            Validate this financial document analysis for completeness and compliance.

            DOCUMENT ANALYSIS:
            {}

            Check for:
            1. Missing required fields based on document type
            2. Data consistency issues
            3. Compliance with financial regulations
            4. Risk factors

            Return JSON:
            {{
                "is_valid": true|false,
                "missing_fields": ["field1", "field2"],
                "data_quality_issues": ["issue1", "issue2"],
                "compliance_issues": ["compliance1", "compliance2"],
                "overall_score": 0.95
            }}
            "#,
            doc_json
        )
    }

    async fn call_llm(&self, request: LLMRequest) -> Result<String> {
        let response: LLMResponse = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.choices[0].message.content.clone())
    }
}