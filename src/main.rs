use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use qrcode::QrCode;
use image::{Luma, DynamicImage};
use std::io::Cursor;

#[derive(serde::Deserialize)]
struct QrRequest {
    text: String,
}

// QR kodu oluşturma ve PNG olarak döndürme fonksiyonu
async fn generate_qr(data: web::Json<QrRequest>) -> impl Responder {
    // QR kodunu oluştur
    let qr = match QrCode::new(data.text.as_bytes()) {
        Ok(code) => code,
        Err(_) => return HttpResponse::BadRequest().body("QR kodu oluşturulamadı."),
    };

    // QR kodunu görüntüye dönüştür
    let image = qr.render::<Luma<u8>>().build();
    let dynamic_image = DynamicImage::ImageLuma8(image);

    // PNG formatına dönüştür ve byte dizisi olarak sakla
    let mut png_bytes = Cursor::new(Vec::new());
    if let Err(_) = dynamic_image.write_to(&mut png_bytes, image::ImageFormat::Png) {
        return HttpResponse::InternalServerError().body("PNG oluşturulamadı.");
    }

    // Byte dizisini yanıt olarak döndür
    HttpResponse::Ok()
        .content_type("image/png")
        .body(png_bytes.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HTTP sunucusunu başlat
    HttpServer::new(|| {
        App::new()
            .route("/generate", web::post().to(generate_qr)) // QR kodu oluşturma rotası
            .service(
                actix_files::Files::new("/", "./static") // Statik dosyalar için servis
                    .index_file("index.html"),
            )
    })
    .bind("127.0.0.1:8080")? // Sunucuyu localhost:8080 üzerinde çalıştır
    .run()
    .await
}
