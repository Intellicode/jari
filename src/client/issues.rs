use crate::client::JiraClient;
use crate::error::JariError;
use crate::models::issue::*;
use serde_json::json;
use std::collections::HashMap;

impl JiraClient {
    pub async fn get_issue(
        &self,
        key: &str,
        fields: Option<&[String]>,
    ) -> Result<Issue, JariError> {
        let path = if let Some(fields) = fields {
            format!("/issue/{}?fields={}", key, fields.join(","))
        } else {
            format!("/issue/{}?fields=summary,status,assignee,priority,issuetype,created,updated,duedate,labels,description,comment,project,issuelinks,subtasks,parent,reporter,resolution,timetracking,votes,watches,worklog,fixVersions,versions,components", key)
        };
        self.get(&path).await
    }

    pub async fn create_issue(
        &self,
        request: &CreateIssueRequest,
    ) -> Result<CreatedIssue, JariError> {
        let mut fields = json!({
            "project": { "key": request.project },
            "summary": request.summary,
            "issuetype": { "name": request.issuetype },
        });

        if let Some(ref adf) = request.description_adf {
            fields["description"] = adf.clone();
        }
        if let Some(ref priority) = request.priority {
            fields["priority"] = json!({ "name": priority });
        }
        if let Some(ref assignee) = request.assignee {
            if assignee.contains('@') {
                fields["assignee"] = json!({ "name": assignee });
            } else {
                fields["assignee"] = json!({ "id": assignee });
            }
        }
        if let Some(ref labels) = request.labels {
            fields["labels"] = json!(labels);
        }
        if let Some(ref parent) = request.parent {
            fields["parent"] = json!({ "key": parent });
        }
        if let Some(ref epic_link) = request.epic_link {
            fields["customfield_10014"] = json!(epic_link);
        }

        let body = json!({ "fields": fields });

        self.post("/issue", &body).await
    }

    pub async fn edit_issue(
        &self,
        key: &str,
        fields: &HashMap<String, serde_json::Value>,
    ) -> Result<(), JariError> {
        let body = json!({ "fields": fields });
        let path = format!("/issue/{}", key);
        self.put_no_body(&path, &body).await
    }

    pub async fn delete_issue(
        &self,
        key: &str,
    ) -> Result<(), JariError> {
        let path = format!("/issue/{}", key);
        self.delete(&path).await
    }

    pub async fn assign_issue(
        &self,
        key: &str,
        user: &str,
    ) -> Result<(), JariError> {
        let body = if user.contains('@') {
            json!({ "name": user })
        } else {
            json!({ "accountId": user })
        };
        let path = format!("/issue/{}/assignee", key);
        self.put_no_body(&path, &body).await
    }

    pub async fn add_watcher(
        &self,
        key: &str,
        account_id: &str,
    ) -> Result<(), JariError> {
        let body = json!({ "accountId": account_id });
        let path = format!("/issue/{}/watchers", key);
        self.post_no_body(&path, &body).await
    }

    pub async fn remove_watcher(
        &self,
        key: &str,
        account_id: &str,
    ) -> Result<(), JariError> {
        let path = format!("/issue/{}/watchers?accountId={}", key, account_id);
        self.delete(&path).await
    }
}
