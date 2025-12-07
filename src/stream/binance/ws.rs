
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async};
use tokio_tungstenite::tungstenite::{Message};
use tokio_tungstenite::MaybeTlsStream;
use reqwest::{Error};
use sha2::{Digest, Sha256};
use chrono::Utc;
use super::spot;
use tokio_tungstenite::WebSocketStream;
use futures_util::SinkExt;
use crate::utils::{get_env,create_signature};

#[allow(dead_code)]
pub struct WebSocketAPI{
    //url: String,
    api_host: String,
    api_key: String,
    api_secret:String,
    websocket_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    
}

impl WebSocketAPI {
    pub async fn new () -> Result<Self, Box<dyn std::error::Error>> {
        let api_host = get_env("WS_API_HOST");
        let api_key = get_env("API_KEY");
        let api_secret = get_env("API_KEY");
   
        let (ws_stream, response) = connect_async(&api_host).await?;
        println!("WebSocket connection established. Status: {}", response.status());
        Ok(Self {api_host,api_key,api_secret,websocket_stream: ws_stream})
    }

    fn gen_id ( event : &str)  -> String {
        let current_timestamp = Utc::now().timestamp_millis();
        let timestamp_string = current_timestamp.to_string();
        let data_to_hash = timestamp_string.as_bytes();
        let mut hasher = Sha256::new();
        hasher.update(data_to_hash);
        hasher.update(event.as_bytes());
        let result_bytes = hasher.finalize();
        let hex_hash = hex::encode(result_bytes);
        hex_hash
    }

    //haddle this
    pub async fn response(
        // แทนที่ด้วยประเภท Stream/Sink ที่คุณใช้จริง
        mut ws_stream: tokio_tungstenite::WebSocketStream<impl tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin>
    ) -> Result<(), Error> {
    
        println!("Starting to listen for WebSocket messages...");
    
        // วนลูปเพื่ออ่านข้อความที่เข้ามาอย่างต่อเนื่อง
        while let Some(msg) = ws_stream.next().await {
            
            let msg = match msg {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("WebSocket Error: {}", e);
                    break; // หยุดลูปหากเกิดข้อผิดพลาดในการรับข้อความ
                }
            };
    
            match msg {
                // 1. จัดการข้อความหลักที่เป็น JSON
                Message::Text(text) => {
                    println!("--- Received Text Message ---");
                    println!("{}", text);
                    
                    // ลอง Deserialize JSON เป็น Struct
                    match serde_json::from_str::<BinanceResponse>(&text) {
                        Ok(response) => {
                            // การตอบกลับ Logon สำเร็จ
                            if response.status.as_deref() == Some("ok") {
                                println!("✅ Session Logon Successful! ID: {:?}", response.id);
                            } else if response.error.is_some() {
                                // การตอบกลับที่มี Error
                                eprintln!("❌ API Error Response: {:?}", response.error);
                            } else {
                                // การตอบกลับข้อมูล Stream อื่นๆ (เช่น Market Data, Order Updates)
                                println!("📄 Generic Data Stream: {:?}", response.result);
                            }
                        },
                        Err(_) => {
                            // บางข้อความอาจไม่ใช่ Response ที่มี ID/Status ตรงๆ (เช่น PONG หรือ Event)
                            println!("⚠️ Could not parse as standard BinanceResponse.");
                        }
                    }
                }
                
                // 2. จัดการข้อความ Binary (ไม่ค่อยพบใน API ทั่วไป)
                Message::Binary(bin) => {
                    println!("Received Binary Message ({} bytes)", bin.len());
                }
    
                // 3. จัดการ PING/PONG (ใช้สำหรับ Keep-Alive)
                Message::Ping(_) => {
                    // โดยทั่วไป tokio-tungstenite จะตอบกลับ PONG ให้อัตโนมัติ
                    // หากไม่: ws_stream.send(Message::Pong(vec![])).await?;
                    println!("Received Ping. (Auto-Ponged)");
                }
    
                // 4. จัดการการปิดการเชื่อมต่อ
                Message::Close(close_frame) => {
                    println!("Connection Closed by Server: {:?}", close_frame);
                    break; // ออกจากลูป
                }
                
                _ => { /* ละเว้นข้อความประเภทอื่น ๆ */ }
            }
        }
    
