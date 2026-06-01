use crate::client::JiraClient;
use crate::error::JariError;
use crate::models::transition::*;
use serde_json::json;

impl JiraClient {
    pub async fn list_transitions(&self, key: &str) -> Result<Vec<Transition>, JariError> {
        let path = format!("/issue/{}/transitions", key);
        let response: TransitionsResponse = self.get(&path).await?;
        Ok(response.transitions)
    }

    pub async fn do_transition(
        &self,
        key: &str,
        transition_id_or_name: &str,
        comment: Option<&str>,
        resolution: Option<&str>,
    ) -> Result<TransitionResult, JariError> {
        let transitions = self.list_transitions(key).await?;

        let resolved_id = resolve_transition_id(transition_id_or_name, &transitions)?;

        let mut body = json!({
            "transition": { "id": resolved_id }
        });

        if let Some(comment_text) = comment {
            let adf = crate::adf::from_markdown::markdown_to_adf(comment_text)?;
            body["update"] = json!({
                "comment": [
                    {
                        "add": {
                            "body": adf
                        }
                    }
                ]
            });
        }

        if let Some(res) = resolution {
            body["fields"] = json!({
                "resolution": { "name": res }
            });
        }

        let path = format!("/issue/{}/transitions", key);

        let response: serde_json::Value = self.post(&path, &body).await?;

        let transition_name = response
            .get("transition")
            .and_then(|t| t.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("");

        let from_status = response
            .get("from")
            .and_then(|f| f.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("");

        let to_status = response
            .get("to")
            .and_then(|t| t.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("");

        Ok(TransitionResult {
            transition: transition_name.to_string(),
            from_status: from_status.to_string(),
            to_status: to_status.to_string(),
        })
    }
}

fn resolve_transition_id(
    id_or_name: &str,
    transitions: &[Transition],
) -> Result<String, JariError> {
    if id_or_name.chars().all(|c| c.is_ascii_digit()) {
        return Ok(id_or_name.to_string());
    }

    let lower = id_or_name.to_lowercase();

    let exact = transitions.iter().find(|t| t.name.to_lowercase() == lower);
    if let Some(t) = exact {
        return Ok(t.id.clone());
    }

    let starts_with = transitions
        .iter()
        .find(|t| t.name.to_lowercase().starts_with(&lower));
    if let Some(t) = starts_with {
        return Ok(t.id.clone());
    }

    let contains = transitions
        .iter()
        .find(|t| t.name.to_lowercase().contains(&lower));
    if let Some(t) = contains {
        return Ok(t.id.clone());
    }

    let available: Vec<String> = transitions.iter().map(|t| t.name.clone()).collect();
    Err(JariError::Cli(format!(
        "No transition matching '{}'. Available transitions: {}",
        id_or_name,
        available.join(", ")
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_numeric_id() {
        let transitions = vec![];
        let result = resolve_transition_id("123", &transitions).unwrap();
        assert_eq!(result, "123");
    }

    #[test]
    fn test_resolve_exact_name() {
        let transitions = vec![Transition {
            id: "11".into(),
            name: "In Progress".into(),
            to: TransitionDestination {
                name: "In Progress".into(),
                id: "3".into(),
                status_category: None,
            },
            has_screen: None,
            is_conditional: None,
        }];
        let result = resolve_transition_id("In Progress", &transitions).unwrap();
        assert_eq!(result, "11");
    }

    #[test]
    fn test_resolve_case_insensitive() {
        let transitions = vec![Transition {
            id: "11".into(),
            name: "In Progress".into(),
            to: TransitionDestination {
                name: "In Progress".into(),
                id: "3".into(),
                status_category: None,
            },
            has_screen: None,
            is_conditional: None,
        }];
        let result = resolve_transition_id("in progress", &transitions).unwrap();
        assert_eq!(result, "11");
    }

    #[test]
    fn test_resolve_starts_with() {
        let transitions = vec![Transition {
            id: "21".into(),
            name: "Start Progress".into(),
            to: TransitionDestination {
                name: "In Progress".into(),
                id: "3".into(),
                status_category: None,
            },
            has_screen: None,
            is_conditional: None,
        }];
        let result = resolve_transition_id("sta", &transitions).unwrap();
        assert_eq!(result, "21");
    }

    #[test]
    fn test_resolve_contains() {
        let transitions = vec![Transition {
            id: "31".into(),
            name: "Close Issue".into(),
            to: TransitionDestination {
                name: "Done".into(),
                id: "4".into(),
                status_category: None,
            },
            has_screen: None,
            is_conditional: None,
        }];
        let result = resolve_transition_id("ose", &transitions).unwrap();
        assert_eq!(result, "31");
    }

    #[test]
    fn test_resolve_no_match() {
        let transitions = vec![Transition {
            id: "11".into(),
            name: "Done".into(),
            to: TransitionDestination {
                name: "Done".into(),
                id: "5".into(),
                status_category: None,
            },
            has_screen: None,
            is_conditional: None,
        }];
        let result = resolve_transition_id("invalid", &transitions);
        assert!(result.is_err());
    }
}
