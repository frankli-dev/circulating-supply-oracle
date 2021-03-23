use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use hex_literal::hex;
use web3::{
    contract::{Contract, Options},
    types::{U256, H160},
};
use std::env;
use dotenv::dotenv;

async fn check(infura_key: &str) -> web3::contract::Result<U256> {
    let transport = web3::transports::WebSocket::new(format!("wss://mainnet.infura.io/ws/v3/{}", infura_key).as_str()).await?;
    let web3 = web3::Web3::new(transport);

    // Accessing existing contract.
    let contract_address: H160 = hex!("44f262622248027f8e2a8fb1090c4cf85072392c").into();

    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../abi/token.json"),
    )?;

    let accounts: Vec<H160> = vec![
        hex!("40DD33bBe1FCcE62AD73040Fa07F8b1037ca6C7c").into(),
        hex!("f66005e3fd85d4e7e6293B96FCf9Eadd32c56Cd2").into(), 
        hex!("A16aF2b4A072F10FC2FDf505a76a0Dc7868b82cD").into(), 
        hex!("bC7175790c7EAf4A429Ae0d20a98862FC1F0352A").into(), 
        hex!("EAe020457249D82488dCEB45eB8E76258C6B4d61").into(),
        hex!("09cBA546a88CF031fFd69a21565E41e9Ca69108b").into(),
    ];

    let mut sum: U256 = U256::zero();
    let total_supply: U256 = U256::from_dec_str("90000000000000000000000000").unwrap();
    let decimals: U256 = U256::from_dec_str("1000000000000000000").unwrap();

    for account in accounts {
        let result = contract.query("balanceOf", account, None, Options::default(), None);
        let balance_of: U256 = result.await?;
        sum = sum + balance_of;
    }

    println!("sum: {}", sum);

    let circulating_supply = (total_supply - sum)/decimals;

    println!("Circulating Supply: {} => {}", (total_supply - sum), circulating_supply);

    Ok(circulating_supply)
}

#[get("/supply")]
async fn supply() -> impl Responder {
    match env::var("INFURA_KEY") {
        Err(_e) => HttpResponse::Ok().body("Invalid key"),
        Ok(infura_key) => {
            match check(&infura_key).await {
                Err(_e) => HttpResponse::Ok().body("Something went wrong"),
                Ok(result) => {
                    HttpResponse::Ok()
                        .content_type("application/json; charset=utf-8")
                        .header("Access-Control-Allow-Origin", "*")
                        .body(format!("{}", result))
                }
            }
        }
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Status: Green")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(supply)
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}