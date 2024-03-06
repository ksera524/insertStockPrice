# Rustの公式イメージをベースにする
FROM rust:1.75.0 as builder

# 作業ディレクトリを設定
WORKDIR /usr/src/myapp

# ソースコードをコンテナにコピー
COPY . .

# アプリケーションのビルド
RUN cargo build --release

# 実行段階
FROM ubuntu:latest

# 必要なライブラリをインストール（libsslの適切なバージョンをインストール）
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# 実行ファイルをコピー
COPY --from=builder /usr/src/myapp/target/release/insertStockPrice /usr/local/bin/insertStockPrice

# コンテナ起動時にアプリケーションを実行
CMD ["insertStockPrice"]
