use rocket_contrib::databases::mysql;
use rocket_contrib::json::JsonValue;
use rocket::http::Status;

#[database("mysqldb")]
pub struct MyRocketSQLConn(mysql::Conn);

#[derive(Deserialize, FromForm, Serialize)]
pub struct Account {
    pub id: u32,
    pub apikey: String,
    pub currency_id: u32,
    pub rarity: u8,
    pub title: String
}

// Account joined with category and currency. This is an intermediate representation.
#[derive(Deserialize, FromForm, Serialize)]
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
#[derive(Clone, Deserialize, Serialize)]
pub struct AccountJoined {
    pub id: u32,
    pub apikey: String,
    pub currency: CurrencyShort1,
    pub rarity: u8,
    pub title: String,
    pub categories: Vec<Acctcat2>
}

// POST needs this.
#[derive(FromForm)]
pub struct AccountShort {
    pub apikey: String,
    pub currency_id: u32,
    pub rarity: u8,
    pub title: String
}

#[derive(Deserialize, FromForm, Serialize)]
pub struct Acctcat {
    pub id: u32,
    pub apikey: String,
    pub account_id: u32,
    pub category_id: u32
}

#[derive(Deserialize, FromForm, Serialize)]
pub struct AcctcatShort {
    pub apikey: String,
    pub account_id: u32,
    pub category_id: u32
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Acctcat2 {
    pub category_symbol: String
}

#[derive(Deserialize)]
pub struct ApiError {
    pub error: String
}

#[derive(Deserialize)]
pub struct Apikey { pub apikey: String }

#[derive(Debug)]
pub struct ApiResponse {
    pub json: JsonValue,
    pub status: Status
}

#[derive(Deserialize, FromForm, Serialize)]
pub struct Category {
    pub id: u32,
    pub apikey: String,
    pub symbol: String,
    pub title: String
}

#[derive(FromForm)]
pub struct CategoryShort {
    pub apikey: String,
    pub symbol: String,
    pub title: String
}

#[derive(Deserialize, FromForm, Serialize)]
pub struct Currency {
    pub id: u32,
    pub apikey: String,
    pub rarity: u8,
    pub symbol: String,
    pub title: String
}

#[derive(FromForm)]
pub struct CurrencyShort {
    pub apikey: String,
    pub rarity: u8,
    pub symbol: String,
    pub title: String
}

// Need this for the AccountJoined record
#[derive(Clone, Deserialize, Serialize)]
pub struct CurrencyShort1 {
    pub symbol: String,
    pub title: String
}

#[derive(Deserialize)]
pub struct DeleteMessage {
    pub info: String
}

#[derive(Deserialize)]
pub struct DeleteSuccess {
    pub data: DeleteMessage
}

#[derive(Deserialize, FromForm, Serialize)]
pub struct Distribution {
    pub id: u32,
    pub account_id: u32,
    pub amount: i64,
    pub amount_exp: i8,
    pub apikey: String,
    pub transaction_id: u32
}

// This struct supports the ability to easily produce a list of distributions for a particular account.
#[derive(Deserialize, Serialize)]
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

#[derive(FromForm)]
pub struct DistributionShort {
    pub account_id: u32,
    pub amount: i64,
    pub amount_exp: i8,
    pub apikey: String,
    pub transaction_id: u32
}

#[derive(Deserialize)]
pub struct InsertMessage {
    pub last_insert_id: u32
}

#[derive(Deserialize)]
pub struct InsertSuccess {
    pub data: InsertMessage
}

// A linter will return a collection of id.
#[derive(Deserialize, Serialize)]
pub struct Linter {
    pub id: u32,
    pub symbol: String,
    pub title: String
}

#[derive(Serialize)]
pub struct Ping {
    pub ping: String
}

#[derive(Deserialize, FromForm, Serialize)]
pub struct Transaction {
    pub id: u32,
    pub apikey: String,
    pub notes: String,
    pub time: String
}

#[derive(FromForm)]
pub struct TransactionShort {
    pub apikey: String,
    pub notes: String,
    pub time: String
}

#[derive(Deserialize)]
pub struct UpdateMessage {
    pub info: String
}

#[derive(Deserialize)]
pub struct UpdateSuccess {
    pub data: UpdateMessage
}
