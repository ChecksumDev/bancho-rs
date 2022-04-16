// //use crate::constants::privileges::ClientPrivileges;
// // use constants::packets::*;
// // use packets::*;
// // use packets::{PACKET_HANDLERS, RESTRICTED_PACKET_HANDLERS};
// // use crate::packets::{PACKET_HANDLERS, RESTRICTED_PACKET_HANDLERS};
// // use crate::utils::LoginResponse;
// // use actix_web::{error, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
// // use async_once::AsyncOnce;
// // use futures_util::stream::StreamExt as _;
// // use mongodb::{options::ClientOptions, Client};
// // use std::cell::RefCell;
// // use std::sync::atomic::{AtomicUsize, Ordering};
// // use std::sync::Mutex;
// // use std::{collections::HashMap, collections::HashSet, sync::Arc};
// // //use structs::player::Player;
// // use crate::constants::packets::*;
// // use crate::packets::Reader;
// // use crate::structs::player::Player;
// // use crate::utils;
// // use crate::{PLAYERS, PLAYER_COUNT};
// // use tokio::net::{TcpListener, TcpStream};

// pub async fn handle_stream(data: Vec<u8>, mut player: Player) -> Result<HttpResponse, Error> {
//     let mut _reader = Reader::new(data);

//     while !_reader.empty() {
//         let (id, len) = _reader.read_header();
//         let packet = unsafe { std::mem::transmute::<i16, Packets>(id as i16) };

//         //println!("Packet: {:?}", &packet);

//         // &* lmao
//         let mut handler_map = &*PACKET_HANDLERS;
//         if player.restricted() {
//             handler_map = &*RESTRICTED_PACKET_HANDLERS;
//         }

//         if packet == Packets::OsuPing {
//             continue;
//         }

//         if packet != Packets::OsuPing {
//             println!("Packet {:?} res for {}", packet, player.name);
//         }

//         if handler_map.contains_key(&packet) {
//             let callback = handler_map[&packet];
//             let r = callback(&mut player, &mut _reader).await?;
//             return Ok(HttpResponse::Ok().body(r));

//             // if should_increment {
//             //     _reader.incr_offset(len as usize);
//             // }

//             // if packet != Packets::OsuPing {
//             //     println!("Packet {:?} handled for {}", packet, player.name);
//             // }
//         } else {
//             _reader.incr_offset(len as usize);
//         }
//     }

//     Ok((HttpResponse::Ok().body(b"".to_vec())).into())
// }

// pub async fn index(mut body: web::Payload, res: HttpRequest) -> Result<HttpResponse, Error> {
//     //println!("{:?}", res.path());
//     // for demonstration only; in a normal case use the `Bytes` extractor
//     // collect payload stream into a bytes object

//     let header = match res.headers().get("osu-token") {
//         Some(h) => h.to_str().unwrap(),
//         None => {
//             // the client is preforming a login
//             println!("No osu-token header found");

//             let mut bytes = Vec::new();
//             while let Some(item) = body.next().await {
//                 bytes.extend_from_slice(&item?);
//             }

//             let mut reader = Reader::new(bytes.clone());

//             let (id, _) = reader.read_header();
//             // let packet = unsafe { std::mem::transmute::<i16, Packets>(1028 as i16) };
//             //println!("{:?}", packet);
//             //println!("{:?}", &id);
//             //println!("{:?}", String::from_utf8(bytes.clone()));

//             let login_data = utils::parse_login_data(bytes.clone());

//             let mut player = Player::new(
//                 login_data.username.as_str(),
//                 "727@blue.zenith",
//                 login_data.password_md5.as_str(),
//                 "US",
//             );

//             player.generate_token();
//             player.set_id(33);

//             PLAYERS
//                 .lock()
//                 .unwrap()
//                 .insert(player.memory.token.clone(), player.clone());
//             PLAYER_COUNT.fetch_add(1, Ordering::SeqCst);

//             println!("PLAYER_COUNT: {}", PLAYER_COUNT.load(Ordering::SeqCst)); // hey there cutie

//             let login_res = LoginResponse::new(player.memory.token, bytes);
//             // let mut data: Vec<u8> = Vec::new();
//             // let mut packet_writer = packets::PacketWriter::new(Packets::ChoNotification);
//             // packet_writer += "BLUE ZENITH 727!!!!".to_string();
//             // data.append(&mut packet_writer.serialise());
//             // let mut packet_writer = packets::PacketWriter::new(Packets::ChoRestart);
//             // packet_writer += 0;
//             //data.append(&mut packet_writer.serialise());

