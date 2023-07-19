use {

    actix_multipart::Multipart,

    futures::stream::StreamExt,

    actix_web::{web, HttpResponse, Result},
    std::fs,
    std::io::Write,
    zip::read::ZipArchive,

};

#[post("/upload")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse> {
    println!("processing zip");
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field
            .content_disposition()
            .ok_or(actix_web::error::ParseError::Incomplete)?;

        let filename = content_disposition.get_filename().unwrap_or("default");
        let filepath = format!("./tmp/{}", filename);

        let filepath_clone = filepath.clone();
        let mut f = web::block(move || std::fs::File::create(&filepath_clone))
            .await
            .unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }

        // Extract the zip contents
        let file = fs::File::open(&filepath).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = format!("./output/{}", file.name());

            if (&*file.name()).ends_with('/') {
                fs::create_dir_all(&outpath).unwrap();
            } else {
                let mut outfile = fs::File::create(&outpath).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }
        }
    }
    println!("processing success");
    Ok(HttpResponse::Ok().into())
}