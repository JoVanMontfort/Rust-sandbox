use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FinancialDocument {
    pub document_type: DocumentType,
    pub confidence: f32,
    pub extracted_data: HashMap<String, String>,
    pub validation_errors: Vec<String>,
    pub suggested_categories: Vec<String>,
    pub tax_implications: Vec<String>,
    pub risk_assessment: RiskLevel,
    pub metadata: DocumentMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DocumentType {
    Invoice,
    Receipt,
    BankStatement,
    TaxForm(String),
    Contract,
    Bill,
    PaymentConfirmation,
    Payroll,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentMetadata {
    pub document_date: Option<String>,
    pub total_amount: Option<f64>,
    pub currency: Option<String>,
    pub parties: Vec<Party>,
    pub line_items: Vec<LineItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Party {
    pub role: String, // "payer", "payee", "employee", "employer"
    pub name: String,
    pub identifier: Option<String>, // SSN, EIN, Account number
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LineItem {
    pub description: String,
    pub quantity: Option<f64>,
    pub unit_price: Option<f64>,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub missing_fields: Vec<String>,
    pub data_quality_issues: Vec<String>,
    pub compliance_issues: Vec<String>,
    pub overall_score: f32,
}

impl FinancialDocument {
    pub fn pretty_print(&self) {
        println!("📋 Document Type: {:?}", self.document_type);
        println!("🎯 Confidence: {:.1}%", self.confidence * 100.0);
        println!("⚠️  Risk Level: {:?}", self.risk_assessment);

        println!("\n💰 Extracted Data:");
        for (key, value) in &self.extracted_data {
            println!("   • {}: {}", key, value);
        }

        if !self.validation_errors.is_empty() {
            println!("\n❌ Validation Errors:");
            for error in &self.validation_errors {
                println!("   • {}", error);
            }
        }

        if !self.suggested_categories.is_empty() {
            println!("\n🏷️ Suggested Categories:");
            for category in &self.suggested_categories {
                println!("   • {}", category);
            }
        }

        if !self.tax_implications.is_empty() {
            println!("\n💰 Tax Implications:");
            for tax in &self.tax_implications {
                println!("   • {}", tax);
            }
        }

        println!("\n📅 Metadata:");
        if let Some(date) = &self.metadata.document_date {
            println!("   • Date: {}", date);
        }
        if let Some(amount) = self.metadata.total_amount {
            println!("   • Total Amount: ${:.2}", amount);
        }
        if let Some(currency) = &self.metadata.currency {
            println!("   • Currency: {}", currency);
        }

        if !self.metadata.parties.is_empty() {
            println!("   • Parties:");
            for party in &self.metadata.parties {
                println!("     - {}: {}", party.role, party.name);
            }
        }

        if !self.metadata.line_items.is_empty() {
            println!("   • Line Items:");
            for item in &self.metadata.line_items {
                println!("     - {}: ${:.2}", item.description, item.amount);
            }
        }
    }
}

impl ValidationResult {
    pub fn pretty_print(&self) {
        println!("\n🔍 Validation Results:");
        println!("   Overall Score: {:.1}%", self.overall_score * 100.0);
        println!("   Valid: {}", if self.is_valid { "✅" } else { "❌" });

        if !self.missing_fields.is_empty() {
            println!("   Missing Fields:");
            for field in &self.missing_fields {
                println!("     • {}", field);
            }
        }

        if !self.data_quality_issues.is_empty() {
            println!("   Data Quality Issues:");
            for issue in &self.data_quality_issues {
                println!("     • {}", issue);
            }
        }

        if !self.compliance_issues.is_empty() {
            println!("   Compliance Issues:");
            for issue in &self.compliance_issues {
                println!("     • {}", issue);
            }
        }
    }
}