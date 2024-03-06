# Rustの公式イメージをベースにする
FROM rust:latest as builder

# 作業ディレクトリを設定
WORKDIR /usr/src/myapp

# ビルド引数
ARG DATABASE_URL
ARG SPREADSHEET_URL
ARG SPREADSHEET_PASSWORD

# ソースコードをコンテナにコピー
COPY . .

# アプリケーションのビルド
RUN cargo build --release

# 実行段階
FROM debian:bullseye-slim

# 必要なライブラリをインストール（libsslのバージョンを更新）
RUN apt-get update && apt-get install -y ca-certificates libssl1.1 && rm -rf /var/lib/apt/lists/*

# 実行ファイルをコピー
COPY --from=builder /usr/src/myapp/target/release/insertStockPrice /usr/local/bin/insertStockPrice

# 実行時の環境変数を設定
ENV DATABASE_URL=${DATABASE_URL}
ENV SPREADSHEET_URL=${SPREADSHEET_URL}
ENV SPREADSHEET_PASSWORD=${SPREADSHEET_PASSWORD}

# コンテナ起動時にアプリケーションを実行
CMD ["insertStockPrice"]
