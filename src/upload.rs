use crate::util::*;
use actix_web::{web, HttpResponse, Result};
use futures::StreamExt;
use regex::Regex;
use std::{fs, process::Command};
use zip::read::ZipArchive;

#[post("/upload")]
async fn upload(mut payload: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let mut body = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk.map_err(|e| {
            println!("Error while reading chunk: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;
        body.extend_from_slice(&chunk);
    }

    let filename = "received.zip";
    let filepath = format!("./tmp/{}", filename);

    fs::write(&filepath, body).map_err(|e| {
        println!("Failed to write to file: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    // Extracting the zip contents
    let file = fs::File::open(&filepath).map_err(|e| {
        println!("Failed to open the saved zip file: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let mut archive = ZipArchive::new(file).map_err(|e| {
        println!("Failed to create ZIP archive: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| {
            println!("Failed to read file from ZIP archive: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

        let outpath = format!("./output/{}", file.name());

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| {
                println!("Failed to create directory for extraction: {}", e);
                actix_web::error::ErrorInternalServerError(e)
            })?;
        } else {
            let mut outfile = fs::File::create(&outpath).map_err(|e| {
                println!("Failed to create output file for extraction: {}", e);
                actix_web::error::ErrorInternalServerError(e)
            })?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| {
                println!("Failed during file extraction process: {}", e);
                actix_web::error::ErrorInternalServerError(e)
            })?;
        }
    }

    println!("Upload and processing successful");

    match Command::new("python3")
        .arg("./src/python/generateCairo.py")
        .output()
    {
        Ok(output) => {
            let output_log = String::from_utf8_lossy(&output.stdout);
            println!("python cairo script success ✅ {}", output_log);
        }
        Err(e) => {
            println!("python cairo script fail ❌ {}", e);
        }
    }

    match Command::new("scarb")
        .arg("cairo-test")
        .arg("-f")
        .arg("mnist_nn_test")
        .output()
    {
        Ok(output) => {
            let output_log = String::from_utf8_lossy(&output.stdout);
            println!("scarb orion script success ✅ {}", output_log);

            Ok(ResponseType::Ok(output_log).get_response()).into()
        }
        Err(e) => {
            println!("scarb orion script fail ❌ {}", e);
            Ok(ResponseType::NotFound("scarb orion script fail").get_response()).into()
        }
    }

    // Ok(HttpResponse::Ok().into())
}
