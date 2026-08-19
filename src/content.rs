use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Deserialize)]
pub struct Story {
    pub title: String,
    pub subtitle: String,
    pub start_node: String,
    pub nodes: Vec<StoryNode>,
    pub evidence: Vec<EvidenceDefinition>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StoryNode {
    pub id: String,
    pub location: String,
    pub title: String,
    pub body: String,
    pub map_zone: String,
    #[serde(default)]
    pub ending: bool,
    #[serde(default)]
    pub choices: Vec<Choice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub hint: String,
    #[serde(default)]
    pub requires: Vec<Requirement>,
    #[serde(default)]
    pub effects: Vec<Effect>,
    pub next: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Requirement {
    Evidence { id: String },
    Flag { key: String, value: bool },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Effect {
    Evidence { id: String },
    Flag { key: String, value: bool },
}

#[derive(Debug, Clone, Deserialize)]
pub struct EvidenceDefinition {
    pub id: String,
    pub title: String,
    pub description: String,
}

impl Story {
    pub fn load_embedded() -> Result<Self, String> {
        let content = include_str!("../assets/story_en.json");
        let story: Self = serde_json::from_str(content)
            .map_err(|error| format!("Unable to parse embedded story: {error}"))?;
        story.validate()?;
        Ok(story)
    }

    pub fn node(&self, id: &str) -> Option<&StoryNode> {
        self.nodes.iter().find(|node| node.id == id)
    }

    pub fn evidence(&self, id: &str) -> Option<&EvidenceDefinition> {
        self.evidence.iter().find(|evidence| evidence.id == id)
    }

    pub fn validate(&self) -> Result<(), String> {
        let node_ids: HashSet<&str> = self.nodes.iter().map(|node| node.id.as_str()).collect();
        if !node_ids.contains(self.start_node.as_str()) {
            return Err("The start node does not exist.".into());
        }
        if node_ids.len() != self.nodes.len() {
            return Err("Story node identifiers must be unique.".into());
        }

        let evidence_ids: HashSet<&str> =
            self.evidence.iter().map(|item| item.id.as_str()).collect();
        if evidence_ids.len() != self.evidence.len() {
            return Err("Evidence identifiers must be unique.".into());
        }

        for node in &self.nodes {
            for choice in &node.choices {
                if !node_ids.contains(choice.next.as_str()) {
                    return Err(format!("Choice '{}' points to an unknown node.", choice.id));
                }
                for effect in &choice.effects {
                    if let Effect::Evidence { id } = effect {
                        if !evidence_ids.contains(id.as_str()) {
                            return Err(format!(
                                "Choice '{}' grants unknown evidence '{}'.",
                                choice.id, id
                            ));
                        }
                    }
                }
                for requirement in &choice.requires {
                    if let Requirement::Evidence { id } = requirement {
                        if !evidence_ids.contains(id.as_str()) {
                            return Err(format!(
                                "Choice '{}' requires unknown evidence '{}'.",
                                choice.id, id
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
