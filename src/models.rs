#![allow(dead_code)]

use serde::{Deserialize, Serialize};
// General Sections
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentShape {
    pub filename: Option<String>,
    pub title: Option<String>,
    pub header: Option<Header>,
    pub summary: Option<Summary>,
    pub employment_history: Option<EmploymentHistory>,
    pub projects: Option<Projects>,
    pub contact_details: Option<ContactDetails>,
    pub skillset: Option<SkillSet>,
    pub certifications: Option<Certifications>,
}

impl Default for DocumentShape {
    fn default() -> Self {
        Self {
            filename: Some("My_Current_Resume.pdf".to_string()),
            title: Some("Joshua_Diehl_Software_Professional".to_string()),
            header: None,
            summary: None,
            employment_history: None,
            projects: None,
            contact_details: None,
            skillset: None,
            certifications: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub name: String,
    pub profession: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Summary {
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactDetails {
    email: String,
    website: String,
    phone: String,
    address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryEntry {
    pub position: String,
    pub location: String,
    pub dates_employed: (String, String),
    pub description: String,
}

pub type EmploymentHistory = Vec<HistoryEntry>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    name: String,
}

pub type SkillSet = Vec<Skill>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Certification {
    date_issued: String,
    name: String,
}

pub type Certifications = Vec<Certification>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub deployment: String,
}

pub type Projects = Vec<Project>;

#[derive(Serialize, Deserialize, Debug)]
pub struct EducationEntry {
    dates: (String, String),
    name: String,
    location: String,
    description: String,
}

pub type Education = Vec<EducationEntry>;

struct Font;
// style?
// size?
