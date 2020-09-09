use bookwerx_core_rust::db as D;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::Client;

/* Please see the comments for transactions for a discussion of constraints in this test.

  These tests include testing the correct number of distributions for_account and for_tx
*/
pub fn distributions(
    client: &Client,
    apikey: &String,
    accounts: &Vec<D::AccountJoined>,
    transactions: &Vec<D::Transaction>,
) -> Vec<D::Distribution> {
    let account_id0: u32 = (*accounts.get(0).unwrap()).id;
    let account_id1: u32 = (*accounts.get(1).unwrap()).id;
    let transaction_id0: u32 = (*transactions.get(0).unwrap()).id;
    let transaction_id1: u32 = (*transactions.get(1).unwrap()).id;
    let transaction_id2: u32 = (*transactions.get(2).unwrap()).id;

    // 1. GET /distributions/for_tx.
    let mut response = client
        .get(format!(
            "/distributions/for_tx?apikey={}&transaction_id={}",
            &apikey, &transaction_id1
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let s: D::GetDistributionJoinedResponse =
        serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap();
    match s {
        D::GetDistributionJoinedResponse::Many(v) => assert_eq!(v.len(), 0),
        _ => assert!(false),
    }

    // 1.1 GET /distributions/for_account.
    let mut response = client
        .get(format!(
            "/distributions/for_account?apikey={}&account_id={}",
            &apikey, &account_id1
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetDistributionJoinedResponse::Many(v) => assert_eq!(v.len(), 0),
        _ => assert!(false),
    }

    // 2. Try to post a new distribution

    // 2.1. But first post using a non-existent apikey.
    response = client
        .post("/distributions")
        .body(format!(
            "apikey=notarealkey&transaction_id={}&account_id={}&amount=3&amount_exp=0",
            transaction_id1, account_id1
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Error(_) => assert!(true),
        _ => assert!(false),
    }

    // 2.2 Successful post.
    response = client
        .post("/distributions")
        .body(format!(
            "&apikey={}&transaction_id={}&account_id={}&amount=3&amount_exp=0",
            apikey, transaction_id0, account_id0
        ))
        .header(ContentType::Form)
        .dispatch();
    let mut lid: u64 = 0;
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid1) => {
            lid = lid1;
            assert!(lid > 0)
        }
        _ => assert!(false),
    }

    // 2.3 Successful put.
    response = client
        .put("/distributions")
        .body(format!(
            "&apikey={}&id={}&account_id={}&transaction_id={}&amount=3&amount_exp=0",
            apikey, lid, account_id0, transaction_id0
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::Info(info) => assert_eq!(info, "(Rows matched: 1  Changed: 0  Warnings: 0"),
        _ => assert!(false),
    }

    // 3. Now verify that there's a single distribution for_tx
    response = client
        .get(format!(
            "/distributions/for_tx?apikey={}&transaction_id={}",
            &apikey, &transaction_id0
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetDistributionJoinedResponse::Many(v) => assert_eq!(v.len(), 1),
        _ => assert!(false),
    }

    // 3.1 Now verify that there's a single distribution for_account
    response = client
        .get(format!(
            "/distributions/for_account?apikey={}&account_id={}",
            &apikey, &account_id0
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetDistributionJoinedResponse::Many(v) => assert_eq!(v.len(), 1),
        _ => assert!(false),
    }

    // 4. Post bad record...

    // 5. Now submit a single GET of the prior POST.
    // Don't do this.  Nobody cares.

    // 6. Make the 2nd Successful post.
    response = client
        .post("/distributions")
        .body(format!(
            "&apikey={}&transaction_id={}&account_id={}&amount=-3&amount_exp=0",
            apikey, transaction_id0, account_id1
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false),
    }

    // 6.1 Now verify the correct count of transactions for_tx and for_account
    response = client
        .get(format!(
            "/distributions/for_tx?apikey={}&transaction_id={}",
            &apikey, &transaction_id0
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetDistributionJoinedResponse::Many(v) => assert_eq!(v.len(), 2),
        _ => assert!(false),
    }

    response = client
        .get(format!(
            "/distributions/for_account?apikey={}&account_id={}",
            &apikey, &account_id0
        ))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetDistributionJoinedResponse::Many(v) => assert_eq!(v.len(), 1),
        _ => assert!(false),
    }

    // 7. Post four more distributions to two different transactions so that we can test account_dist_sum and category_dist_sums.

    // 7.1 tx1

    // 7.1.1 Successful post.
    response = client
        .post("/distributions")
        .body(format!(
            "&apikey={}&transaction_id={}&account_id={}&amount=4&amount_exp=0",
            apikey, transaction_id1, account_id0
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false),
    }

    // 7.1.2 Successful post.
    response = client
        .post("/distributions")
        .body(format!(
            "&apikey={}&transaction_id={}&account_id={}&amount=-4&amount_exp=0",
            apikey, transaction_id1, account_id1
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false),
    }

    // 7.2 tx2

    // 7.2.1 Successful post.
    response = client
        .post("/distributions")
        .body(format!(
            "&apikey={}&transaction_id={}&account_id={}&amount=5&amount_exp=0",
            apikey, transaction_id2, account_id0
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false),
    }

    // 7.2.2 Successful post.
    response = client
        .post("/distributions")
        .body(format!(
            "&apikey={}&transaction_id={}&account_id={}&amount=-5&amount_exp=0",
            apikey, transaction_id2, account_id1
        ))
        .header(ContentType::Form)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::APIResponse::LastInsertId(lid) => assert!(lid > 0),
        _ => assert!(false),
    }

    // 8. Retrieve _all_ distributions because we'll need to delete 'em later.
    response = client
        .get(format!("/distributions?apikey={}", &apikey))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    let mut ret_val = Vec::new();
    match serde_json::from_str(&(response.body_string().unwrap())[..]).unwrap() {
        D::GetDistributionResponse::Many(v) => {
            assert_eq!(v.len(), 6);
            ret_val = v
        }
        _ => assert!(false),
    }
    ret_val
}
