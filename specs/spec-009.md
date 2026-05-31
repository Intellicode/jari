# Spec 009: Client — Get Issue

- [ ] Not implemented

## Goal

Implement the `get_issue` API method to retrieve a single issue by key with optional field expansion.

## Requirements

- `async fn get_issue(&self, key: &str, fields: Option<&[String]>) -> Result<Issue>`
- HTTP `GET /rest/api/3/issue/{key}`
- Default fields included: `summary,status,assignee,priority,issuetype,created,updated,duedate,labels,description,comment,project,issuelinks,subtasks,parent,reporter,resolution,timetracking,votes,watches,worklog,fixVersions,versions,components`
- If `fields` parameter is provided, only those fields are requested (comma-separated query param)
- `description` field comes back as ADF JSON from Jira — stored raw, conversion happens elsewhere
- Parse `ErrorCollection` from error responses with status 400/404
- Return `NotFound` error for 404
- Return `Validation` error for 400 with Jira error details
- Defined in `src/client/issues.rs`
