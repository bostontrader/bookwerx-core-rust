use bookwerx_core_rust::db as D;
use rocket::local::Client;
use rocket::http::ContentType;
use rocket::http::Status;

/* Please see the comments for transactions for a discussion of contraints in this test.

  These tests include testing the correct number of distributions for_account and for_tx
*/
pub fn distributions(client: &Client, apikey: &String, accounts: &Vec<D::AccountJoined>, transactions: &Vec<D::Transaction>) -> Vec<D::Distribution> {

    let account_id1: u32 = (*accounts.get(0).unwrap()).id;
    let account_id2: u32 = (*accounts.get(1).unwrap()).id;
    let transaction_id1: u32 = (*transactions.get(0).unwrap()).id;
    let transaction_id2: u32 = (*transactions.get(1).unwrap()).id;
    let transaction_id3: u32 = (*transactions.get(2).unwrap()).id;

    // 1. GET /distributions/for_tx. sb 200, empty array
    let mut response = client.get(format!("/distributions/for_tx?apikey={}&transaction_id={}", &apikey, &transaction_id1)).dispatch();
    let v: Vec<D::DistributionJoined> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 1.1 GET /distributions/for_account. sb 200, empty array
    let mut response = client.get(format!("/distributions/for_account?apikey={}&account_id={}", &apikey, &account_id1)).dispatch();
    let v: Vec<D::DistributionJoined> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 0);

    // 2. Try to post a new distribution

    // 2.1. But first post using a non-existent apikey. 200 and db error.
    response = client.post("/distributions")
        .body(format!("apikey=notarealkey&transaction_id={}&account_id={}&amount=3&amount_exp=0", transaction_id1, account_id1))
        .header(ContentType::Form)
        .dispatch();
    let r: D::ApiError = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.error.len() > 0);

    // 2.2 Successful post. 200 and InsertSuccess.
    response = client.post("/distributions")
        .body(format!("&apikey={}&transaction_id={}&account_id={}&amount=3&amount_exp=0", apikey, transaction_id1, account_id1))
        .header(ContentType::Form)
        .dispatch();
    let li: D::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(li.data.last_insert_id > 0);

    // 2.3 Successful put. 200 and UpdateSuccess
    response = client.put("/distributions")
        .body(format!("&apikey={}&id={}&account_id={}&transaction_id={}&amount=3&amount_exp=0", apikey, li.data.last_insert_id, account_id1, transaction_id1))
        .header(ContentType::Form)
        .dispatch();
    let r: D::UpdateSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(r.data.info.len() > 0);

    // 3. Now verify that there's a single distribution for_tx
    response = client.get(format!("/distributions/for_tx?apikey={}&transaction_id={}", &apikey, &transaction_id1)).dispatch();
    let v: Vec<D::DistributionJoined> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 1);

    // 3.1 Now verify that there's a single distribution for_account
    response = client.get(format!("/distributions/for_account?apikey={}&account_id={}", &apikey, &account_id1)).dispatch();
    let v: Vec<D::DistributionJoined> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 1);


    // 4. Post bad record...

    // 5. Now submit a single GET of the prior POST. 200 and distribution.
    // Don't do this.  Nobody cares.

    // 6. Make the 2nd Successful post. 200 and InsertSuccess
    response = client.post("/distributions")
        .body(format!("&apikey={}&transaction_id={}&account_id={}&amount=-3&amount_exp=0", apikey, transaction_id1, account_id2))
        .header(ContentType::Form)
        .dispatch();
    let li: D::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(li.data.last_insert_id > 0);

    // 6.1 Now verify the correct count of transactions for_tx and for_account
    response = client.get(format!("/distributions/for_tx?apikey={}&transaction_id={}", &apikey, &transaction_id1)).dispatch();
    let v: Vec<D::DistributionJoined> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 2);

    response = client.get(format!("/distributions/for_account?apikey={}&account_id={}", &apikey, &account_id1)).dispatch();
    let v: Vec<D::DistributionJoined> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 1);

    // 7. Post two more distributions to two different transactions so that we can test account_dist_sum and category_dist_sums.

    // 7.1 Successful post. 200 and InsertSuccess.
    response = client.post("/distributions")
        .body(format!("&apikey={}&transaction_id={}&account_id={}&amount=4&amount_exp=0", apikey, transaction_id2, account_id1))
        .header(ContentType::Form)
        .dispatch();
    let li: D::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(li.data.last_insert_id > 0);

    // 7.2 Successful post. 200 and InsertSuccess.
    response = client.post("/distributions")
        .body(format!("&apikey={}&transaction_id={}&account_id={}&amount=5&amount_exp=0", apikey, transaction_id3, account_id1))
        .header(ContentType::Form)
        .dispatch();
    let li: D::InsertSuccess = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert!(li.data.last_insert_id > 0);


    // 8. Retrieve _all_ distributions because we'll need to delete 'em later
    response = client.get(format!("/distributions?apikey={}", &apikey)).dispatch();
    let v: Vec<D::Distribution> = serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(v.len(), 4);
    v
}
