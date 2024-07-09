/*  #region      ------------------------------- [ IMPORTS ] ------------------------------------------  */

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use anyhow::Result;
use dotenv::dotenv;
use httpc_test::Client;
use serde_json::json;

mod todos;
/*  #endregion   ------------------------------- [ IMPORTS ] ------------------------------------------  */

/*  #region      ------------------------------- [ TEST ] ------------------------------------------  */

const ADDRESS: &str = "http://localhost:9999";

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client(ADDRESS)?;

    test_root(&hc).await?;
    todos::test_todos(&hc).await?;

    Ok(())
}

async fn test_root(hc: &Client) -> Result<()> {
    let req_root = hc.do_get("/");
    req_root.await?.print().await?;
    Ok(())
}
