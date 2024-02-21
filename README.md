# rstest-example

paild社のテックブログで使用したサンプルコードを掲載しています。下記のコードが収録されています。

- `async.rs`: `tokio::test`を織り交ぜたコードです。
- `basic.rs`: 最も基本的なパラメータ化テストでのrstestの使い方を示したコードです。
- `combination.rs`: rstestのValue Listという機能の利用例を示したものです。
- `fixture.rs`: rstestの`#[fixture]`の使い方を示したものです。データベース接続を模倣した構造体をFixtureとして各テストに配る想定のコードです。
- `parse_json.rs`: rstestのファイルパスを渡す機能を示したものです。`files`というディレクトリ配下のJSONファイルをひとつずつ読み、テストケースとして利用します。
