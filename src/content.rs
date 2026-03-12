// src/content.rs
//
// All portfolio data lives in /content.json — edit that file to update
// every section of the site. This module parses it once at startup using
// include_str! (embedded at compile time, zero HTTP request needed).

use serde::Deserialize;

// ── JSON shapes ───────────────────────────────────────────────────────────────

#[derive(Deserialize, Clone)]
pub struct Meta {
    pub issue:         String,
    pub edition:       String,
    pub ticker_top:    String,
    pub ticker_bottom: String,
}

#[derive(Deserialize, Clone)]
pub struct Stat {
    pub num:   String,
    pub label: String,
}

#[derive(Deserialize, Clone)]
pub struct Hero {
    pub headline_1: String,
    pub headline_2: String,
    pub body:       String,
    pub stats:      Vec<Stat>,
}

#[derive(Deserialize, Clone)]
pub struct Project {
    pub num:      String,
    pub title:    String,
    pub subtitle: String,
    pub tags:     Vec<String>,
    pub desc:     String,
    pub accent:   String,
    pub href:     String,
}

#[derive(Deserialize, Clone)]
pub struct CvEntry {
    pub years: String,
    pub role:  String,
    pub org:   String,
    pub desc:  String,
}

#[derive(Deserialize, Clone)]
pub struct SkillBucket {
    pub title: String,
    pub color: String,
    pub list:  String,
}

#[derive(Deserialize, Clone)]
pub struct Skills {
    pub buckets: Vec<SkillBucket>,
    pub marquee: String,
}

#[derive(Deserialize, Clone)]
pub struct RecItem {
    pub org:   String,
    pub award: String,
}

#[derive(Deserialize, Clone)]
pub struct Article {
    pub kind:  String,
    pub title: String,
    pub href:  String,
}

#[derive(Deserialize, Clone)]
pub struct Contact {
    pub email:           String,
    pub github_org:      String,
    pub github_personal: String,
    pub linkedin:        String,
    pub youtube:         String,
    pub medium:          String,
    pub discord:         String,
    pub buymeacoffee:    String,
}

#[derive(Deserialize, Clone)]
pub struct Footer {
    pub left:  String,
    pub right: String,
}

#[derive(Deserialize, Clone)]
pub struct Content {
    pub meta:        Meta,
    pub hero:        Hero,
    pub projects:    Vec<Project>,
    pub experience:  Vec<CvEntry>,
    pub skills:      Skills,
    pub recognition: Vec<RecItem>,
    pub articles:    Vec<Article>,
    pub contact:     Contact,
    pub footer:      Footer,
}

// ── Compile-time embed ────────────────────────────────────────────────────────

/// Returns the parsed content.json, embedded at compile time.
/// If the JSON is malformed the build will panic with a clear error.
pub fn load() -> Content {
    const RAW: &str = include_str!("../content.json");
    serde_json::from_str(RAW).expect("content.json is invalid JSON — check syntax and re-build")
}