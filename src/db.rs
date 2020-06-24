use crate::dfp::DFP;
use rocket::http::Status;
use rocket::request::FromForm;
use rocket::response::Responder;
use rocket_contrib::database;
use rocket_contrib::databases::mysql;
use rocket_contrib::json::JsonValue;
use serde::{Deserialize, Serialize};

#[database("mysqldb")]
pub struct MyRocketSQLConn(mysql::Conn);

/*
We have a blizzard of structs for a variety of reasons.  Unfortunately, it's rather tedious to
manage them.  Please allow me to enumerate the problems and solutions.

Some of the structs are substantially similar to the underlying db row that they model. Naming them is reasonably easy.  However, many structs have "decorations".  That is, account names, currency symbols, and other similar related info.  Naming these is not easy, especially since we tend to many similar variations.

In addition, these structs also have a variety of derives.  These derives have accumulated over the eons and there's no good way to determine if any of them are unused. Granted, an unused derive does not have significant consequence.  But it does offend my finely honed sense of aesthetics and that's reason enough to fret about this.

What we have here now is my one-time pass over all of this to cleanup whatever chaos lurks within.  It's the best I can reasonably do with this foul situation.

many derives.
 */

// 1. The basic data formats.
#[derive(Clone)]
#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(FromForm)]    // PUT /accounts.
#[derive(Serialize)]   // We send these as a json result.
pub struct Account {
    pub id: u32,
    pub apikey: String,
    pub currency_id: u32,
    pub rarity: u8,
    pub title: String
}

// Account joined with category and currency. This is an intermediate representation.
pub struct AccountDenormalized {
    pub id: u32,
    pub apikey: String,
    pub rarity: u8,
    pub title: String,
    pub cur_symbol: String,
    pub cur_title: String,
    pub ac_category_id: u32,
    pub cat_symbol: String,
    pub cat_title: String
}

// Final form to send as a response.
#[derive(Clone)]
#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub struct AccountJoined {
    pub id: u32,
    pub apikey: String,
    pub currency: CurrencyShort1,
    pub rarity: u8,
    pub title: String,
    pub categories: Vec<Acctcat2>
}

#[derive(FromForm)]    // POST /accounts.
pub struct AccountShort {
    pub apikey: String,
    pub currency_id: u32,
    pub rarity: u8,
    pub title: String
}

pub struct AccountShort1 {
    pub currency_id: u32,
    pub title: String
}

#[derive(Clone)]
#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(FromForm)]    // PUT /acctcats.
#[derive(Serialize)]   // We send these as a json result.
pub struct Acctcat {
    pub id: u32,
    pub apikey: String,
    pub account_id: u32,
    pub category_id: u32
}

#[derive(FromForm)]    // POST /acctcats.
pub struct AcctcatShort {
    pub apikey: String,
    pub account_id: u32,
    pub category_id: u32
}

#[derive(Clone)]
#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub struct Acctcat2 {
    pub category_symbol: String
}

#[derive(Deserialize)] // A test parses a response into this struct.
pub struct Apikey { pub apikey: String }

#[derive(Clone)]
#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(FromForm)]    // PUT /categories.
#[derive(Serialize)]   // We send these as a json result.
pub struct Category {
    pub id: u32,
    pub apikey: String,
    pub symbol: String,
    pub title: String
}

#[derive(FromForm)]    // POST /categories.
pub struct CategoryShort {
    pub apikey: String,
    pub symbol: String,
    pub title: String
}

#[derive(Clone)]
#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(FromForm)]    // PUT /currencies.
#[derive(Serialize)]   // We send these as a json result.
pub struct Currency {
    pub id: u32,
    pub apikey: String,
    pub rarity: u8,
    pub symbol: String,
    pub title: String
}

#[derive(FromForm)]    // POST /currencies.
pub struct CurrencyShort {
    pub apikey: String,
    pub rarity: u8,
    pub symbol: String,
    pub title: String
}

#[derive(Clone)]
#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub struct CurrencyShort1 {
    pub symbol: String,
    pub title: String
}

pub struct CurrencyShort2 {
    pub id: u32,
    pub symbol: String,
    pub title: String
}

