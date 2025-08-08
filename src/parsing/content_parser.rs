use std::io::Error;
// ===========================================================
// File: content_parser.rs
// Description: 
// Author: BEKs <ebaah72@gmail.com>
// Created: 05/08/2025
// ===========================================================
use crate::parsing::parsed_node::{ParsedBlock, ParsedNode};
use serde_json::Value;
use crate::parsing::types::{Directive, Populator, PopulatorTypes, VisibilityRule, VisibilityRuleTypes};
use regex::Regex;

pub fn parse_content(json_string: String) -> Result<ParsedNode, Box<dyn std::error::Error>> {
    let root :Value = serde_json::from_str(&json_string)?;
    let mut node = ParsedNode::new();
    
    if let Some(content) = root["content"].as_array() {
        
        for (index,item) in content.iter().enumerate() {
            // check outer layer for processable directives
            if let Some(vr) = parse_visibility_rule(item) {
                let mut block = ParsedBlock::new();
                block.directive(vr);
                block.path(format!("/content/{}", index));
                node.add_block(block);
            }
            if let Some(populator) = parse_populator(item).ok() {
                let mut block = ParsedBlock::new();
                block.directive(populator);
                block.path(format!("/content/{}", index));
                node.add_block(block);
            }
                                        
            // enter inner lists for possible processors
            if item.as_object().is_some() {
                let list = item.get("list").and_then(Value::as_array);
                list.map(|values| {
                    values.iter().for_each(|v| {
                        
                    })
                });
            }
        }
    }


    Ok(ParsedNode::new())
}

fn parse_visibility_rule(json: &Value) -> Option<Directive> {
    let rule = json.pointer("/_libProcess/_visibility/_rule")
        .and_then(|v| v.as_str());
    if let Some(r) = rule {
        let rule: VisibilityRuleTypes = VisibilityRuleTypes::from_str(r);

        let value = json.pointer("/_libProcess/_visibility/_params")
            .and_then(|v| v.as_array())
            .and_then(|arr| {
                let collected: Option<Vec<String>> = arr.iter()
                    .map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
                collected
            });

        return Some(
            Directive::VisibilityDirective(
                VisibilityRule::new(rule, value.unwrap_or(Vec::new()))
            )
        );
    }
    None
}

fn parse_populator(json: &Value) -> Result<Directive,Box<dyn std::error::Error>> {
    let api = json.pointer("/_libProcess/_populate/_api")
        .and_then(|v| v.as_str());
    if let Some(r) = api {
        let re = Regex::new(r"^([^\(]+)(?:\(([^)]*)\))?$")?;
        let caps = re.captures(r);
        let mut api_name = String::new();
        let mut api_param: Vec<String> = vec![];
        if let Some(caps) = caps {
            api_name = caps[1].to_string();
            api_param.push(caps[2].to_string());
        }
        let limit = json.pointer("/_libProcess/_visibility/_max")
            .and_then(|v| v.as_number())
            .and_then(|n| n.as_u64())
            .and_then(|n| Some(n as u8));

        return Ok(
            Directive::PopulatorDirective(
                Populator::new(PopulatorTypes::from_str(api_name.as_str()), api_param, limit)
            )
        );
    }
    Err("no populator api found".into())
}