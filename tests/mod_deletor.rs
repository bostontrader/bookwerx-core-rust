use bookwerx_core_rust::routes as R;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

// Now try to delete things.  Ensure that referential integrity constraints prevent inappropriate deletions.
pub fn deletor(client: &Client, apikey: &String, accounts: &Vec<R::AccountJoined>, acctcats: &Vec<R::Acctcat>, categories: &Vec<R::Category>, currencies: &Vec<R::Currency>, distributions: &Vec<R::Distribution>, transactions: &Vec<R::Transaction>)  {

    // 1. First try to delete things that cannot be deleted because of referential integrity constraints.  Watch and laugh as these efforts fail with status 200 and ApiError.

    // 1.1 Try to DELETE account 0.
    let mut response = client.delete(format!("/account/{}/?apikey={}", (accounts.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);

    // 1.2 Try to DELETE category 0.
    response = client.delete(format!("/category/{}/?apikey={}", (categories.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);

    // 1.3 Try to DELETE currency 0.
    response = client.delete(format!("/currency/{}/?apikey={}", (currencies.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);

    // 1.4 Try to DELETE transaction 0.
    response = client.delete(format!("/transaction/{}/?apikey={}", (transactions.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);


    
    // 2. Now start deleting everything in a proper order such that the referential integrity constraints are satisfied.
    // 2.1 DELETE distributions.  200 and DeleteSuccess.

    // 2.1.1
    response = client.delete(format!("/distribution/{}/?apikey={}", (distributions.get(1).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);

    // 2.1.2
    response = client.delete(format!("/distribution/{}/?apikey={}", (distributions.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);


    // 2.2 DELETE transactions.  200 and DeleteSuccess.

    // 2.2.1
    response = client.delete(format!("/transaction/{}/?apikey={}", (transactions.get(1).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);

    // 2.2.2
    response = client.delete(format!("/transaction/{}/?apikey={}", (transactions.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);


    // 2.3 DELETE acctcats.  200 and DeleteSuccess.

    // 2.3.1
    response = client.delete(format!("/acctcat/{}/?apikey={}", (acctcats.get(1).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);

    // 2.3.2
    response = client.delete(format!("/acctcat/{}/?apikey={}", (acctcats.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);
    
    
    // 2.4 DELETE accounts.  200 and DeleteSuccess.

    // 2.4.1
    response = client.delete(format!("/account/{}/?apikey={}", (accounts.get(1).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);

    // 2.4.2
    response = client.delete(format!("/account/{}/?apikey={}", (accounts.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);
    

    // 2.5 DELETE currencies.  200 and DeleteSuccess.

    // 2.5.1
    response = client.delete(format!("/currency/{}/?apikey={}", (currencies.get(1).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);

    // 2.5.2
    response = client.delete(format!("/currency/{}/?apikey={}", (currencies.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);


    // 2.6 DELETE categories.  200 and DeleteSuccess.

    // 2.6.1
    response = client.delete(format!("/category/{}/?apikey={}", (categories.get(1).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);

    // 2.6.2
    response = client.delete(format!("/category/{}/?apikey={}", (categories.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    let r: R::DeleteSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() == 0);
    
    
    // 3. Now verify that all these collections are empty

    // 3.1 GET /accounts. sb 200, empty array
    response = client.get(format!("/accounts?apikey={}", &apikey))
        .dispatch();
    let v: Vec<R::Account> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 3.2 GET /acctcats. sb 200, empty array
    response = client.get(format!("/acctcats/for_category?apikey={}&category_id={}", &apikey, (categories.get(0).unwrap()).id))
        .dispatch();
    let v: Vec<R::Acctcat> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 3.3 GET /categories. sb 200, empty array
    response = client.get(format!("/categories?apikey={}", &apikey))
        .dispatch();
    let v: Vec<R::Category> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 3.4 GET /currencies. sb 200, empty array
    response = client.get(format!("/currencies?apikey={}", &apikey))
        .dispatch();
    let v: Vec<R::Currency> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 3.5 GET /distributions. sb 200, empty array
    response = client.get(format!("/distributions?apikey={}", &apikey))
        .dispatch();
    let v: Vec<R::Distribution> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 3.6 GET /transactions. sb 200, empty array
    response = client.get(format!("/transactions?apikey={}", &apikey))
        .dispatch();
    let v: Vec<R::Transaction> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);
    
}
