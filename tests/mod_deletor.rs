use bookwerx_core_rust::routes as R;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

// Now try to delete things.  Ensure that referential integrity constraints prevent inappropriate deletions.
pub fn deletor(client: &Client, apikey: &String, accounts: &Vec<R::Account>, currencies: &Vec<R::Currency>, distributions: &Vec<R::Distribution>, transactions: &Vec<R::Transaction>)  {

    // 1. First try to delete things that cannot be deleted because of referential integrity constraints.  Watch and laugh as these efforts fail with status 200 and ApiError.
    // 1.1 Try to DELETE currency 0.
    let mut response = client.delete(format!("/currency/{}/?apikey={}", (currencies.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);

    // 1.2 Try to DELETE account 0.
    response = client.delete(format!("/account/{}/?apikey={}", (accounts.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);

    // 1.3 Try to DELETE transaction 0.
    response = client.delete(format!("/transaction/{}/?apikey={}", (transactions.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);

    // 2. Now start deleting everything in a proper order such that the referential integrity constraints are satisfied.
    // 2.1 DELETE distribution 1.  200 and DeleteSuccess.
    response = client.delete(format!("/distribution/{}/?apikey={}", (distributions.get(1).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);

    // 2.2 DELETE distribution 0.  200 and DeleteSuccess.
    response = client.delete(format!("/distribution/{}/?apikey={}", (distributions.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);

    // 2.3 DELETE transaction 1.  200 and DeleteSuccess.
    response = client.delete(format!("/transaction/{}/?apikey={}", (transactions.get(1).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);

    // 2.4 DELETE transaction 0.  200 and DeleteSuccess.
    response = client.delete(format!("/transaction/{}/?apikey={}", (transactions.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);

    // 2.5 DELETE account 1.  200 and DeleteSuccess.
    response = client.delete(format!("/account/{}/?apikey={}", (accounts.get(1).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);

    // 2.6 DELETE account 0.  200 and DeleteSuccess.
    response = client.delete(format!("/account/{}/?apikey={}", (accounts.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);
    
    // 2.7 DELETE currency 1.  200 and DeleteSuccess.
    response = client.delete(format!("/currency/{}/?apikey={}", (currencies.get(1).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);

    // 2.8 DELETE currency 0.  200 and DeleteSuccess.
    response = client.delete(format!("/currency/{}/?apikey={}", (currencies.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);

    // 3. Now verify that all these collections are empty

    // 3.1 GET /currencies. sb 200, empty array
    response = client.get(format!("/accounts?apikey={}", &apikey))
        .dispatch();
    let v: Vec<R::Account> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 3.2 GET /currencies. sb 200, empty array
    response = client.get(format!("/currencies?apikey={}", &apikey))
        .dispatch();
    let v: Vec<R::Currency> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 3.3 GET /currencies. sb 200, empty array
    response = client.get(format!("/distributions?apikey={}", &apikey))
        .dispatch();
    let v: Vec<R::Distribution> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 3.4 GET /currencies. sb 200, empty array
    response = client.get(format!("/transactions?apikey={}", &apikey))
        .dispatch();
    let v: Vec<R::Transaction> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);
    
}
