use reqwest::Error;
use serde::Deserialize;

// Structs
#[derive(Deserialize)]
struct Currency {
    code: String,
    amount: Option<f32>, // amount might be null
}

#[derive(Deserialize, Debug)]
struct Comparison {
    #[serde(rename = "sourceCurrency")]
    source_currency: String,

    #[serde(rename = "targetCurrency")]
    target_currency: String,

    amount: f64,

    #[serde(rename = "amountType")]
    amount_type: String,

    providers: Vec<Provider>,
}

#[derive(Deserialize, Debug)]
struct Provider {
    id: u32,
    alias: String,
    name: String,
    quotes: Vec<Quote>,
}

#[derive(Deserialize, Debug)]
struct Quote {
    rate: f64,
    fee: f64,
    #[serde(rename = "receivedAmount")]
    received_amount: f64,
    #[serde(rename = "sendAmount")]
    send_amount: Option<f64>,
}



#[tauri::command]
fn greet(origincurrency: Currency, targetcurrencies: Vec<Currency>) -> f32 {
    log::info!("Tauri is awesome!");
    log::info!("Origin: {} {:?}", origincurrency.code, origincurrency.amount);
    log::info!("Target: {} {:?}", targetcurrencies[0].code, targetcurrencies[0].amount);

    let send_amount = origincurrency.amount.unwrap_or(0.0) as f64;
    let target_currency = &targetcurrencies[0].code;

    let rt = tokio::runtime::Runtime::new().unwrap();

    let result: f32 = rt.block_on(async {
        let url = format!(
            "https://api.wise.com/v4/comparisons/?sourceCurrency={}&targetCurrency={}&sendAmount={}",
            origincurrency.code, target_currency, send_amount
        );

        match reqwest::get(&url).await {
            Ok(resp) => {
                let json: Result<Comparison, _> = resp.json().await;
                match json {
                    Ok(data) => {
                        log::info!("Received comparison: {:?}", data);
        
                        // request wise rate
                        if let Some(provider) = data.providers.iter().find(|p| p.alias == "wise") {
                            if let Some(quote) = provider.quotes.first() {
                                return quote.rate as f32;
                            }
                        }
        
                        0.0
                    }
                    Err(e) => {
                        log::error!("Failed to parse JSON: {}", e);
                        0.0
                    }
                }
            }
            Err(e) => {
                log::error!("Request failed: {}", e);
                0.0
            }
        }
    });

    result
}




#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
