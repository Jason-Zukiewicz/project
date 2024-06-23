/*  #region      ------------------------------- [ IMPORTS ] ------------------------------------------  */

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use anyhow::Result;
use dotenv::dotenv;
use serde_json::json;

/*  #endregion   ------------------------------- [ IMPORTS ] ------------------------------------------  */

/*  #region      ------------------------------- [ TEST ] ------------------------------------------  */

#[tokio::test]
async fn quick_dev() -> Result<()> {
    dotenv().ok();
    let BACKEND_PORT = std::env::var("BACKEND_PORT").expect("BACKEND_PORT must be set.");
    let hc = httpc_test::new_client(format!("http://localhost:{}", BACKEND_PORT))?;

    //$ Login
    let req_login = hc.do_post(
        "/login",
        json!({
            "username": "user",
            "password": "pass"
        }),
    );
    req_login.await?.print().await?;

    //$ Todo
    let req_create_todo = hc.do_post(
        "/todos",
        json!({
            "title": "todo AAA"
        }),
    );
    req_create_todo.await?.print().await?;

    hc.do_get("/todos").await?.print().await?;

    Ok(())
}

/*  #endregion   ------------------------------- [ Test ] ------------------------------------------  */
