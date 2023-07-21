#![allow(dead_code)]
// General Sections
struct DocumentShape {
    header: Header,
    summary: Summary,
    employment_history: EmploymentHistory,
    projects: Projects,
    contact_details: ContactDetails,
    skillset: SkillSet,
    certifications: Certifications,
}

struct Header {
    name: String,
    profession: String,
}

struct Summary {
    body: String,
}

struct ContactDetails {
    email: String,
    website: String,
    phone: String,
    address: String,
}

struct HistoryEntry {
    position: String,
    location: String,
    dates_employed: (String, String),
    descriptions: Vec<String>,
}

type EmploymentHistory = Vec<HistoryEntry>;

struct Skill {
    name: String,
}

type SkillSet = Vec<Skill>;
struct Certification {
    date_issued: String,
    name: String,
}

type Certifications = Vec<Certification>;
struct Project {
    name: String,
    descriptions: Vec<String>,
}

type Projects = Vec<Project>;

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
