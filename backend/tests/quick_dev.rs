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
    let BACKEND_ADDRESS = std::env::var("BACKEND_ADDRESS").expect("BACKEND_ADDRESS must be set.");
    let BACKEND_VEC:  Vec<&str> = BACKEND_ADDRESS.splitn(2, ':').collect();
    let BACKEND_PORT = BACKEND_VEC[1];


    let hc = httpc_test::new_client(format!("http://localhost:{}", BACKEND_PORT))?;

    //# LOGIN
    let req_login = hc.do_post("/login", 
            json!({
                "username": "user", 
                "password": "pass"
            }));
    req_login.await?.print().await?;

    //# TICKET
    let req_create_todo = hc.do_post("/todos", 
                json!({
                    "title": "Todo AAA"
                }),
        );
    req_create_todo.await?.print().await?;
    
    hc.do_get("/todos").await?.print().await?;

    hc.do_get("/todos/0").await?.print().await?;
    



    Ok(())
}

/*  #endregion   ------------------------------- [ Test ] ------------------------------------------  */
