use crate::AssetData;
use crate::variables::CHAIN_ID_TO_STRING;
use alloy::primitives::Address;
use chrono::Local;
use google_sheets4::{
    FieldMask, Sheets,
    api::{
        AddSheetRequest, BatchUpdateSpreadsheetRequest, BatchUpdateValuesRequest, Border, CellData,
        CellFormat, GridRange, NumberFormat, RepeatCellRequest, Request, SheetProperties,
        TextFormat, UpdateBordersRequest, ValueRange,
    },
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    hyper_util::{
        client::legacy::{Client, connect::HttpConnector},
        rt::TokioExecutor,
    },
    yup_oauth2::{ServiceAccountAuthenticator, read_service_account_key},
};
use std::collections::hash_map::HashMap;

pub async fn write_to_google_sheet(asset_data: &[AssetData]) {
    let creds = read_service_account_key("jooce-cred.json")
        .await
        .expect("Can't read credential, an error occurred");
    let auth = ServiceAccountAuthenticator::builder(creds)
        .build()
        .await
        .expect("There was an error, trying to build connection with authenticator");

    let hub = Sheets::new(
        Client::builder(TokioExecutor::new()).build(
            HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_only()
                .enable_all_versions()
                .build(),
        ),
        auth,
    );
    // Get sheet index before we add new sheet
    let sheet_index = get_last_sheet_properties(&hub).await.index.unwrap();
    let new_sheet_id = sheet_index + 2;

    let mut sheet_data = Vec::with_capacity(asset_data.len() + 2);
    sheet_data.push(vec![
        serde_json::to_value("Asset").unwrap(),
        serde_json::to_value("Percentage").unwrap(),
        serde_json::to_value("Uint16").unwrap(),
        serde_json::to_value("Chain").unwrap(),
        serde_json::to_value("").unwrap(),
    ]);

    asset_data.iter().for_each(|asset| {
        let row = vec![
            serde_json::to_value(asset.symbol.as_ref().unwrap()).unwrap(),
            serde_json::to_value(asset.actual_weight.unwrap()).unwrap(),
            serde_json::to_value(asset.converted_weight.unwrap()).unwrap(),
            serde_json::to_value(CHAIN_ID_TO_STRING.get(&asset.chain_id).unwrap()).unwrap(),
        ];
        sheet_data.push(row);
    });

    let data_length = sheet_data.len();
    sheet_data.push(vec![
        serde_json::to_value("Checks").unwrap(),
        serde_json::to_value(format!("=1-SUM(B2:B{})", data_length)).unwrap(),
        serde_json::to_value(format!("={}-SUM(C2:C{})", u16::MAX, data_length)).unwrap(),
    ]);

    let composition_value_range = ValueRange {
        major_dimension: None,
        range: Some(format!("{}!A1:E50", new_sheet_id)),
        values: Some(sheet_data),
    };

    let snapshot_value_range = ValueRange {
        major_dimension: None,
        range: Some(format!("{}!F1:G1", new_sheet_id)),
        values: Some(vec![vec![
            serde_json::to_value("Snapshot Date").unwrap(),
            serde_json::to_value(format!("{}", Local::now().format("%d/%m/%Y"))).unwrap(),
        ]]),
    };
    add_sheet_and_set_formatting(&hub, new_sheet_id, data_length).await;
    let req = hub
        .spreadsheets()
        .values_batch_update(
            BatchUpdateValuesRequest {
                data: Some(vec![snapshot_value_range, composition_value_range]),
                value_input_option: Some("USER_ENTERED".to_owned()),
                ..BatchUpdateValuesRequest::default()
            },
            std::env::var("SPREADSHEET_ID").unwrap().as_str(),
        )
        .doit()
        .await;
    println!("{:?}", req);
}

pub fn print_hashmap(asset_data: &[AssetData]) {
    let mut map: HashMap<&Address, u16> = HashMap::new();
    map.insert(&Address::ZERO, 0);
    for i in asset_data {
        map.insert(&i.token_addr, i.converted_weight.unwrap());
    }
    println!("{:?}", map)
}

async fn get_last_sheet_properties(hub: &Sheets<HttpsConnector<HttpConnector>>) -> SheetProperties {
    hub.spreadsheets()
        .get(std::env::var("SPREADSHEET_ID").unwrap().as_str())
        .doit()
        .await
        .unwrap()
        .1
        .sheets
        .unwrap()
        .last()
        .unwrap()
        .properties
        .clone()
        .unwrap()
}

async fn batch_update_request(
    hub: &Sheets<HttpsConnector<HttpConnector>>,
    updates: Option<Vec<Request>>,
) {
    let req = hub
        .spreadsheets()
        .batch_update(
            BatchUpdateSpreadsheetRequest {
                requests: updates,
                ..BatchUpdateSpreadsheetRequest::default()
            },
            std::env::var("SPREADSHEET_ID").unwrap().as_str(),
        )
        .doit()
        .await;

    println!("{:?}", req);
}

