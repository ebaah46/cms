use std::str::FromStr;
// ===========================================================
// File: types.rs
// Description: 
// Author: BEKs <ebaah72@gmail.com>
// Created: 06/08/2025
// ===========================================================

#[derive(Debug, Clone)]
pub struct Populator {
    pub api: PopulatorTypes,
    pub params: Vec<String>,
    pub limit: Option<u8>,
}
impl Populator {
    pub fn new(api: PopulatorTypes, params: Vec<String>, limit: Option<u8>) -> Self {
        Self {
            api,
            params,
            limit,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VisibilityRule {
    pub rule: VisibilityRuleTypes,
    pub params: Vec<String>,
    has_match: bool,
}

impl VisibilityRule {
    pub fn new (rule: VisibilityRuleTypes, params: Vec<String>) -> Self {
        Self {
            rule,
            params,
            has_match: false
        }
    }
}

#[derive(Debug, Clone)]
pub enum Directive {
    PopulatorDirective(Populator),
    VisibilityDirective(VisibilityRule),
    Unknown(String)
}

#[derive(Clone, Debug)]
pub enum DependencyType {
    Favorites,
    Subscriptions,
    Games,
    Categories,
    Profiles,
    Unknown
}


#[derive(Debug, Clone)]
pub enum PopulatorTypes {
    UserFavoriteGames,
    MostPlayedGames,
    UserMostPlayedGames,
    RecommendedGames,
    SubscriptionOffers,
    ListGames(String),
    Categories,
    BackendGames(String),
    Unknown(String)
}

impl PopulatorTypes {
    pub fn from_str(s: &str) -> Self {

        match s {
            "userFavoriteGames" => Self::UserFavoriteGames,
            "mostPlayedGames" => Self::MostPlayedGames,
            "userMostPlayedGames" => Self::UserMostPlayedGames,
            "recommendedGames" => Self::RecommendedGames,
            "subscriptionOffers" => Self::SubscriptionOffers,
            "listGames" => Self::ListGames(String::new()),
            "categories" => Self::Categories,
            "backendGames" => Self::BackendGames(String::new()),
            _ => Self::Unknown(s.to_string())
        }
    }
}
#[derive(Clone, Debug)]
pub enum VisibilityRuleTypes {
    OfferStateMatch,
    ClientProfileMatch,
    UserPropertyMatch,
    Unknown(String)
}

impl VisibilityRuleTypes {
    pub fn from_str(s: &str) -> Self {
        match s {
            "offerStateMatch" => Self::OfferStateMatch,
            "clientProfileMatch" => Self::ClientProfileMatch,
            "userPropertyMatch" => Self::UserPropertyMatch,
            _ => Self::Unknown(s.to_string())
        }
    }
}


