use std::future::Future;

use dioxus::{core::AnyhowContext, CapturedError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ContentResponse<T> {
    total: i32,
    skip: i32,
    limit: i32,
    items: Vec<ContentItem<T>>,
}

#[derive(Deserialize)]
pub struct ContentItemSys {
    id: String,
}

#[derive(Deserialize)]
pub struct ContentTagSys {
    id: String,
    #[serde(rename = "linkType")]
    _link_type: String,
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Deserialize)]
pub struct ContentTag {
    sys: ContentTagSys,
}

#[derive(Deserialize)]
pub struct ContentMetadata {
    tags: Vec<ContentTag>,
}

#[derive(Deserialize)]
pub struct ContentItem<T> {
    metadata: ContentMetadata,
    sys: ContentItemSys,
    fields: T,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Project {
    #[serde(skip_deserializing)]
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(skip_deserializing)]
    pub tags: Vec<String>,
    pub badge: Option<String>,
    pub featured: bool,
    pub github: Option<String>,
    pub live: Option<String>,
}

pub fn fetch_projects(
    limit: Option<i32>,
    skip: Option<i32>,
) -> impl Future<Output = dioxus::Result<Vec<Project>, CapturedError>> {
    async move {
        let response = reqwest::get(format!("https://cdn.contentful.com/spaces/8310jhjcmnzt/environments/master/entries?access_token=xIO5R3Op8af4a3PjVZ9qdb1WVlBpTe7W1TAcJe44AWg&content_type=project&limit={}&skip={}", limit.unwrap_or(5), skip.unwrap_or(0)))
        .await.context("Failed to fetch projects")?
        .json::<ContentResponse<Project>>()
        .await.context("Failed to parse projects")?;

        Ok(response
            .items
            .into_iter()
            .map(|item| Project {
                tags: item
                    .metadata
                    .tags
                    .into_iter()
                    .map(|tag| tag.sys.id)
                    .collect(),
                id: item.sys.id,
                ..item.fields
            })
            .collect::<Vec<Project>>())
    }
}

pub fn fetch_project(id: String) -> impl Future<Output = dioxus::Result<Project, CapturedError>> {
    async move {
        let response = reqwest::get(format!("https://cdn.contentful.com/spaces/8310jhjcmnzt/environments/master/entries/{}?access_token=xIO5R3Op8af4a3PjVZ9qdb1WVlBpTe7W1TAcJe44AWg", id))
        .await.context("Failed to fetch project")?
        .json::<ContentItem<Project>>()
        .await.context("Failed to parse project")?;

        Ok(Project {
            tags: response.metadata.tags.into_iter().map(|tag| tag.sys.id).collect::<Vec<String>>(),
            id: response.sys.id,
            ..response.fields
        })
    }
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Til {
    #[serde(skip_deserializing)]
    pub id: String,
    pub title: String,
    #[serde(skip_deserializing)]
    pub tags: Vec<String>,
    pub date: String,
    pub article: String,
}

pub fn fetch_tils(
    limit: Option<i32>,
    skip: Option<i32>,
) -> impl Future<Output = dioxus::Result<Vec<Til>, CapturedError>> {
    async move {
        let response = reqwest::get(format!("https://cdn.contentful.com/spaces/8310jhjcmnzt/environments/master/entries?access_token=xIO5R3Op8af4a3PjVZ9qdb1WVlBpTe7W1TAcJe44AWg&content_type=til&limit={}&skip={}", limit.unwrap_or(5), skip.unwrap_or(0)))
    .await.context("Failed to fetch tils")?
    .json::<ContentResponse<Til>>()
    .await.context("Failed to parse tils")?;

        Ok(response
            .items
            .into_iter()
            .map(|item| Til {
                tags: item
                    .metadata
                    .tags
                    .into_iter()
                    .map(|tag| tag.sys.id)
                    .collect(),
                id: item.sys.id,
                ..item.fields
            })
            .collect::<Vec<Til>>())
    }
}

pub fn fetch_til(id: String) -> impl Future<Output = dioxus::Result<Til, CapturedError>> {
    async move {
        let response = reqwest::get(format!("https://cdn.contentful.com/spaces/8310jhjcmnzt/environments/master/entries/{}?access_token=xIO5R3Op8af4a3PjVZ9qdb1WVlBpTe7W1TAcJe44AWg", id))
            .await
            .context("Failed to fetch til")?
            .json::<ContentItem<Til>>()
            .await
            .context("Failed to parse til")?;

        Ok(Til {
            tags: response
                .metadata
                .tags
                .into_iter()
                .map(|tag| tag.sys.id)
                .collect::<Vec<String>>(),
            id: response.sys.id,
            ..response.fields
        })
    }
}
