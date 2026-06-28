//! Contract generation module with guided prompts and legal templates

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ContractGenerator {
    templates: HashMap<String, ContractTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: ContractCategory,
    pub prompts: Vec<ContractPrompt>,
    pub template_text: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContractCategory {
    Freelance,
    NDA,
    Employment,
    Rental,
    Sales,
    Partnership,
    Consulting,
    Coaching,
    Custom,
}

impl ContractCategory {
    pub fn name(&self) -> &'static str {
        match self {
            ContractCategory::Freelance => "Freelance",
            ContractCategory::NDA => "NDA",
            ContractCategory::Employment => "Employment",
            ContractCategory::Rental => "Rental",
            ContractCategory::Sales => "Sales",
            ContractCategory::Partnership => "Partnership",
            ContractCategory::Consulting => "Consulting",
            ContractCategory::Coaching => "Coaching",
            ContractCategory::Custom => "Custom",
        }
    }
    
    pub fn all() -> Vec<ContractCategory> {
        vec![
            ContractCategory::Freelance,
            ContractCategory::NDA,
            ContractCategory::Employment,
            ContractCategory::Rental,
            ContractCategory::Sales,
            ContractCategory::Partnership,
            ContractCategory::Consulting,
            ContractCategory::Coaching,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractPrompt {
    pub field: String,
    pub question: String,
    pub placeholder: String,
    pub required: bool,
    pub help_text: String,
    pub field_type: FieldType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    Text,
    Number,
    Date,
    Select(Vec<String>),
    TextArea,
    Currency,
    Email,
}

#[derive(Debug, Clone)]
pub struct GeneratedContract {
    pub title: String,
    pub content: String,
    pub plain_english_summary: String,
    pub disclaimer: String,
}

impl ContractGenerator {
    pub fn new() -> Self {
        let mut generator = Self {
            templates: HashMap::new(),
        };
        generator.load_templates();
        generator
    }
    
    fn load_templates(&mut self) {
        let templates = vec![
            ContractTemplate {
                id: "freelance_service".to_string(),
                name: "Freelance Service Agreement".to_string(),
                description: "Standard agreement for freelance services".to_string(),
                category: ContractCategory::Freelance,
                prompts: vec![
                    ContractPrompt {
                        field: "client_name".to_string(),
                        question: "What is the client's full name or company name?".to_string(),
                        placeholder: "e.g., Acme Corporation".to_string(),
                        required: true,
                        help_text: "Full legal name as it will appear on the contract".to_string(),
                        field_type: FieldType::Text,
                    },
                    ContractPrompt {
                        field: "freelancer_name".to_string(),
                        question: "What is your full name or business name?".to_string(),
                        placeholder: "e.g., Jane Smith".to_string(),
                        required: true,
                        help_text: "Your full legal name".to_string(),
                        field_type: FieldType::Text,
                    },
                    ContractPrompt {
                        field: "service_description".to_string(),
                        question: "Describe the services you will provide:".to_string(),
                        placeholder: "e.g., Website design and development...".to_string(),
                        required: true,
                        help_text: "Be specific about deliverables".to_string(),
                        field_type: FieldType::TextArea,
                    },
                    ContractPrompt {
                        field: "payment_amount".to_string(),
                        question: "What is the total payment amount?".to_string(),
                        placeholder: "e.g., 5000".to_string(),
                        required: true,
                        help_text: "Total contract value in dollars".to_string(),
                        field_type: FieldType::Currency,
                    },
                    ContractPrompt {
                        field: "payment_schedule".to_string(),
                        question: "What is the payment schedule?".to_string(),
                        placeholder: "Select schedule".to_string(),
                        required: true,
                        help_text: "When payments are due".to_string(),
                        field_type: FieldType::Select(vec![
                            "50% upfront, 50% on completion".to_string(),
                            "100% upfront".to_string(),
                            "100% on completion".to_string(),
                            "Monthly installments".to_string(),
                        ]),
                    },
                    ContractPrompt {
                        field: "timeline".to_string(),
                        question: "What is the project timeline?".to_string(),
                        placeholder: "e.g., 30 days from contract signing".to_string(),
                        required: true,
                        help_text: "Start date, end date, or duration".to_string(),
                        field_type: FieldType::Text,
                    },
                    ContractPrompt {
                        field: "revisions".to_string(),
                        question: "How many revisions are included?".to_string(),
                        placeholder: "Select number".to_string(),
                        required: true,
                        help_text: "Number of revision rounds before additional fees".to_string(),
                        field_type: FieldType::Select(vec![
                            "1".to_string(),
                            "2".to_string(),
                            "3".to_string(),
                            "Unlimited".to_string(),
                        ]),
                    },
                    ContractPrompt {
                        field: "jurisdiction".to_string(),
                        question: "What state/country governs this contract?".to_string(),
                        placeholder: "e.g., California, USA".to_string(),
                        required: true,
                        help_text: "This determines which laws apply".to_string(),
                        field_type: FieldType::Text,
                    },
                ],
                template_text: r#"FREELANCE SERVICE AGREEMENT

This Agreement is made between:

CLIENT: {client_name}
FREELANCER: {freelancer_name}

1. SERVICES
{service_description}

2. COMPENSATION
Total Amount: ${payment_amount}
Payment Schedule: {payment_schedule}

3. TIMELINE
{timeline}

4. REVISIONS
{revisions} revision(s) are included in the base price. Additional revisions will be billed at an hourly rate.

5. INTELLECTUAL PROPERTY
Upon full payment, Client will own all rights to the deliverables. Freelancer retains the right to use the work in their portfolio.

6. TERMINATION
Either party may terminate this agreement with 7 days written notice. Client pays for work completed to date.

7. GOVERNING LAW
This Agreement shall be governed by the laws of {jurisdiction}.

8. SIGNATURES

_________________________
Client: {client_name}
Date: _______________

_________________________
Freelancer: {freelancer_name}
Date: _______________

{disclaimer}
"#.to_string(),
            },
            ContractTemplate {
                id: "nda_mutual".to_string(),
                name: "Mutual Non-Disclosure Agreement".to_string(),
                description: "Protect confidential information shared between parties".to_string(),
                category: ContractCategory::NDA,
                prompts: vec![
                    ContractPrompt {
                        field: "party_a".to_string(),
                        question: "First party name:".to_string(),
                        placeholder: "e.g., Company A".to_string(),
                        required: true,
                        help_text: "Full legal name".to_string(),
                        field_type: FieldType::Text,
                    },
                    ContractPrompt {
                        field: "party_b".to_string(),
                        question: "Second party name:".to_string(),
                        placeholder: "e.g., Company B".to_string(),
                        required: true,
                        help_text: "Full legal name".to_string(),
                        field_type: FieldType::Text,
                    },
                    ContractPrompt {
                        field: "purpose".to_string(),
                        question: "What is the purpose of sharing confidential information?".to_string(),
                        placeholder: "e.g., Exploring a potential business partnership...".to_string(),
                        required: true,
                        help_text: "Why information is being shared".to_string(),
                        field_type: FieldType::TextArea,
                    },
                    ContractPrompt {
                        field: "duration_years".to_string(),
                        question: "How many years should the NDA last?".to_string(),
                        placeholder: "e.g., 3".to_string(),
                        required: true,
                        help_text: "Standard is 2-5 years".to_string(),
                        field_type: FieldType::Number,
                    },
                    ContractPrompt {
                        field: "jurisdiction".to_string(),
                        question: "Governing law jurisdiction:".to_string(),
                        placeholder: "e.g., California, USA".to_string(),
                        required: true,
                        help_text: "Which laws apply".to_string(),
                        field_type: FieldType::Text,
                    },
                ],
                template_text: r#"MUTUAL NON-DISCLOSURE AGREEMENT

This Agreement is made between:

PARTY A: {party_a}
PARTY B: {party_b}

1. PURPOSE
The parties wish to explore {purpose} and need to share confidential information.

2. DEFINITION OF CONFIDENTIAL INFORMATION
Confidential Information means any and all non-public, proprietary, or confidential information disclosed by either party.

3. OBLIGATIONS OF RECEIVING PARTY
Each party agrees to:
- Keep all Confidential Information strictly confidential
- Not disclose to any third parties without written consent
- Use the information solely for the stated purpose
- Protect the information with the same care as their own confidential information

4. TERM
This Agreement shall remain in effect for {duration_years} years from the date of signing.

5. RETURN OF INFORMATION
Upon request or termination, each party shall return or destroy all Confidential Information.

6. GOVERNING LAW
This Agreement shall be governed by the laws of {jurisdiction}.

7. SIGNATURES

_________________________
Party A: {party_a}
Date: _______________

_________________________
Party B: {party_b}
Date: _______________

{disclaimer}
"#.to_string(),
            },
            ContractTemplate {
                id: "rental_agreement".to_string(),
                name: "Rental/Lease Agreement".to_string(),
                description: "Residential or commercial property lease".to_string(),
                category: ContractCategory::Rental,
                prompts: vec![
                    ContractPrompt {
                        field: "landlord_name".to_string(),
                        question: "Landlord name:".to_string(),
                        placeholder: "e.g., John Smith".to_string(),
                        required: true,
                        help_text: "Full legal name".to_string(),
                        field_type: FieldType::Text,
                    },
                    ContractPrompt {
                        field: "tenant_name".to_string(),
                        question: "Tenant name:".to_string(),
                        placeholder: "e.g., Jane Doe".to_string(),
                        required: true,
                        help_text: "Full legal name".to_string(),
                        field_type: FieldType::Text,
                    },
                    ContractPrompt {
                        field: "property_address".to_string(),
                        question: "Property address:".to_string(),
                        placeholder: "Full street address".to_string(),
                        required: true,
                        help_text: "Complete address".to_string(),
                        field_type: FieldType::TextArea,
                    },
                    ContractPrompt {
                        field: "monthly_rent".to_string(),
                        question: "Monthly rent amount:".to_string(),
                        placeholder: "e.g., 1500".to_string(),
                        required: true,
                        help_text: "Base monthly rent".to_string(),
                        field_type: FieldType::Currency,
                    },
                    ContractPrompt {
                        field: "security_deposit".to_string(),
                        question: "Security deposit:".to_string(),
                        placeholder: "e.g., 1500".to_string(),
                        required: true,
                        help_text: "Typically 1-2 months rent".to_string(),
                        field_type: FieldType::Currency,
                    },
                    ContractPrompt {
                        field: "lease_term".to_string(),
                        question: "Lease term:".to_string(),
                        placeholder: "Select term".to_string(),
                        required: true,
                        help_text: "Duration of lease".to_string(),
                        field_type: FieldType::Select(vec![
                            "Month-to-month".to_string(),
                            "6 months".to_string(),
                            "1 year".to_string(),
                            "2 years".to_string(),
                        ]),
                    },
                ],
                template_text: r#"RESIDENTIAL LEASE AGREEMENT

LANDLORD: {landlord_name}
TENANT: {tenant_name}

PROPERTY: {property_address}

1. TERM
This lease is for {lease_term} beginning on the date of signing.

2. RENT
Monthly Rent: ${monthly_rent}
Security Deposit: ${security_deposit}
Rent is due on the 1st of each month.

3. USE OF PROPERTY
The property shall be used solely as a residential dwelling.

4. UTILITIES
Tenant is responsible for all utilities unless otherwise agreed in writing.

5. MAINTENANCE
Landlord is responsible for major repairs. Tenant is responsible for minor repairs under $100.

6. PETS
No pets without prior written consent from Landlord.

7. TERMINATION
Either party may terminate with 30 days written notice (or as required by local law).

8. GOVERNING LAW
This Agreement shall be governed by local landlord-tenant laws.

9. SIGNATURES

_________________________
Landlord: {landlord_name}
Date: _______________

_________________________
Tenant: {tenant_name}
Date: _______________

{disclaimer}
"#.to_string(),
            },
        ];
        
        for template in templates {
            self.templates.insert(template.id.clone(), template);
        }
    }
    
    pub fn get_template(&self, id: &str) -> Option<&ContractTemplate> {
        self.templates.get(id)
    }
    
    pub fn list_templates(&self) -> Vec<&ContractTemplate> {
        self.templates.values().collect()
    }
    
    pub fn by_category(&self, category: ContractCategory) -> Vec<&ContractTemplate> {
        self.templates.values()
            .filter(|t| t.category == category)
            .collect()
    }
    
    pub fn generate(&self, template_id: &str, answers: HashMap<String, String>) -> Result<GeneratedContract, String> {
        let template = self.templates.get(template_id)
            .ok_or("Template not found")?;
        
        // Validate required fields
        for prompt in &template.prompts {
            if prompt.required && !answers.contains_key(&prompt.field) {
                return Err(format!("Required field '{}' not provided", prompt.field));
            }
        }
        
        // Build contract content
        let mut content = template.template_text.clone();
        for (field, value) in &answers {
            let placeholder = format!("{{{}}}", field);
            content = content.replace(&placeholder, value);
        }
        
        // Add disclaimer
        let disclaimer = self.get_legal_disclaimer();
        content = content.replace("{disclaimer}", &disclaimer);
        
        // Generate plain English summary
        let summary = self.generate_summary(template, &answers);
        
        Ok(GeneratedContract {
            title: template.name.clone(),
            content,
            plain_english_summary: summary,
            disclaimer,
        })
    }
    
    fn generate_summary(&self, template: &ContractTemplate, answers: &HashMap<String, String>) -> String {
        match template.category {
            ContractCategory::Freelance => {
                format!(
                    "This is a service agreement between {} and {} for {}. \
                    The total payment is ${} with payment schedule: {}. \
                    The project should be completed within {} with {} revisions included.",
                    answers.get("client_name").unwrap_or(&"the client".to_string()),
                    answers.get("freelancer_name").unwrap_or(&"the freelancer".to_string()),
                    answers.get("service_description").unwrap_or(&"services".to_string()),
                    answers.get("payment_amount").unwrap_or(&"0".to_string()),
                    answers.get("payment_schedule").unwrap_or(&"as agreed".to_string()),
                    answers.get("timeline").unwrap_or(&"the agreed timeframe".to_string()),
                    answers.get("revisions").unwrap_or(&"specified number of".to_string()),
                )
            },
            ContractCategory::NDA => {
                format!(
                    "This is a mutual non-disclosure agreement between {} and {}. \
                    It protects confidential information shared for the purpose of: {}. \
                    The agreement lasts for {} years.",
                    answers.get("party_a").unwrap_or(&"Party A".to_string()),
                    answers.get("party_b").unwrap_or(&"Party B".to_string()),
                    answers.get("purpose").unwrap_or(&"the stated purpose".to_string()),
                    answers.get("duration_years").unwrap_or(&"specified".to_string()),
                )
            },
            ContractCategory::Rental => {
                format!(
                    "This is a {} lease between landlord {} and tenant {} \
                    for the property at {}. Monthly rent is ${} with a ${} security deposit.",
                    answers.get("lease_term").unwrap_or(&"specified term".to_string()),
                    answers.get("landlord_name").unwrap_or(&"the landlord".to_string()),
                    answers.get("tenant_name").unwrap_or(&"the tenant".to_string()),
                    answers.get("property_address").unwrap_or(&"the property".to_string()),
                    answers.get("monthly_rent").unwrap_or(&"0".to_string()),
                    answers.get("security_deposit").unwrap_or(&"0".to_string()),
                )
            },
            _ => "Please review the contract carefully before signing.".to_string(),
        }
    }
    
    fn get_legal_disclaimer(&self) -> String {
        r#"
═══════════════════════════════════════════════════════════════
IMPORTANT LEGAL DISCLAIMER
═══════════════════════════════════════════════════════════════

This contract template is provided for informational purposes only 
and does not constitute legal advice. The generator is not a lawyer 
or law firm and is not a substitute for professional legal counsel.

BEFORE SIGNING THIS AGREEMENT:

1. HAVE A LAWYER REVIEW: Consult with a qualified attorney licensed 
   in your jurisdiction to ensure this contract meets your specific 
   needs and complies with local laws.

2. CUSTOMIZE FOR YOUR SITUATION: This is a template that may need 
   modifications to fit your unique circumstances.

3. UNDERSTAND THE TERMS: Do not sign any contract you do not fully 
   understand. Ask questions and seek clarification.

4. CONSIDER LOCAL LAWS: Contract laws vary by state, country, and 
   jurisdiction. What works in one place may not work in another.

5. COMPLEX TRANSACTIONS: For high-value, complex, or high-risk 
   agreements, always work with a qualified attorney.

By using this contract, you acknowledge that you understand this is a 
template only and assume all responsibility for its use. The creators 
of this tool disclaim all liability for any disputes, losses, or 
damages arising from the use of this template.

Last Updated: 2026
"#.to_string()
    }
}
