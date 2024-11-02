#![allow(unused)]

use io::{Packet, CRC};
use sailfish::TemplateSimple;
use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use warp::http::{Response, Uri};
use warp::{Filter, Rejection, Reply};

enum Clients {
    TypeScript(TypeScriptClient),
    JavaUnsigned(JavaUnsignedClient),
    Java(JavaClient),
    TeaVM(TeaVMClient),
}

#[derive(TemplateSimple)]
#[template(path = "tsclient.ejs")]
struct TypeScriptClient {
    plugin: String,
    nodeid: String,
    portoff: String,
    lowmem: String,
    members: String,
}

#[derive(TemplateSimple)]
#[template(path = "javaclientunsigned.ejs")]
struct JavaUnsignedClient {
    plugin: String,
    nodeid: String,
    portoff: String,
    lowmem: bool,
    members: bool,
}

#[derive(TemplateSimple)]
#[template(path = "javaclient.ejs")]
struct JavaClient {
    plugin: String,
    nodeid: String,
    portoff: String,
    lowmem: bool,
    members: bool,
}

#[derive(TemplateSimple)]
#[template(path = "teavmclient.ejs")]
struct TeaVMClient {
    plugin: String,
    nodeid: String,
    portoff: String,
    lowmem: String,
    members: String,
}

#[tokio::main]
#[rustfmt::skip]
async fn main() {
    let nodeid: String = std::env::var("NODE_ID").unwrap();
    let portoff: String =(std::env::var("NODE_PORT").unwrap().parse::<i32>().unwrap() - 43594).to_string();
    let members: bool = std::env::var("NODE_MEMBERS").unwrap() == "true";

    // static assets (.js, .wasm, .sf2)
    let assets = warp::path::full().and(warp::get()).and_then(assets);

    // game clients
    // http://localhost/rs2.cgi?lowmem=0&plugin=0
    let rs2cgi = warp::path("rs2.cgi")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(move |params: HashMap<String, String>| {
            rs2cgi(
                params.get("plugin").map(String::as_str).unwrap_or("0").to_string().clone(),
                params.get("lowmem").map(String::as_str).unwrap_or("0").to_string().clone(),
                nodeid.clone(),
                portoff.clone(),
                members,
            )
        });

    // cache
    let cache = warp::path::full()
        .and(warp::get())
        .and_then(cache);

    // everything else
    let default = warp::path::end()
        .map(|| warp::redirect::found(Uri::from_static("/rs2.cgi?lowmem=0&plugin=0")));

    // open the socket
    warp::serve(
        assets
            .or(rs2cgi)
            .or(cache)
            .or(default)
    ).run(([0, 0, 0, 0], 80)).await;
}

fn crcs() -> Vec<u8> {
    let crc: CRC = CRC::new();
    let mut buf: Packet = Packet::new(4 * 9);

    buf.p4(0);
    buf.p4(make_crc(&crc, "data/pack/client/title"));
    buf.p4(make_crc(&crc, "data/pack/client/config"));
    buf.p4(make_crc(&crc, "data/pack/client/interface"));
    buf.p4(make_crc(&crc, "data/pack/client/media"));
    buf.p4(make_crc(&crc, "data/pack/client/models"));
    buf.p4(make_crc(&crc, "data/pack/client/textures"));
    buf.p4(make_crc(&crc, "data/pack/client/wordenc"));
    buf.p4(make_crc(&crc, "data/pack/client/sounds"));
    return buf.data;
}

fn make_crc(crc: &CRC, path: &str) -> i32 {
    let buf: Packet = Packet::io(path.to_string());
    return crc.getcrc(&buf.data, 0, buf.data.len());
}

async fn rs2cgi(
    plugin: String,
    lowmem: String,
    nodeid: String,
    portoff: String,
    members: bool,
) -> Result<impl Reply, Rejection> {
    let client: Clients = match plugin.as_str() {
        "3" => Clients::JavaUnsigned(JavaUnsignedClient {
            plugin,
            nodeid,
            portoff,
            lowmem: lowmem == "1",
            members,
        }),
        "2" => Clients::Java(JavaClient {
            plugin,
            nodeid,
            portoff,
            lowmem: lowmem == "1",
            members,
        }),
        "1" => Clients::TeaVM(TeaVMClient {
            plugin,
            nodeid,
            portoff,
            lowmem: if lowmem == "1" {
                String::from("lowmem")
            } else {
                String::from("highmem")
            },
            members: members.to_string(),
        }),
        _ => Clients::TypeScript(TypeScriptClient {
            plugin,
            nodeid,
            portoff,
            lowmem,
            members: members.to_string(),
        }),
    };

    let html: String = match client {
        Clients::TypeScript(client) => client.render_once().unwrap(),
        Clients::JavaUnsigned(client) => client.render_once().unwrap(),
        Clients::Java(client) => client.render_once().unwrap(),
        Clients::TeaVM(client) => client.render_once().unwrap(),
    };

    return Ok(warp::reply::html(html));
}

async fn assets(path: warp::path::FullPath) -> Result<impl Reply, Rejection> {
    let path_str = path.as_str();

    let content_type: &str = match path_str {
        _ if path_str.ends_with(".js") => "application/javascript",
        _ if path_str.ends_with(".wasm") => "application/wasm",
        _ if path_str.ends_with(".sf2") => "application/octet-stream",
        _ if path_str.ends_with(".mjs") => "application/javascript",
        _ if path_str.ends_with(".ico") => "image/vnd.microsoft.icon",
        _ => return Err(warp::reject::not_found()),
    };

    let mut file: File = File::open(format!("public{}", path_str))
        .await
        .map_err(|_| warp::reject::not_found())?;

    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)
        .await
        .map_err(|_| warp::reject::not_found())?;

    return Ok(Response::builder()
        .header("Content-Type", content_type)
        .body(contents)
        .map_err(|_| warp::reject::not_found())?);
}

#[rustfmt::skip]
async fn cache(path: warp::path::FullPath) -> Result<impl Reply, Rejection> {
    let path_str = path.as_str();

    let bytes: Vec<u8> = match path_str {
        _ if path_str.starts_with("/title") => Packet::io("data/pack/client/title".to_string()).data,
        _ if path_str.starts_with("/config") => Packet::io("data/pack/client/config".to_string()).data,
        _ if path_str.starts_with("/interface") => Packet::io("data/pack/client/interface".to_string()).data,
        _ if path_str.starts_with("/media") => Packet::io("data/pack/client/media".to_string()).data,
        _ if path_str.starts_with("/models") => Packet::io("data/pack/client/models".to_string()).data,
        _ if path_str.starts_with("/textures") => Packet::io("data/pack/client/textures".to_string()).data,
        _ if path_str.starts_with("/wordenc") => Packet::io("data/pack/client/wordenc".to_string()).data,
        _ if path_str.starts_with("/sounds") => Packet::io("data/pack/client/sounds".to_string()).data,
        _ if path_str.starts_with("/crc") => crcs(),
        _ => return Err(warp::reject::not_found()),
    };

    return Ok(bytes);
}