//             //return Ok(HttpResponse::Ok().append_header(("cho-token", login_res.cho_token)).body(data));

//             /*
//                 packets needed:
//                 - protocol version (19) 1
//                 - user_id (p.userid) 2
//                 - bancho_privileges 3
//                 - notification 4
//                 - channel info 5
//                 - channel info end 6
//                 - main menu icon 7
//                 - friends list 
//                 - silence end
//                 - user presence
//                 - user stats
//                 - send message 
//             */

//             let mut data = Vec::new();
//             let id = player.id.unwrap();
//             data.append(&mut ChoProtocolVersion::new(19).await.write().await?); // CLEAR
//             data.append(&mut ChoUserId::new(id).await.write().await?); //CLEAR
//             data.append(&mut ChoPrivileges::new(ClientPrivileges::OWNER.bits() as i32).await.write().await?); // CLEAR
//             data.append(&mut ChoNotification::new("Welcome back to cmyui.xyz!\nRunning bancho.py v4.3.2.".to_string()).await.write().await?); //CLEAR
//             data.append(&mut ChoChannelInfo::new("#osu", "General discussion.", 2).await.write().await?);
//             data.append(&mut ChoChannelInfo::new("#announce", "Exemplary performance and public announcements.", 1).await.write().await?);
//             data.append(&mut ChoChannelInfoEnd::write().await?);
//             data.append(&mut ChoMainMenuIcon::new("https://akatsuki.pw/static/logos/logo_ingame.png", "https://akatsuki.pw").await.write().await?);
//             data.append(&mut ChoFriendsList::new(vec![]).await.write().await?);
//             data.append(&mut ChoSilenceEnd::new(0).await.write().await?);
//             data.append(&mut ChoUserPresence::new(id, &player.name, 0 as u8, 0 as u8, ClientPrivileges::OWNER.bits() as u8, 0.0, 0.0, 727).await.write().await?);
//             data.append(&mut ChoUserStats::new(id, 0, "", "", 0, 0, 0, 727, 69.420, 727, 727, 727, 727 as i16).await.write().await?);
//             data.append(&mut ChoSendMessage::new("swargy", "Hey there cutie", &player.name.as_str(), 3).await.write().await?);

//             return Ok(HttpResponse::Ok()
//                 .append_header(("cho-token", login_res.cho_token))
//                 //.body("\x05\x00\x00\x04\x00\x00\x00\x7f\x7f\x7f\x7f"));
//                 //.body(base64::decode_config("SwAABAAAABMAAAAFAAAEAAAAIQAAAEcAAAQAAAAIAAAAGAAANgAAAAs0V2VsY29tZSBiYWNrIHRvIGNteXVpLnh5eiEKUnVubmluZyBiYW5jaG8ucHkgdjQuMy4yLkEAAB8AAAALBCNvc3ULE0dlbmVyYWwgZGlzY3Vzc2lvbi4CAEEAAEAAAAALCSNhbm5vdW5jZQsvRXhlbXBsYXJ5IHBlcmZvcm1hbmNlIGFuZCBwdWJsaWMgYW5ub3VuY2VtZW50cy4BAFkAAAAAAABMAABMAAAARGh0dHBzOi8vYWthdHN1a2kucHcvc3RhdGljL2xvZ29zL2xvZ29faW5nYW1lLnBuZ3xodHRwczovL2FrYXRzdWtpLnB3SAAACAAAAAAAAAAAAAAAXAAABAAAAAAAAABTAAAjAAAAIQAAAAgAAAAAAAAAQ2hlY2tzdW0YAAgAAAAAAAAAANcCAAALAAAuAAAAIQAAAAAAAAAAAAAAAAAAANcCAAAAAAAACteKQtcCAADXAgAAAAAAANcCAADXAgcAACcAAAALBnN3YXJneQsPSGV5IHRoZXJlIGN1dGllCwhDaGVja3N1bQMAAAA=", base64::URL_SAFE).unwrap()));
//                 .body(data));
//                 //.body(ChoNotification::new("Server has restarted.").await.write().await?))
//         }
//     };

//     let mut bytes = Vec::new();
//     while let Some(item) = body.next().await {
//         bytes.extend_from_slice(&item?);
//     }

//     //println!("{:?}", String::from_utf8(bytes.clone()).unwrap());
//     //println!("osu-token: {}", header);

//     let player = PLAYERS.lock().unwrap();
//     let player = player.get(header).expect("Player not found");

//     Ok(handle_stream(bytes, player.to_owned()).await?)
//     //Ok(HttpResponse::Ok().body("Hello world!"))
// }
