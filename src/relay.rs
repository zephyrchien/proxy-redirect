use std::sync::Arc;
use std::io::{Result, Error, ErrorKind};
use futures::try_join;

use tokio::io::{AsyncWriteExt, copy};
use tokio::net::{TcpStream, TcpListener};

use super::Config;
use crate::rules;

pub async fn run(conf: Arc<Config>) {
    let lis = TcpListener::bind(conf.listen.clone()).await.unwrap();
    loop {
        if let Ok((src, _)) = lis.accept().await {
            tokio::spawn(choose_dst(src, conf.clone()));
        }
    }
}

async fn choose_dst(src: TcpStream, conf: Arc<Config>) -> Result<()> {
    let mut buf = vec![0; 64];
    let n = src.peek(&mut buf).await?;
    if n == 0 {
        return Err(Error::new(ErrorKind::UnexpectedEof, "eof"));
    }

    let dst_addr = if rules::is_socks5(&buf) {
        conf.socks5.to_string()
    } else if rules::is_http(&buf) {
        conf.http.to_string()
    } else {
        conf.default.to_string()
    };
    let dst = TcpStream::connect(dst_addr).await?;

    relay(src, dst).await
}

async fn relay(mut x: TcpStream, mut y: TcpStream) -> Result<()> {
    let (mut xr, mut xw) = x.split();
    let (mut yr, mut yw) = y.split();
    let forward = async {
        copy(&mut xr, &mut yw).await?;
        yw.shutdown().await
    };
    let reverse = async {
        copy(&mut yr, &mut xw).await?;
        xw.shutdown().await
    };

    let res = try_join!(forward, reverse);
    res.map(|_| ())
}