        Ok(())
    }

    pub async fn login (&mut self) -> Result< (), Box<dyn std::error::Error>>  {
        let event = "session.logon";
        let timestamp = Utc::now().timestamp_millis();
        let signature :String = create_signature(
            &vec![
                ("apiKey",self.api_key.to_string()),
                ("timestamp",timestamp.to_string())
            ],
            &self. api_secret).unwrap();

        let params = spot::ws::LoginParams {
            api_key: self.api_key.to_string(),
            signature: signature.clone(),
            timestamp
        };

        let login_request = spot::ws::LoginRequest {
            id: WebSocketAPI::gen_id(&event),
            method: event.to_string(),
            params,
        };

        let json_message = serde_json::to_string(&login_request)?;
        //websocket_stream.send(Message::Text(json_message)).await?;
        self.websocket_stream
        .send(Message::Text(json_message.into()))
        .await?;
        
        Ok(())
    
    }
        
        
        
        

}










// api_key:String,
// signature: String,
// timestamp: i64,
//Log in with API key (SIGNED)
// {
//   "id": "c174a2b1-3f51-4580-b200-8528bd237cb7",
//   "method": "session.logon",
//   "params": {
//     "apiKey": "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
//     "signature": "1cf54395b336b0a9727ef27d5d98987962bc47aca6e13fe978612d0adee066ed",
//     "timestamp": 1649729878532
//   }
// }
//----res---
// {
//     "id": "c174a2b1-3f51-4580-b200-8528bd237cb7",
//     "status": 200,
//     "result": {
//       "apiKey": "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
//       "authorizedSince": 1649729878532,
//       "connectedSince": 1649729873021,
//       "returnRateLimits": false,
//       "serverTime": 1649729878630,
//       "userDataStream": false // is User Data Stream subscription active?
//     }
//   }
//session status
// //{
//     "id": "b50c16cd-62c9-4e29-89e4-37f10111f5bf",
//     "method": "session.status"
//   }
//{
//     "id": "b50c16cd-62c9-4e29-89e4-37f10111f5bf",
//     "status": 200,
//     "result": {
//       // if the connection is not authenticated, "apiKey" and "authorizedSince" will be shown as null
//       "apiKey": "vmPUZE6mv9SD5VNHk4HlWFsOr6aKE2zvsw0MuIgwCIPy6utIco14y7Ju91duEh8A",
//       "authorizedSince": 1649729878532,
//       "connectedSince": 1649729873021,
//       "returnRateLimits": false,
//       "serverTime": 1649730611671,
//       "userDataStream": true // is User Data Stream subscription active?
//     }
//   }
//logout 
//{
//     "id": "c174a2b1-3f51-4580-b200-8528bd237cb7",
//     "method": "session.logout"
//   }
 //---rest
// {
//     "id": "c174a2b1-3f51-4580-b200-8528bd237cb7",
//     "status": 200,
//     "result": {
//       "apiKey": null,
//       "authorizedSince": null,
//       "connectedSince": 1649729873021,
//       "returnRateLimits": false,
//       "serverTime": 1649730611671,
//       "userDataStream": false // is User Data Stream subscription active?
//     }
//   }
//Place new order (TRADE)
//Cancel active order (TRADE)
//Cancel open orders order (TRADE)
//
//Subscribe to User Data Stream
// {
//     "id": "d3df8a21-98ea-4fe0-8f4e-0fcea5d418b7",
//     "method": "userDataStream.subscribe"
//   }
//----return
// {
//     "id": "d3df8a21-98ea-4fe0-8f4e-0fcea5d418b7",
//     "status": 200,
//     "result": {
//       "subscriptionId": 0
//     }
//   }
//----------------------
// let res = client
// .get(&url)
// .header("X-MBX-APIKEY", &api_key)
// .header("Accept", "application/json")
// .send()
// .await?;

// let status = res.status();
// let text = res.text().await?;
//unsub
//{
//     "id": "d3df8a21-98ea-4fe0-8f4e-0fcea5d418b7",
//     "method": "userDataStream.unsubscribe"
//   }
//--res
// {
//   "id": "d3df8a21-98ea-4fe0-8f4e-0fcea5d418b7",
//   "status": 200,
//   "result": {}
// }
//Subscribe to User Data Stream through signature subscription (USER_STREAM)
// {
//   "id": "d3df8a22-98ea-4fe0-9f4e-0fcea5d418b7",
//   "method": "userDataStream.subscribe.signature",
//   "params": {
//     "apiKey": "mjcKCrJzTU6TChLsnPmgnQJJMR616J4yWvdZWDUeXkk6vL6dLyS7rcVOQlADlVjA",
//     "timestamp": 1747385641636,
//     "signature": "yN1vWpXb+qoZ3/dGiFs9vmpNdV7e3FxkA+BstzbezDKwObcijvk/CVkWxIwMCtCJbP270R0OempYwEpS6rDZCQ=="
//   }
// }
// // //println!("{} : {}", status.as_u16(), status.as_str());

// if status.is_success() {
// let ob: OrderBook = serde_json::from_str(&text)?;
// Ok(ob)
// } else {
// let err = format!("status {} : {}", status.as_u16(), text);
// Err(err.into())
// }