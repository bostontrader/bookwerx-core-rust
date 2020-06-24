use bookwerx_core_rust::db as D;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

// Examine categories
pub fn categories(client: &Client, apikey: &String) -> Vec<D::Category> {

    // 1. GET /categories.
    let mut response = client.get(format!("/categories?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetCategoryResponse::Many(v) => assert_eq!(v.len(), 0),
        _ => assert!(false)
    }

    // 2. Try to post a new category.

    // 2.1. But first post using a non-existent apikey.
    response = client.post("/categories")
        .body("apikey=notarealkey&symbol=A&title=Assets")
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false)
    }

    // 2.2 Successful post.
    response = client.post("/categories")
        .body(format!("apikey={}&symbol=A&title=Assets", apikey))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let mut lid: u64 = 0;
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid1) => { lid = lid1; assert!(lid > 0) },
        _ => assert!(false)
    }

    // 2.3 Successful put.
    response = client.put("/categories")
        .body(format!("apikey={}&id={}&symbol=A&title=Assets", apikey, lid))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Info(info) => assert_eq!(info, "(Rows matched: 1  Changed: 0  Warnings: 0"),
        _ => assert!(false)
    }

    // 3. Now verify that there's a single category.
    response = client.get(format!("/categories?apikey={}", &apikey)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetCategoryResponse::Many(v) => assert_eq!(v.len(), 1),
        _ => assert!(false)
    }

    // 4. Try to post a category with a duplicated symbol.
    response = client.post("/categories")
        .body(format!("apikey={}&symbol=A&title=Assets", apikey))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false)
    }

    // 5. Now submit a single GET of the prior POST.
    response = client.get(format!("/category/{}/?apikey={}", lid, apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetCategoryResponse::One(_) => assert!(true),
        _ => assert!(false)
    }

    // 6. Make a 2nd Successful post.
    response = client.post("/categories")
        .body(format!("apikey={}&symbol=L&title=Liabilities", apikey))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false)
    }

    // 7. Make a 3rd Successful post.
    response = client.post("/categories")
        .body(format!("apikey={}&symbol=Eq&title=Equity", apikey))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false)
    }

    // 8. Verify that there are now three categories
    response = client.get(format!("/categories?apikey={}", &apikey)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let mut ret_val = Vec::new();
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetCategoryResponse::Many(v) => {
            assert_eq!(v.len(), 3);
            ret_val = v
        },
        _ => assert!(false)
    }
    ret_val

}