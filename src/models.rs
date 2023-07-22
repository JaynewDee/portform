#![allow(dead_code)]

use serde::{Deserialize, Serialize};
// General Sections
#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentShape {
    header: Option<Header>,
    summary: Option<Summary>,
    employment_history: Option<EmploymentHistory>,
    projects: Option<Projects>,
    contact_details: Option<ContactDetails>,
    skillset: Option<SkillSet>,
    certifications: Option<Certifications>,
}

impl Default for DocumentShape {
    fn default() -> Self {
        Self {
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
struct Header {
    name: String,
    profession: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Summary {
    body: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContactDetails {
    email: String,
    website: String,
    phone: String,
    address: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct HistoryEntry {
    position: String,
    location: String,
    dates_employed: (String, String),
    descriptions: Vec<String>,
}

type EmploymentHistory = Vec<HistoryEntry>;

#[derive(Serialize, Deserialize, Debug)]
struct Skill {
    name: String,
}

type SkillSet = Vec<Skill>;

#[derive(Serialize, Deserialize, Debug)]
struct Certification {
    date_issued: String,
    name: String,
}

type Certifications = Vec<Certification>;

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    descriptions: Vec<String>,
}

type Projects = Vec<Project>;

#[derive(Serialize, Deserialize, Debug)]
struct EducationEntry {
    dates: (String, String),
    name: String,
    location: String,
    description: String,
}

type Education = Vec<EducationEntry>;

struct Font;
// style?
// size?
