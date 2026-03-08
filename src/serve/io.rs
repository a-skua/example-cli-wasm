use std::io::{Read, Write};
use std::net::TcpStream;
use wstd::http::{Body, Request, Response};

/// TcpStream から HTTP/1.1 リクエストをパースする
pub async fn parse_request(stream: &TcpStream) -> anyhow::Result<Request<Body>> {
    let mut reader = stream;
    let mut buf = Vec::new();
    let mut tmp = [0u8; 1024];

    // ヘッダー末尾（\r\n\r\n）まで読み込む
    loop {
        let n = reader.read(&mut tmp)?;
        if n == 0 {
            anyhow::bail!("connection closed before headers completed");
        }
        buf.extend_from_slice(&tmp[..n]);
        if buf.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }

    // ヘッダーとボディの境界を特定
    let header_end = buf.windows(4).position(|w| w == b"\r\n\r\n").unwrap();
    let header_bytes = &buf[..header_end];
    let remaining = &buf[header_end + 4..];

    let header_str = std::str::from_utf8(header_bytes)?;
    let mut lines = header_str.split("\r\n");

    // リクエストライン: "GET /path HTTP/1.1"
    let request_line = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing request line"))?;
    let mut parts = request_line.split_whitespace();
    let method = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing method"))?;
    let uri = parts.next().ok_or_else(|| anyhow::anyhow!("missing URI"))?;

    let mut builder = Request::builder().method(method).uri(uri);

    // ヘッダーをパース
    let mut content_length: usize = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some((name, value)) = line.split_once(':') {
            let value = value.trim();
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.parse().unwrap_or(0);
            }
            builder = builder.header(name, value);
        }
    }

    // Content-Length に基づいてボディを読み込む
    let body = if content_length > 0 {
        let mut body_buf = remaining.to_vec();
        while body_buf.len() < content_length {
            let n = reader.read(&mut tmp)?;
            if n == 0 {
                break;
            }
            body_buf.extend_from_slice(&tmp[..n]);
        }
        body_buf.truncate(content_length);
        Body::from(body_buf)
    } else {
        Body::empty()
    };

    Ok(builder.body(body)?)
}

/// HTTP/1.1 レスポンスを TcpStream に書き込む
pub async fn write_response(stream: &TcpStream, response: Response<Body>) -> anyhow::Result<()> {
    let mut writer = stream;
    let (parts, mut body) = response.into_parts();

    let body_bytes = body.contents().await?;

    // ステータスライン
    let status = parts.status;
    let reason = status.canonical_reason().unwrap_or("OK");
    let status_line = format!("HTTP/1.1 {} {}\r\n", status.as_u16(), reason);
    writer.write_all(status_line.as_bytes())?;

    // ヘッダー
    for (name, value) in &parts.headers {
        let header_line = format!("{}: {}\r\n", name, value.to_str().unwrap_or(""));
        writer.write_all(header_line.as_bytes())?;
    }

    // Content-Length（元のヘッダーに無い場合）
    if !parts.headers.contains_key("content-length") {
        let cl = format!("Content-Length: {}\r\n", body_bytes.len());
        writer.write_all(cl.as_bytes())?;
    }

    // ヘッダー終端
    writer.write_all(b"\r\n")?;

    // ボディ
    writer.write_all(body_bytes)?;
    writer.flush()?;

    Ok(())
}
