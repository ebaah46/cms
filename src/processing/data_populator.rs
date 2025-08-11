// ===========================================================
// File: data_populator
// Description: 
// Author: BEKs <ebaah72@gmail.com>
// Created: 09/08/2025
// ===========================================================

use std::collections::HashMap;
use crate::parsing::types::{DependencyType, Directive};


pub trait DataPopulator {
    fn process(self, directive: &Directive) -> Result<(), Box<dyn std::error::Error>>;

    fn get_result(self) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>>;
}


struct FavoritesPopulator {
    data: Vec<HashMap<String, String>>,
}

impl DataPopulator for FavoritesPopulator {
    fn process(self,  directive: &Directive) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn get_result(self) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>>{
        Ok(self.data)
    }
}

struct ListGamesPopulator {
    data: Vec<HashMap<String, String>>,
}

impl DataPopulator for ListGamesPopulator {
    fn process(self, directive: &Directive) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn get_result(self) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
        todo!()
    }
}

impl ListGamesPopulator {

    fn parse_parameters(self, params: String) -> Vec<String>{
        let all_params = params.split("=").map(|s| s.to_string()).collect();
        all_params
    }
}