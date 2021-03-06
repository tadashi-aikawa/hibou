use serde::{Deserialize, Serialize};

use crate::external::gtfs::{Lang, MailAddress, TelephoneNumber, Timezone, Url};
use crate::external::gtfscsv::GTFSFile;
use crate::external::gtfsdb::Table;

/// 事業者ID  (ex: 8000020130001, 8000020130001_1)
pub type AgencyId = String;

/// 事業者情報
/// https://developers.google.com/transit/gtfs/reference?hl=ja#agencytxt
/// https://www.gtfs.jp/developpers-guide/format-reference.html#agency
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct Agency {
    /// 事業者ID
    pub agency_id: AgencyId,
    /// 事業者名称 (ex: 都営バス)
    pub agency_name: String,
    /// 事業者URL (ex: http://www.kotsu.metro.tokyo.jp/bus/)
    pub agency_url: Url,
    /// タイムゾーン (ex: Asia/Tokyo)
    pub agency_timezone: Timezone,
    /// 言語
    pub agency_lang: Option<Lang>,
    /// 電話番号
    pub agency_phone: Option<TelephoneNumber>,
    /// オンライン購入URL
    pub agency_fare_url: Option<Url>,
    /// 事業者Eメール
    pub agency_email: Option<MailAddress>,
}

impl GTFSFile for Agency {
    fn file_name() -> &'static str {
        "agency.txt"
    }
}

impl Table for Agency {
    fn table_name() -> &'static str {
        "agency"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "agency_id",
            "agency_name",
            "agency_url",
            "agency_timezone",
            "agency_lang",
            "agency_phone",
            "agency_fare_url",
            "agency_email",
        ]
    }

    fn create_sql() -> &'static str {
        "
        agency_id text primary key,
        agency_name text not null,
        agency_url text not null,
        agency_timezone text not null,
        agency_lang text,
        agency_phone text,
        agency_fare_url text,
        agency_email text
        "
    }
}
