// ===========================================================
// File: types.rs
// Description: 
// Author: BEKs <ebaah72@gmail.com>
// Created: 06/08/2025
// ===========================================================
#[macro_use]


use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Populator {
    pub api: PopulatorTypes,
    pub params: Vec<String>,
    pub limit: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibilityRule {
    pub rule: VisibilityRuleTypes,
    pub params: Vec<String>,
    has_match: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Directive {
    Populator,
    VisibilityRule,
    Unknown
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DependencyType {
    Favorites,
    Subscriptions,
    Games,
    Categories,
    Profiles,
    Unknown
}


#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VisibilityRuleTypes {
    OfferStateMatch,
    ClientProfileMatch,
    UserPropertyMatch
}


