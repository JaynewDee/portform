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

#[derive(Debug)]
pub struct DocumentData {
    pub filename: String,
    pub title: String,
    pub header: Header,
    pub summary: Summary,
    pub employment_history: EmploymentHistory,
    pub projects: Projects,
    pub contact_details: ContactDetails,
    pub skillset: SkillSet,
    pub certifications: Certifications,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub name: String,
    pub profession: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Summary {
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContactDetails {
    pub email: String,
    pub website: String,
    pub phone: String,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryEntry {
    pub position: String,
    pub location: String,
    pub dates_employed: (String, String),
    pub description: String,
}

pub type EmploymentHistory = Vec<HistoryEntry>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skill {
    pub name: String,
}

pub type SkillSet = Vec<Skill>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Certification {
    pub date_issued: String,
    pub name: String,
}

pub type Certifications = Vec<Certification>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub deployment: String,
}

pub type Projects = Vec<Project>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EducationEntry {
    pub dates: (String, String),
    pub name: String,
    pub location: String,
    pub description: String,
}

pub type Education = Vec<EducationEntry>;

struct Font;
// style?
// size?
