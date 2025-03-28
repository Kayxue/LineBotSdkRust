/*
 * LINE Messaging API
 *
 * This document describes LINE Messaging API.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AreaDemographic {
    #[serde(rename = "jp_01")]
    HOKKAIDO,
    #[serde(rename = "jp_02")]
    AOMORI,
    #[serde(rename = "jp_03")]
    IWATE,
    #[serde(rename = "jp_04")]
    MIYAGI,
    #[serde(rename = "jp_05")]
    AKITA,
    #[serde(rename = "jp_06")]
    YAMAGATA,
    #[serde(rename = "jp_07")]
    FUKUSHIMA,
    #[serde(rename = "jp_08")]
    IBARAKI,
    #[serde(rename = "jp_09")]
    TOCHIGI,
    #[serde(rename = "jp_10")]
    GUNMA,
    #[serde(rename = "jp_11")]
    SAITAMA,
    #[serde(rename = "jp_12")]
    CHIBA,
    #[serde(rename = "jp_13")]
    TOKYO,
    #[serde(rename = "jp_14")]
    KANAGAWA,
    #[serde(rename = "jp_15")]
    NIIGATA,
    #[serde(rename = "jp_16")]
    TOYAMA,
    #[serde(rename = "jp_17")]
    ISHIKAWA,
    #[serde(rename = "jp_18")]
    FUKUI,
    #[serde(rename = "jp_19")]
    YAMANASHI,
    #[serde(rename = "jp_20")]
    NAGANO,
    #[serde(rename = "jp_21")]
    GIFU,
    #[serde(rename = "jp_22")]
    SHIZUOKA,
    #[serde(rename = "jp_23")]
    AICHI,
    #[serde(rename = "jp_24")]
    MIE,
    #[serde(rename = "jp_25")]
    SHIGA,
    #[serde(rename = "jp_26")]
    KYOTO,
    #[serde(rename = "jp_27")]
    OSAKA,
    #[serde(rename = "jp_28")]
    HYOUGO,
    #[serde(rename = "jp_29")]
    NARA,
    #[serde(rename = "jp_30")]
    WAKAYAMA,
    #[serde(rename = "jp_31")]
    TOTTORI,
    #[serde(rename = "jp_32")]
    SHIMANE,
    #[serde(rename = "jp_33")]
    OKAYAMA,
    #[serde(rename = "jp_34")]
    HIROSHIMA,
    #[serde(rename = "jp_35")]
    YAMAGUCHI,
    #[serde(rename = "jp_36")]
    TOKUSHIMA,
    #[serde(rename = "jp_37")]
    KAGAWA,
    #[serde(rename = "jp_38")]
    EHIME,
    #[serde(rename = "jp_39")]
    KOUCHI,
    #[serde(rename = "jp_40")]
    FUKUOKA,
    #[serde(rename = "jp_41")]
    SAGA,
    #[serde(rename = "jp_42")]
    NAGASAKI,
    #[serde(rename = "jp_43")]
    KUMAMOTO,
    #[serde(rename = "jp_44")]
    OITA,
    #[serde(rename = "jp_45")]
    MIYAZAKI,
    #[serde(rename = "jp_46")]
    KAGOSHIMA,
    #[serde(rename = "jp_47")]
    OKINAWA,
    #[serde(rename = "tw_01")]
    TAIPEI_CITY,
    #[serde(rename = "tw_02")]
    NEW_TAIPEI_CITY,
    #[serde(rename = "tw_03")]
    TAOYUAN_CITY,
    #[serde(rename = "tw_04")]
    TAICHUNG_CITY,
    #[serde(rename = "tw_05")]
    TAINAN_CITY,
    #[serde(rename = "tw_06")]
    KAOHSIUNG_CITY,
    #[serde(rename = "tw_07")]
    KEELUNG_CITY,
    #[serde(rename = "tw_08")]
    HSINCHU_CITY,
    #[serde(rename = "tw_09")]
    CHIAYI_CITY,
    #[serde(rename = "tw_10")]
    HSINCHU_COUNTY,
    #[serde(rename = "tw_11")]
    MIAOLI_COUNTY,
    #[serde(rename = "tw_12")]
    CHANGHUA_COUNTY,
    #[serde(rename = "tw_13")]
    NANTOU_COUNTY,
    #[serde(rename = "tw_14")]
    YUNLIN_COUNTY,
    #[serde(rename = "tw_15")]
    CHIAYI_COUNTY,
    #[serde(rename = "tw_16")]
    PINGTUNG_COUNTY,
    #[serde(rename = "tw_17")]
    YILAN_COUNTY,
    #[serde(rename = "tw_18")]
    HUALIEN_COUNTY,
    #[serde(rename = "tw_19")]
    TAITUNG_COUNTY,
    #[serde(rename = "tw_20")]
    PENGHU_COUNTY,
    #[serde(rename = "tw_21")]
    KINMEN_COUNTY,
    #[serde(rename = "tw_22")]
    LIENCHIANG_COUNTY,
    #[serde(rename = "th_01")]
    BANGKOK,
    #[serde(rename = "th_02")]
    PATTAYA,
    #[serde(rename = "th_03")]
    NORTHERN,
    #[serde(rename = "th_04")]
    CENTRAL,
    #[serde(rename = "th_05")]
    SOUTHERN,
    #[serde(rename = "th_06")]
    EASTERN,
    #[serde(rename = "th_07")]
    NORTHEASTERN,
    #[serde(rename = "th_08")]
    WESTERN,
    #[serde(rename = "id_01")]
    BALI,
    #[serde(rename = "id_02")]
    BANDUNG,
    #[serde(rename = "id_03")]
    BANJARMASIN,
    #[serde(rename = "id_04")]
    JABODETABEK,
    #[serde(rename = "id_05")]
    MAKASSAR,
    #[serde(rename = "id_06")]
    MEDAN,
    #[serde(rename = "id_07")]
    PALEMBANG,
    #[serde(rename = "id_08")]
    SAMARINDA,
    #[serde(rename = "id_09")]
    SEMARANG,
    #[serde(rename = "id_10")]
    SURABAYA,
    #[serde(rename = "id_11")]
    YOGYAKARTA,
    #[serde(rename = "id_12")]
    LAINNYA,

}

impl std::fmt::Display for AreaDemographic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::HOKKAIDO => write!(f, "jp_01"),
            Self::AOMORI => write!(f, "jp_02"),
            Self::IWATE => write!(f, "jp_03"),
            Self::MIYAGI => write!(f, "jp_04"),
            Self::AKITA => write!(f, "jp_05"),
            Self::YAMAGATA => write!(f, "jp_06"),
            Self::FUKUSHIMA => write!(f, "jp_07"),
            Self::IBARAKI => write!(f, "jp_08"),
            Self::TOCHIGI => write!(f, "jp_09"),
            Self::GUNMA => write!(f, "jp_10"),
            Self::SAITAMA => write!(f, "jp_11"),
            Self::CHIBA => write!(f, "jp_12"),
            Self::TOKYO => write!(f, "jp_13"),
            Self::KANAGAWA => write!(f, "jp_14"),
            Self::NIIGATA => write!(f, "jp_15"),
            Self::TOYAMA => write!(f, "jp_16"),
            Self::ISHIKAWA => write!(f, "jp_17"),
            Self::FUKUI => write!(f, "jp_18"),
            Self::YAMANASHI => write!(f, "jp_19"),
            Self::NAGANO => write!(f, "jp_20"),
            Self::GIFU => write!(f, "jp_21"),
            Self::SHIZUOKA => write!(f, "jp_22"),
            Self::AICHI => write!(f, "jp_23"),
            Self::MIE => write!(f, "jp_24"),
            Self::SHIGA => write!(f, "jp_25"),
            Self::KYOTO => write!(f, "jp_26"),
            Self::OSAKA => write!(f, "jp_27"),
            Self::HYOUGO => write!(f, "jp_28"),
            Self::NARA => write!(f, "jp_29"),
            Self::WAKAYAMA => write!(f, "jp_30"),
            Self::TOTTORI => write!(f, "jp_31"),
            Self::SHIMANE => write!(f, "jp_32"),
            Self::OKAYAMA => write!(f, "jp_33"),
            Self::HIROSHIMA => write!(f, "jp_34"),
            Self::YAMAGUCHI => write!(f, "jp_35"),
            Self::TOKUSHIMA => write!(f, "jp_36"),
            Self::KAGAWA => write!(f, "jp_37"),
            Self::EHIME => write!(f, "jp_38"),
            Self::KOUCHI => write!(f, "jp_39"),
            Self::FUKUOKA => write!(f, "jp_40"),
            Self::SAGA => write!(f, "jp_41"),
            Self::NAGASAKI => write!(f, "jp_42"),
            Self::KUMAMOTO => write!(f, "jp_43"),
            Self::OITA => write!(f, "jp_44"),
            Self::MIYAZAKI => write!(f, "jp_45"),
            Self::KAGOSHIMA => write!(f, "jp_46"),
            Self::OKINAWA => write!(f, "jp_47"),
            Self::TAIPEI_CITY => write!(f, "tw_01"),
            Self::NEW_TAIPEI_CITY => write!(f, "tw_02"),
            Self::TAOYUAN_CITY => write!(f, "tw_03"),
            Self::TAICHUNG_CITY => write!(f, "tw_04"),
            Self::TAINAN_CITY => write!(f, "tw_05"),
            Self::KAOHSIUNG_CITY => write!(f, "tw_06"),
            Self::KEELUNG_CITY => write!(f, "tw_07"),
            Self::HSINCHU_CITY => write!(f, "tw_08"),
            Self::CHIAYI_CITY => write!(f, "tw_09"),
            Self::HSINCHU_COUNTY => write!(f, "tw_10"),
            Self::MIAOLI_COUNTY => write!(f, "tw_11"),
            Self::CHANGHUA_COUNTY => write!(f, "tw_12"),
            Self::NANTOU_COUNTY => write!(f, "tw_13"),
            Self::YUNLIN_COUNTY => write!(f, "tw_14"),
            Self::CHIAYI_COUNTY => write!(f, "tw_15"),
            Self::PINGTUNG_COUNTY => write!(f, "tw_16"),
            Self::YILAN_COUNTY => write!(f, "tw_17"),
            Self::HUALIEN_COUNTY => write!(f, "tw_18"),
            Self::TAITUNG_COUNTY => write!(f, "tw_19"),
            Self::PENGHU_COUNTY => write!(f, "tw_20"),
            Self::KINMEN_COUNTY => write!(f, "tw_21"),
            Self::LIENCHIANG_COUNTY => write!(f, "tw_22"),
            Self::BANGKOK => write!(f, "th_01"),
            Self::PATTAYA => write!(f, "th_02"),
            Self::NORTHERN => write!(f, "th_03"),
            Self::CENTRAL => write!(f, "th_04"),
            Self::SOUTHERN => write!(f, "th_05"),
            Self::EASTERN => write!(f, "th_06"),
            Self::NORTHEASTERN => write!(f, "th_07"),
            Self::WESTERN => write!(f, "th_08"),
            Self::BALI => write!(f, "id_01"),
            Self::BANDUNG => write!(f, "id_02"),
            Self::BANJARMASIN => write!(f, "id_03"),
            Self::JABODETABEK => write!(f, "id_04"),
            Self::MAKASSAR => write!(f, "id_05"),
            Self::MEDAN => write!(f, "id_06"),
            Self::PALEMBANG => write!(f, "id_07"),
            Self::SAMARINDA => write!(f, "id_08"),
            Self::SEMARANG => write!(f, "id_09"),
            Self::SURABAYA => write!(f, "id_10"),
            Self::YOGYAKARTA => write!(f, "id_11"),
            Self::LAINNYA => write!(f, "id_12"),
        }
    }
}

impl Default for AreaDemographic {
    fn default() -> AreaDemographic {
        Self::HOKKAIDO
    }
}

