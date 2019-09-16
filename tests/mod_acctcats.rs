use bookwerx_core_rust::routes as R;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

// Examine acctcats
pub fn acctcats(client: &Client, apikey: &String, accounts: &Vec<R::AccountJoined>, categories: &Vec<R::Category>) -> Vec<R::Acctcat> {

    // 1. GET /acctcats. sb 200, empty array
    let mut response = client.get(format!("/acctcats/for_category?apikey={}&category_id={}", &apikey, (categories.get(0).unwrap()).id)).dispatch();
    let v: Vec<R::Acctcat> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 2. Try to post a new acctcat.

    // 2.1. But first post using a non-existent apikey. 200 and db error.
    response = client.post("/acctcats")
        .body(format!("apikey=notarealkey&account_id={}&category_id={}",(accounts.get(0).unwrap()).id,(categories.get(0).unwrap()).id))
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);

    // 2.2 Successful post. 200  and InsertSuccess
    response = client.post("/acctcats")
        .body(
            format!("apikey={}&account_id={}&category_id={}"
                , apikey
                , (accounts.get(0).unwrap()).id
                , (categories.get(0).unwrap()).id
            ))
        .header(ContentType::Form)
        .dispatch();
    let li: R::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(li.data.last_insert_id > 0);

    // 2.3 Successful put. 200  and UpdateSuccess
    response = client.put("/acctcats")
        .body(
            format!("apikey={}&id={}&account_id={}&category_id={}"
                , apikey, li.data.last_insert_id
                , (accounts.get(0).unwrap()).id
                ,(categories.get(0).unwrap()).id

            ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::UpdateSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() > 0);

    // 3. Now verify that there's a single acctcat.
    response = client.get(format!("/acctcats/for_category?apikey={}&category_id={}", &apikey, (categories.get(0).unwrap()).id))
        .dispatch();
    let v: Vec<R::Acctcat> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 1);

    // 4. Try to post a acctcat with a duplicated account_id, category_id. 200 db error.
    response = client.post("/acctcats")
        .body(
            format!("apikey={}&account_id={}&category_id={}"
                    , apikey
                    , (accounts.get(0).unwrap()).id
                    ,(categories.get(0).unwrap()).id

            ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);

    // 5. Now submit a single GET of the prior POST. 200 and acctcat.
    response = client.get(format!("/acctcat/{}/?apikey={}", li.data.last_insert_id, apikey))
        .dispatch();
    // The mere fact that this successfully parses a acctcat _is_ the test
    let c: R::Acctcat = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);

    // 6. Make a 2nd Successful post. 200.
    response = client.post("/acctcats")
        .body(
            format!("apikey={}&account_id={}&category_id={}"
                    , apikey
                    , (accounts.get(1).unwrap()).id
                    , (categories.get(0).unwrap()).id
            ))
        .header(ContentType::Form)
        .dispatch();
    let li: R::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(li.data.last_insert_id > 0);

    // 6.1 Verify that there are now two acctcats
    response = client.get(format!("/acctcats/for_category?apikey={}&category_id={}", &apikey, (categories.get(0).unwrap()).id))
        .dispatch();
    let v: Vec<R::Acctcat> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 2);

    v

}