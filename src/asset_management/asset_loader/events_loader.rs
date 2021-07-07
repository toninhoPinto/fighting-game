use std::{collections::HashMap, fs};

use serde_json::to_string;

use crate::game_logic::events::{Challenge, Cost, Event, EventType, Rewards};

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventJson {
    pub id: u32,
    pub text: String,
    #[serde(rename = "on_completion_text")]
    pub on_completion_text: Option<String>,
    #[serde(rename = "on_failure_text")]
    pub on_failure_text: Option<String>,
    #[serde(rename = "on_refusal_text")]
    pub on_refusal_text: Option<String>,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "portrait_id")]
    pub portrait_id: String,
    pub rewards: Option<RewardsJson>,
    pub challenge: Option<ChallengeJson>,
    pub cost: Option<CostJson>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardsJson {
    pub currency: Option<i32>,
    #[serde(rename = "item_ids")]
    #[serde(default)]
    pub item_ids: Vec<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeJson {
    pub target: i32,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CostJson {
    pub energy: Option<i32>,
    pub health: Option<i32>,
    pub currency: Option<i32>,
    pub items: Option<Vec<i32>>,
}

pub fn load_events(dir: String) -> HashMap<u32, Event>{
    let json_string = fs::read_to_string(dir.clone()).unwrap();
    let v = &serde_json::from_str::<Vec<EventJson>>(&json_string).unwrap();
    
    let mut map = HashMap::new();

    for event_json in v.iter() {

        map.insert(event_json.id, Event{
            id: event_json.id as i32,
            event_type: match event_json.event_type.clone().as_str() {
                "Challenge" => EventType::Challenge,
                "TradeOffer" => EventType::TradeOffer,
                "LevelMod" => EventType::LevelMod,
                "WorldMod" => EventType::WorldMod,
                _ => EventType::Challenge,
            },
            text: handle_text_and_details(event_json.text.clone(), &event_json.challenge),
            portrait_id: event_json.portrait_id.clone(),
            rewards: if let Some(rewards) = &event_json.rewards { 
                Some(Rewards {
                    currency: option_or_0 (rewards.currency),
                    item_ids: rewards.item_ids.clone(),
                })
            } else {
                None
            },
            details: if let Some(details) = &event_json.challenge { 
                Some(Challenge {
                    target: details.target,
                })
            } else {
                None
            },
            cost: if let Some(cost) = &event_json.cost { 
                Some(Cost {
                        health: option_or_0(cost.health),
                        currency: option_or_0(cost.currency),
                        items: if let Some(items) = &cost.items {
                            items.clone()
                        } else {
                            Vec::new()
                        },
                })
            } else {
                None
            },
            on_completion_text: event_json.on_completion_text.clone(),
            on_failure_text: event_json.on_failure_text.clone(),
            on_refusal_text: event_json.on_refusal_text.clone(),
        });
    }   

    map
}

fn option_or_0(val: Option<i32>) -> i32 {
    if let Some(val) = val {val as i32} else {0}
}

fn handle_text_and_details(mut desc: String, details: &Option<ChallengeJson>) -> String {
    if let Some(details) =  details {

        let replace_index = desc.find('*').unwrap_or(desc.len());

        // Replace the range up until the β from the string
        desc.replace_range(replace_index..replace_index+1, &details.target.to_string());
    }

    return desc;
}