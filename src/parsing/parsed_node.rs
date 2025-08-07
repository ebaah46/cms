// ===========================================================
// File: parsed_node.rs
// Description: 
// Author: BEKs <ebaah72@gmail.com>
// Created: 06/08/2025
// ===========================================================

use serde::{Deserialize, Serialize};
use crate::parsing::types::Directive;
use crate::parsing::types::DependencyType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedNode {
    pub name: String,
    pub blocks: Vec<ParsedBlock>,
    pub source: String,
}

impl ParsedNode {
    pub fn new()-> Self {
        Self{
            name: String::new(),
            blocks: Vec::new(),
            source: String::new(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedBlock {
    pub path : Option<String>,
    pub directive: Directive,
    pub can_update: bool,
    pub processed: bool,
    pub updated_by: DependencyType
}

impl ParsedBlock {
    pub fn new()-> Self {
        Self{
            path: None,
            directive: Directive::Unknown,
            can_update: false,
            processed: false,
            updated_by: DependencyType::Unknown,
        }
    }

    pub fn path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    pub fn directive(mut self, directive: Directive) -> Self {
        self.directive = directive;
        self
    }

    pub fn can_update(mut self, can_update: bool) -> Self {
        self.can_update = can_update;
        self
    }

    pub fn processed(mut self, processed: bool) -> Self {
        self.processed = processed;
        self
    }

    pub fn updated_by(mut self, updated_by: DependencyType) -> Self {
        self.updated_by = updated_by;
        self
    }
}
