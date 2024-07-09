#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use anyhow::Result;
use dotenv::dotenv;
use httpc_test::Client;
use serde_json::json;

pub async fn test_todos(hc: &Client) -> Result<()> {
    create_todo(hc).await?;
    create_todo(hc).await?;
    get_todo(hc).await?;
    delete_todo(hc).await?;
    list_todos(hc).await?;

    Ok(())
}

async fn create_todo(hc: &Client) -> Result<()> {
    let req = hc.do_post(
        "/todos",
        json!({
            "title": "todo AAA"
        }),
    );
    req.await?.print().await?;
    Ok(())
}

async fn list_todos(hc: &Client) -> Result<()> {
    let req = hc.do_get("/todos");
    req.await?.print().await?;
    Ok(())
}

async fn get_todo(hc: &Client) -> Result<()> {
    let req = hc.do_get("/todos/0");
    req.await?.print().await?;
    Ok(())
}

async fn delete_todo(hc: &Client) -> Result<()> {
    let req = hc.do_delete("/todos/0");
    req.await?.print().await?;
    Ok(())
}
