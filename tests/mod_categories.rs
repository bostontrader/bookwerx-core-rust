use bookwerx_core_rust::routes as R;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

// Examine categories
pub fn categories(client: &Client, apikey: &String, accounts: &Vec<R::AccountJoined>) -> Vec<R::Category> {

    // 1. GET /categories. sb 200, empty array
    let mut response = client.get(format!("/categories?apikey={}", &apikey))
        .dispatch();
    let v: Vec<R::Category> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 2. Try to post a new category.

    // 2.1. But first post using a non-existent apikey. 200 and db error.
    response = client.post("/categories")
        .body("apikey=notarealkey&symbol=QTL&title=Quatloo")
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);

    // 2.2 Successful post. 200  and InsertSuccess
    response = client.post("/categories")
        .body(format!("apikey={}&symbol=QTL&title=Quatloo", apikey))
        .header(ContentType::Form)
        .dispatch();
    let li: R::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(li.data.last_insert_id > 0);

    // 2.3 Successful put. 200  and UpdateSuccess
    response = client.put("/categories")
        .body(format!("apikey={}&id={}&symbol=QTL&title=Quatloo", apikey, li.data.last_insert_id))
        .header(ContentType::Form)
        .dispatch();
    let r: R::UpdateSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() > 0);

    // 3. Now verify that there's a single category.
    response = client.get(format!("/categories?apikey={}", &apikey))
        .dispatch();
    let v: Vec<R::Category> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 1);

    // 4. Try to post a category with a duplicated symbol. 200 db error.
    response = client.post("/categories")
        .body(format!("apikey={}&symbol=QTL&title=Quatloo", apikey))
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);

    // 5. Now submit a single GET of the prior POST. 200 and category.
    response = client.get(format!("/category/{}/?apikey={}", li.data.last_insert_id, apikey))
        .dispatch();
    // The mere fact that this successfully parses a category _is_ the test
    let c: R::Category = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);

    // 6. Make a 2nd Successful post. 200.
    response = client.post("/categories")
        .body(format!("apikey={}&symbol=XAU&title=Quatloo", apikey))
        .header(ContentType::Form)
        .dispatch();
    let r: R::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.last_insert_id > 0);

    // 6.1 Verify that there are now two categories
    response = client.get(format!("/categories?apikey={}", &apikey)).dispatch();
    let v: Vec<R::Category> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 2);

    v

}