#[derive(Clone)]
#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(FromForm)]    // PUT /distributions.
#[derive(Serialize)]   // We send these as a json result.
pub struct Distribution {
    pub id: u32,
    pub account_id: u32,
    pub amount: i64,
    pub amount_exp: i8,
    pub apikey: String,
    pub transaction_id: u32
}

// This struct supports the ability to easily produce a list of distributions for a particular account.
#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub struct DistributionJoined {
    pub id: u32,
    pub tid: u32,
    pub aid: u32,
    pub amount: i64,
    pub amount_exp: i8,
    pub apikey: String,
    pub account_title: String,
    pub tx_notes: String,
    pub tx_time: String
}

#[derive(FromForm)]    // POST /distributions.
pub struct DistributionShort {
    pub account_id: u32,
    pub amount: i64,
    pub amount_exp: i8,
    pub apikey: String,
    pub transaction_id: u32
}

// A linter will return a collection of id.
#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub struct LinterShort {
    pub id: u32,
    pub title: String
}

#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub struct LinterLong {
    pub id: u32,
    pub symbol: String,
    pub title: String
}

//#[derive(Serialize)]
//pub struct Ping {
    //pub ping: String
//}

#[derive(Clone)]
#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(FromForm)]    // PUT /transactions.
#[derive(Serialize)]   // We send these as a json result.
pub struct Transaction {
    pub id: u32,
    pub apikey: String,
    pub notes: String,
    pub time: String
}

#[derive(FromForm)]    // POST /transactions.
pub struct TransactionShort {
    pub apikey: String,
    pub notes: String,
    pub time: String
}

/*
2. account_dist_sum and category_dist_sums produce a variety of outputs related to the sum of all distributions for accounts, as well as optional decorations.
 */
#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub struct AccountCurrency {
    pub account_id: u32,
    pub title: String,
    pub currency: CurrencySymbol
}

pub struct AccountCurrencyDecorations {
    pub account_id: u32,
    pub title: String,
    pub currency_id: u32,
    pub symbol: String,
}

#[derive(Clone, Copy)]
#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub struct AcctSum {
    pub account_id: u32,
    pub sum: DFP,
}

pub struct BalanceResult {
    pub account_id: u32,
    pub amount: i64,
    pub amount_exp: i8,
}

#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub struct BalanceResultDecorated {
    pub account: AccountCurrency,
    pub sum: DFP
}

#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub struct CurrencySymbol {
    pub currency_id: u32,
    pub symbol: String
}

#[derive(Deserialize)] // A test parses a response into this struct.
pub struct DFPResult {
    pub sum: DFP
}

#[derive(Deserialize)] // A test parses a response into this struct.
pub struct Sums {
    pub sums: Vec<AcctSum>
}

#[derive(Deserialize)] // A test parses a response into this struct.
pub struct SumsDecorated {
    pub sums: Vec<BalanceResultDecorated>
}

// 3. The response types
#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub enum APIResponse {
    Info(String),
    LastInsertId(u64),
    Error(String)
}

#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub enum GetAccountResponse {
    One(Account),
    Many(Vec<AccountJoined>),
    Error(String)
}

#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub enum GetAcctcatResponse {
    One(Acctcat),
    Many(Vec<Acctcat>),
    Error(String)
}

#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub enum GetCategoryResponse {
    One(Category),
    Many(Vec<Category>),
    Error(String)
}

#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub enum GetCurrencyResponse {
    One(Currency),
    Many(Vec<Currency>),
    Error(String)
}

#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub enum GetDistributionResponse {
    One(Distribution),
    Many(Vec<Distribution>),
    ManyJoined(Vec<DistributionJoined>),
    Error(String)
}

#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub enum GetTransactionResponse {
    One(Transaction),
    Many(Vec<Transaction>),
    Error(String)
}

#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Serialize)]   // We send these as a json result.
pub enum PostApikeysResponse {
    Apikey(String),
    Error(String)
}

#[derive(Deserialize)] // A test parses a response into this struct.
#[derive(Responder)]
#[derive(Serialize)]   // We send these as a json result.
pub struct ApiError {
    pub error: String
}

pub struct ApiResponseOld {
    pub json: JsonValue,
    pub status: Status
}