async fn add_sheet_and_set_formatting(
    hub: &Sheets<HttpsConnector<HttpConnector>>,
    sheet_id: i32,
    data_length: usize,
) {
    // Create struct to add new sheet to spreadsheet
    let add_new_sheet = Request {
        add_sheet: Some(AddSheetRequest {
            properties: Some(SheetProperties {
                title: Some(sheet_id.to_string()),
                sheet_id: Some(sheet_id),
                ..SheetProperties::default()
            }),
        }),
        ..Request::default()
    };

    // Create struct to bold first row
    let bold_first_row = Request {
        repeat_cell: Some(RepeatCellRequest {
            range: Some(GridRange {
                sheet_id: Some(sheet_id),
                end_row_index: Some(1),
                end_column_index: Some(6),
                ..GridRange::default()
            }),
            fields: Some(FieldMask::new(&[
                "userEnteredFormat.textFormat.bold".to_owned()
            ])),
            cell: Some(CellData {
                user_entered_format: Some(CellFormat {
                    text_format: Some(TextFormat {
                        bold: Some(true),
                        ..TextFormat::default()
                    }),
                    ..CellFormat::default()
                }),
                ..CellData::default()
            }),
        }),
        ..Request::default()
    };
    // Convert weight column to percentage
    let convert_column_percent = Request {
        repeat_cell: Some(RepeatCellRequest {
            range: Some(GridRange {
                sheet_id: Some(sheet_id),
                start_column_index: Some(1),
                end_column_index: Some(2),
                ..GridRange::default()
            }),
            fields: Some(FieldMask::new(&[
                "userEnteredFormat.numberFormat".to_owned()
            ])),
            cell: Some(CellData {
                user_entered_format: Some(CellFormat {
                    number_format: Some(NumberFormat {
                        pattern: Some("0.00%".to_owned()),
                        type_: Some("PERCENT".to_owned()),
                    }),
                    ..CellFormat::default()
                }),
                ..CellData::default()
            }),
        }),
        ..Request::default()
    };

    // Set snapshot date to a date format
    let set_snapshot_type = Request {
        repeat_cell: Some(RepeatCellRequest {
            range: Some(GridRange {
                sheet_id: Some(sheet_id),
                start_column_index: Some(6),
                end_column_index: Some(7),
                start_row_index: Some(0),
                end_row_index: Some(1),
            }),
            fields: Some(FieldMask::new(&[
                "userEnteredFormat.numberFormat".to_owned()
            ])),
            cell: Some(CellData {
                user_entered_format: Some(CellFormat {
                    number_format: Some(NumberFormat {
                        type_: Some("DATE".to_owned()),
                        pattern: Some("dd/mm/yyyy".to_owned()),
                    }),
                    ..CellFormat::default()
                }),
                ..CellData::default()
            }),
        }),
        ..Request::default()
    };

    // Create struct to set itlaics on check row
    let italic_check_row = Request {
        repeat_cell: Some(RepeatCellRequest {
            range: Some(GridRange {
                sheet_id: Some(sheet_id),
                start_row_index: Some(data_length as i32),
                end_row_index: Some(data_length as i32 + 1),
                ..GridRange::default()
            }),
            fields: Some(FieldMask::new(&[
                "userEnteredFormat.textFormat.italic".to_owned()
            ])),
            cell: Some(CellData {
                user_entered_format: Some(CellFormat {
                    text_format: Some(TextFormat {
                        italic: Some(true),
                        ..TextFormat::default()
                    }),
                    ..CellFormat::default()
                }),
                ..CellData::default()
            }),
        }),
        ..Request::default()
    };

    let set_borders = Request {
        update_borders: Some(UpdateBordersRequest {
            range: Some(GridRange {
                end_column_index: Some(4),
                end_row_index: Some(data_length as i32),
                sheet_id: Some(sheet_id),
                ..GridRange::default()
            }),
            bottom: Some(Border {
                width: Some(1),
                style: Some("DOUBLE".to_owned()),
                ..Border::default()
            }),
            top: Some(Border {
                width: Some(1),
                style: Some("SOLID".to_owned()),
                ..Border::default()
            }),
            left: Some(Border {
                width: Some(1),
                style: Some("SOLID".to_owned()),
                ..Border::default()
            }),
            right: Some(Border {
                width: Some(1),
                style: Some("SOLID".to_owned()),
                ..Border::default()
            }),
            inner_horizontal: Some(Border {
                width: Some(1),
                style: Some("SOLID".to_owned()),
                ..Border::default()
            }),
            inner_vertical: Some(Border {
                width: Some(1),
                style: Some("SOLID".to_owned()),
                ..Border::default()
            }),
        }),
        ..Request::default()
    };

    batch_update_request(
        hub,
        Some(vec![
            add_new_sheet,
            bold_first_row,
            convert_column_percent,
            set_snapshot_type,
            italic_check_row,
            set_borders,
        ]),
    )
    .await;
}
