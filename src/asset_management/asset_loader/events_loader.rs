use std::{collections::HashMap, fs};

use crate::game_logic::events::{Cost, Details, Event, Rewards};

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventJson {
    pub id: u32,
    pub text: String,
    #[serde(rename = "portrait_id")]
    pub portrait_id: String,
    pub options: Vec<String>,
    pub rewards: Option<RewardsJson>,
    pub details: Option<DetailsJson>,
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
pub struct DetailsJson {
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
            text: handle_text_and_details(event_json.text.clone(), &event_json.details),
            portrait_id: event_json.portrait_id.clone(),
            options: event_json.options.clone(),
            rewards: if let Some(rewards) = &event_json.rewards { 
                Some(Rewards {
                    currency: option_or_0 (rewards.currency),
                    item_ids: rewards.item_ids.clone(),
                })
            } else {
                None
            },
            details: if let Some(details) = &event_json.details { 
                Some(Details {
                    target: details.target,
                })
            } else {
                None
            },
            cost: if let Some(cost) = &event_json.cost { 
                Some(Cost {
                        health: option_or_0(cost.health),
                        energy: option_or_0(cost.energy),
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
        });
    }   

    map
}

fn option_or_0(val: Option<i32>) -> i32 {
    if let Some(val) = val {val as i32} else {0}
}

fn handle_text_and_details(mut desc: String, details: &Option<DetailsJson>) -> String {
    if let Some(details) =  details {

        let replace_index = desc.find('*').unwrap_or(desc.len());

        // Replace the range up until the β from the string
        desc.replace_range(replace_index..replace_index+1, &details.target.to_string());
    }

    return desc;
}