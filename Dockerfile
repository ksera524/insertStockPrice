# Rustの公式イメージをベースにする
FROM rust:latest as builder

# ビルド引数
ARG DATABASE_URL
ARG SPREADSHEET_URL
ARG SPREADSHEET_PASSWORD

ENV SQLX_OFFLINE=true

# 作業ディレクトリを設定
WORKDIR /usr/src/myapp

# ソースコードをコンテナにコピー
COPY . .


# アプリケーションのビルド
RUN cargo build --release

# 実行段階
FROM debian:buster-slim
COPY --from=builder /usr/src/myapp/target/release/insertStockPrice /usr/local/bin/insertStockPrice

# 実行時の環境変数を設定
ENV DATABASE_URL=${DATABASE_URL}
ENV SPREADSHEET_URL=${SPREADSHEET_URL}
ENV SPREADSHEET_PASSWORD=${SPREADSHEET_PASSWORD}

# コンテナ起動時にアプリケーションを実行
CMD ["insertStockPrice"]