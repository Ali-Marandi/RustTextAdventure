use crate::content::{Choice, Effect, Requirement, Story, StoryNode};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub schema_version: u32,
    pub current_node: String,
    pub evidence: BTreeSet<String>,
    pub flags: BTreeMap<String, bool>,
    pub visited_nodes: BTreeSet<String>,
    pub recent_event: String,
    pub completed: bool,
}

impl GameState {
    pub const SCHEMA_VERSION: u32 = 1;

    pub fn new(story: &Story) -> Self {
        let mut state = Self {
            schema_version: Self::SCHEMA_VERSION,
            current_node: story.start_node.clone(),
            evidence: BTreeSet::new(),
            flags: BTreeMap::new(),
            visited_nodes: BTreeSet::new(),
            recent_event: "Investigation opened. Your evidence ledger is ready.".to_owned(),
            completed: false,
        };
        state.visited_nodes.insert(story.start_node.clone());
        state
    }

    pub fn current<'a>(&self, story: &'a Story) -> &'a StoryNode {
        story
            .node(&self.current_node)
            .expect("Game state must always reference a valid story node")
    }

    pub fn is_choice_available(&self, choice: &Choice) -> bool {
        choice.requires.iter().all(|requirement| match requirement {
            Requirement::Evidence { id } => self.evidence.contains(id),
            Requirement::Flag { key, value } => {
                self.flags.get(key).copied().unwrap_or(false) == *value
            }
        })
    }

    pub fn missing_requirements(&self, choice: &Choice, story: &Story) -> Vec<String> {
        choice
            .requires
            .iter()
            .filter_map(|requirement| match requirement {
                Requirement::Evidence { id } if !self.evidence.contains(id) => story
                    .evidence(id)
                    .map(|evidence| evidence.title.clone())
                    .or_else(|| Some(id.clone())),
                Requirement::Flag { key, value }
                    if self.flags.get(key).copied().unwrap_or(false) != *value =>
                {
                    Some(key.clone())
                }
                _ => None,
            })
            .collect()
    }

    pub fn choose(&mut self, story: &Story, choice_id: &str) -> Result<(), String> {
        if self.completed {
            return Err(
                "This case is complete. Start a new investigation to make another choice.".into(),
            );
        }
        let current = self.current(story);
        let choice = current
            .choices
            .iter()
            .find(|choice| choice.id == choice_id)
            .ok_or_else(|| "That option is no longer available.".to_string())?;

        if !self.is_choice_available(choice) {
            return Err(
                "That option is still locked. Review the evidence ledger for what is missing."
                    .into(),
            );
        }

        for effect in &choice.effects {
            match effect {
                Effect::Evidence { id } => {
                    self.evidence.insert(id.clone());
                    if let Some(evidence) = story.evidence(id) {
                        self.recent_event = format!("Evidence secured: {}.", evidence.title);
                    }
                }
                Effect::Flag { key, value } => {
                    self.flags.insert(key.clone(), *value);
                }
            }
        }

        self.current_node = choice.next.clone();
        self.visited_nodes.insert(self.current_node.clone());
        self.completed = self.current(story).ending;
        if self.completed {
            self.recent_event =
                "Case outcome recorded. You can start a new investigation to explore another path."
                    .to_owned();
        } else if self.recent_event.is_empty() {
            self.recent_event = format!("Moved to {}.", self.current(story).location);
        }
        Ok(())
    }

    pub fn progress(&self, story: &Story) -> f32 {
        let discoverable = story
            .nodes
            .iter()
            .filter(|node| !node.ending)
            .count()
            .max(1);
        (self.visited_nodes.len().min(discoverable) as f32 / discoverable as f32).min(1.0)
    }

    pub fn outcome_label(&self) -> Option<&str> {
        if !self.completed {
            return None;
        }
        if self
            .flags
            .get("ethical_resolution")
            .copied()
            .unwrap_or(false)
        {
            Some("Truth released")
        } else if self.flags.get("negotiated").copied().unwrap_or(false) {
            Some("Signal remains")
        } else {
            Some("Quiet option")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn story() -> Story {
        Story::load_embedded().expect("embedded story should remain valid")
    }

    #[test]
    fn story_is_valid_and_starts_at_arrival() {
        let story = story();
        let state = GameState::new(&story);
        assert_eq!(state.current_node, "arrival");
        assert!(!state.completed);
    }

    #[test]
    fn locked_choice_describes_missing_evidence() {
        let story = story();
        let mut state = GameState::new(&story);
        state.choose(&story, "enter_hall").unwrap();
        let lab = state
            .current(&story)
            .choices
            .iter()
            .find(|choice| choice.id == "to_lab")
            .unwrap();
        assert!(!state.is_choice_available(lab));
        assert_eq!(
            state.missing_requirements(lab, &story),
            vec!["Administrator key"]
        );
    }

    #[test]
    fn ethical_path_reaches_truth_ending() {
        let story = story();
        let mut state = GameState::new(&story);
        state.choose(&story, "enter_hall").unwrap();
        state.choose(&story, "to_archive").unwrap();
        state.choose(&story, "take_key").unwrap();
        state.choose(&story, "to_lab").unwrap();
        state.choose(&story, "collect_protocol").unwrap();
        state.choose(&story, "ethical_release").unwrap();
        assert!(state.completed);
        assert_eq!(state.outcome_label(), Some("Truth released"));
    }
}
