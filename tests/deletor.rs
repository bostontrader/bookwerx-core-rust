use bookwerx_core_rust::db as D;

use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

// Now try to delete things.  Ensure that referential integrity constraints prevent inappropriate deletions.
pub fn deletor(client: &Client, apikey: &String, accounts: &Vec<D::AccountJoined>, acctcats: &Vec<D::Acctcat>, categories: &Vec<D::Category>, currencies: &Vec<D::Currency>, distributions: &Vec<D::Distribution>, transactions: &Vec<D::Transaction>)  {

    // 1. First try to delete things that cannot be deleted because of referential integrity constraints.  Watch and laugh as these efforts fail with status 200 and ApiError.

    // 1.1 Try to DELETE account 0.
    let mut response = client.delete(format!("/account/{}/?apikey={}", (accounts.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false)
    }

    // 1.2 Try to DELETE category 0.
    response = client.delete(format!("/category/{}/?apikey={}", (categories.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false)
    }

    // 1.3 Try to DELETE currency 0.
    response = client.delete(format!("/currency/{}/?apikey={}", (currencies.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false)
    }

    // 1.4 Try to DELETE transaction 0.
    response = client.delete(format!("/transaction/{}/?apikey={}", (transactions.get(0).unwrap()).id, apikey ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false)
    }

    // 2. Now start deleting everything in a proper order such that the referential integrity constraints are satisfied.

    // 2.1 DELETE all distributions.
    for distribution in distributions {
        response = client.delete(format!("/distribution/{}/?apikey={}", distribution.id, apikey ))
            .header(ContentType::Form)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
            D::APIResponse::Info(_) => assert!(true),
            _ => assert!(false)
        }
    }

    // 2.2 DELETE all transactions.
    for transaction in transactions {
        response = client.delete(format!("/transaction/{}/?apikey={}", transaction.id, apikey ))
            .header(ContentType::Form)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
            D::APIResponse::Info(_) => assert!(true),
            _ => assert!(false)
        }
    }

    // 2.3 DELETE all acctcats.
    for acctcat in acctcats {
        response = client.delete(format!("/acctcat/{}/?apikey={}", acctcat.id, apikey ))
            .header(ContentType::Form)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
            D::APIResponse::Info(_) => assert!(true),
            _ => assert!(false)
        }
    }

    // 2.4 DELETE all accounts.
    for account in accounts {
        response = client.delete(format!("/account/{}/?apikey={}", account.id, apikey ))
            .header(ContentType::Form)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
            D::APIResponse::Info(_) => assert!(true),
            _ => assert!(false)
        }
    }

    // 2.5 DELETE all currencies.
    for currency in currencies {
        response = client.delete(format!("/currency/{}/?apikey={}", currency.id, apikey ))
            .header(ContentType::Form)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
            D::APIResponse::Info(_) => assert!(true),
            _ => assert!(false)
        }
    }

    // 2.6 DELETE all categories.
    for category in categories {
        response = client.delete(format!("/category/{}/?apikey={}", category.id, apikey ))
            .header(ContentType::Form)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
            D::APIResponse::Info(_) => assert!(true),
            _ => assert!(false)
        }
    }

    // 3. Now verify that all these collections are empty

    // 3.1 GET /accounts.
    response = client.get(format!("/accounts?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAccountResponse::Many(v) => assert_eq!(v.len(), 0),
        _ => assert!(false)
    }

    // 3.2 GET /acctcats.
    response = client.get(format!("/acctcats/for_category?apikey={}&category_id={}", &apikey, (categories.get(0).unwrap()).id))
        .dispatch();
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAcctcatResponse::Many(v) => assert_eq!(v.len(), 0),
        _ => assert!(false)
    }

    // 3.3 GET /categories.
    response = client.get(format!("/categories?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetCategoryResponse::Many(v) => assert_eq!(v.len(), 0),
        _ => assert!(false)
    }

    // 3.4 GET /currencies.
    response = client.get(format!("/currencies?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetCurrencyResponse::Many(v) => assert_eq!(v.len(), 0),
        _ => assert!(false)
    }

    // 3.5 GET /distributions.
    response = client.get(format!("/distributions?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetDistributionResponse::Many(v) => assert_eq!(v.len(), 0),
        _ => assert!(false)
    }

    // 3.6 GET /transactions.
    response = client.get(format!("/transactions?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetAccountResponse::Many(v) => assert_eq!(v.len(), 0),
        _ => assert!(false)
    }
    
}
