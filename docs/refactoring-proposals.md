# リファクタリング提案

## 目的

この文書は、現在の外部 API と正常系の動作をできるだけ維持しながら、DYNAMIXEL Protocol 2.0 の受信処理を安全かつ拡張しやすくするための候補を整理したものです。単なるファイル分割ではなく、入力境界でのパニック防止、プロトコル上の責務分離、コントロールテーブル定義の一元化を優先します。

調査時点の規模は次のとおりです。

| モジュール | 行数 | 主な責務 |
| --- | ---: | --- |
| `src/control_table.rs` | 2,126 | レジスタ定義、型対応、アドレス・サイズ、読み書き API、テスト |
| `src/packet_handler.rs` | 1,352 | 受信、フレーミング、CRC、命令実行、応答生成、送信順制御、テスト |
| その他 6 モジュール | 245 | バッファ、公開 trait、補助型など |

ベースラインとして `cargo test` は 23 件すべて成功します。ただし、`cargo clippy --all-targets -- -D warnings` は 41 件の指摘により失敗します。既存テストは正常系の回帰検知には使えますが、不正長・容量上限・不正アドレスなどの異常系を十分に固定していません。

## 優先度一覧

| 優先度 | 対象 | 狙い | 主なリスク |
| --- | --- | --- | --- |
| P0 | パケット長とアドレス範囲の検証 | 外部入力による panic をなくす | 可用性、メモリ境界 |
| P0 | 命令の decode と実行を分離 | 命令ごとの検証漏れを防ぐ | 誤応答、誤書き込み |
| P1 | パケット codec の独立 | stuffing/CRC/フレーミングを単体検証可能にする | 複雑な状態遷移 |
| P1 | コントロールテーブル定義の一元化 | 約 1,600 行の反復と定義ずれを減らす | 保守性、定義不整合 |
| P1 | エラー型と非ブロッキング状態 API の整理 | 呼び出し側が待機・破損・範囲外を区別できるようにする | 障害解析性 |
| P1 | リングバッファの不変条件を型・テストで保証 | サイズ条件と wrap-around バグを除く | panic、データ欠落 |
| P2 | HAL 境界と公開 API の縮小 | 実装差し替えと互換性管理を容易にする | API の曖昧さ |
| P2 | テスト構成と品質ゲートの整理 | 変更を小さく安全に進める | 回帰の見逃し |
| P3 | 命名・未使用コード・小さな複製の整理 | 読みやすさとドキュメント品質を上げる | 認知負荷 |

## 提案詳細

### P0-1. 外部入力を、添字アクセスより前に検証する

`DynamixelProtocolHandler::parse_data` は、Read/Write/Sync Read/Sync Write のパラメータを直接添字アクセスし、その後でスライスを作っています。たとえば Read は `address..address + length`、Write は受信 length から `5` を減算します。不正または途中で切れたパケットでは、範囲外アクセス・整数 underflow・overflow により panic する可能性があります。

`receive_packet` にも次の境界問題があります。

- プロトコルの `length` のみを `MAX_PACKET_LEN` と比較しており、ヘッダを加えた実際の格納長を検証していない。
- 宣言長が現在の受信済み長より短い場合、`self.wait_length - self.msg.len()` が underflow し得る。
- `heapless::Vec::resize`、`push` の一部で `unwrap()` を使い、stuffing 後の容量増加も呼び出し側へ返せない。
- `BufferInterface::read_bytes` が返す長さが渡した slice の長さ以下であることを API で保証できず、`res[0..readlen]` が panic し得る。

まず、受信フレームを次の順で検証する関数を設けます。

1. 固定ヘッダと最小長を検証する。
2. `checked_add` / `checked_sub` で総フレーム長を計算する。
3. 総フレーム長が `MAX_PACKET_LEN` 以下であることを検証する。
4. CRC と stuffing を検証・解除する。
5. 命令ごとの最小パラメータ長を検証する。
6. Control Table の `address + length` を `checked_add` し、テーブル長以内か検証する。

不正入力は panic ではなく、`PacketError::InvalidLength`、`PacketError::CapacityExceeded`、`PacketError::AddressOutOfRange` などへ変換します。DYNAMIXEL の Status Packet を返すべきケースは、内部エラーと分けて `ErrorBit` へ対応付けます。

完了条件:

- 任意の byte 列を入力しても panic しない。
- 長さ 0、最小長未満、`MAX_PACKET_LEN` 境界、末尾アドレス越えのテストがある。
- 容量不足を `unwrap()` ではなく結果型で観測できる。

### P0-2. 「decode」「命令実行」「応答方針」を分ける

現在の `parse_data` は、受信状態の更新、宛先判定、パラメータ decode、Control Table の更新、応答 packet の構築、他 ID の応答待ちを一つのメソッドで行います。この構造では命令ごとの規則が分散し、追加時に境界検証や broadcast 規則を入れ忘れやすくなります。

次の内部表現への decode を提案します。

```rust
enum DecodedInstruction<'a> {
    Ping,
    Read { address: u16, length: u16 },
    Write { address: u16, data: &'a [u8] },
    SyncRead { address: u16, length: u16, ids: &'a [u8] },
    SyncWrite { address: u16, item_size: u16, entries: &'a [u8] },
}
```

その上で、純粋に近い `execute(instruction, device_id, control_table)` が「更新内容」「応答要否」「Status Error」を返し、状態機械は送受信タイミングだけを担当します。

この分離時に、コード上で曖昧な次の挙動を仕様テストとして固定します。

- Sync Read の ID リストに自 ID がない場合にも現在は応答候補になる。
- broadcast Write に対して現在は Status Packet を組み立てる。
- 未対応命令と不正パラメータのどちらも、呼び出し側からは実質的に区別しづらい。
- `return_delay_time` が 0 より大きい場合の分岐は未実装のまま送信へ進む。
- `status_return_level` が応答要否の判断に使われていない。

完了条件:

- 命令 decode のテストが UART や時計なしで書ける。
- 宛先・broadcast・Status Return Level・Return Delay Time の規則が表形式のテストで固定される。
- `parse_data` は状態遷移と I/O のオーケストレーションに限定される。

### P1-1. パケット codec を独立モジュールにする

CRC、byte stuffing、header 探索、Packet の位置定数、Status Packet 構築はプロトコル codec の責務です。これらを `packet_handler.rs` から、たとえば次の構成へ移します。

```text
src/protocol/
  mod.rs       # 公開しない共通型と定数
  decoder.rs   # byte stream -> validated frame
  encoder.rs   # response -> bounded byte vector
  crc.rs       # CRC-16
  stuffing.rs  # stuffing / unstuffing
```

`Packet::to_pos()` は実行時 enum より `const` オフセットの方が用途に合います。また、応答生成の `ping_response_packet`、`read_response_packet`、`write_response_packet` は共通の Status Packet builder に統合できます。CRC table は各呼び出しのローカル配列ではなく、モジュール `const` にします。

完了条件:

- encode/decode が I/O trait と Control Table に依存しない。
- stuffing により上限を超えるケースを明示的なエラーにする。
- 既存の packet byte 列テストを codec のテーブル駆動テストへ移す。

### P1-2. Control Table のメタデータを一元化する

`control_table.rs` では各フィールドの情報が、少なくとも次の場所へ反復されています。

- `ControlTable` の variant
- `CustomInt` の型対応
- `to_address` の match
- `to_size` の match
- `R` の getter
- `W` の getter
- 一部のフィールド別テスト

このため、フィールド追加時に複数箇所の同期が必要で、`VelocityPgain` / `velocity_pgain` のような表記差も入り込んでいます。単一の宣言から enum、アドレス、サイズ、型付き accessor を生成する `macro_rules!`、または小さな生成スクリプトへ寄せます。`no_std` crate なので、まずは追加依存のない宣言 macro が扱いやすい選択です。

フィールド定義には型・アドレス・アクセス属性・初期値を持たせます。

```rust
control_table! {
    model_number: u16 { address: 0, access: ReadOnly, reset: 0 },
    id: u8 { address: 7, access: ReadWrite, reset: 1 },
    // ...
}
```

これにより、現在すべて 0 で初期化される `ControlTableData::new`、任意範囲を書ける `W::bytes`、未使用の `data_spec.rs` を一つの設計に統合できます。EEPROM/RAM、read-only、値域などをどこまで強制するかは、先に既存利用側との互換性を確認します。

また、`R::bits()` と `Cell<[u8; 231]>` は読み書きのたびにテーブル全体を値コピーします。組み込み環境での stack 使用量と実行時間を計測し、必要ならスナップショット API と範囲読み出し API を分けます。割り込みと main loop から同時に触る設計なら、`Cell` による内部可変性だけでは同期にならないため、所有権または critical section の方針も明文化します。

完了条件:

- フィールドの型・アドレス・サイズを一箇所だけで宣言する。
- 全フィールドについて「範囲内」「重複なし（予約領域を除く）」「型サイズ一致」を自動テストする。
- raw read/write と型付き read/write の責務を分ける。

### P1-3. エラー型と poll 結果を用途別にする

公開 API の `parse_data() -> Result<(), ()>` と `QueueInterface::enqueue() -> Result<(), ()>` は、原因を呼び出し側へ伝えられません。一方、内部の `CommunicationResult` は通信結果、進行中、タイムアウト、未対応、包括的な失敗を一つに含み、状態とエラーが混在しています。

非ブロッキング API として、たとえば次を分けます。

```rust
enum PollOutcome {
    Idle,
    Receiving,
    ResponseQueued,
    ResponseSent,
}

enum ProtocolError {
    InvalidFrame(PacketError),
    InvalidInstruction(InstructionError),
    Transport(TransportError),
}
```

`CommunicationResult` の文字列表現を維持する必要があれば、互換層で変換します。`packet_return_time()` は現状更新箇所が見当たらず常に初期値なので、実際に deadline を公開するか、未実装 API を削るかを決めます。

完了条件:

- 「まだ受信中」は `Err` ではない。
- ログなしでも失敗原因を match できる。
- 公開エラー型に `core::fmt::Display` を実装し、必要なら `#[non_exhaustive]` を付ける。

### P1-4. RingBuffer のサイズ制約と wrap-around を修正する

`RingBuffer` の index 更新は `(index + 1) & (BUFFER_SIZE - 1)` を使うため、`BUFFER_SIZE` が 2 の累乗であることを前提にしています。しかし、その制約は型にも constructor にも表現されていません。さらに満杯判定の `self.wp_ - self.rp_` は `wp_ < rp_` のとき debug build で underflow し得ます。サイズ 0 や 1 も有効な型として生成でき、`Default`、`len`、`capacity` もありません。

次のどちらかへ変更します。

- 任意サイズ対応: 次 index を `% BUFFER_SIZE` で計算し、`len` を保持する。
- 2 の累乗限定: constructor または const assertion で条件を強制し、差分は `wrapping_sub` ではなく明示した状態で管理する。

用途が UART の SPSC queue だけであれば、独自実装を維持する必要性と `heapless::spsc::Queue` の採用も比較します。

完了条件:

- wrap-around 後の満杯・空判定をテストする。
- サイズ 0、1、非 2 冪の扱いが compile error または仕様化された結果になる。
- `enqueue` の失敗で格納済みデータが変化しない。

### P2-1. I/O trait を最小化し、命名を実態に合わせる

`BufferInterface` は UART を想定した名前と、byte 単位・slice 単位の重複 API を持ちます。実装側が「データなし」と「I/O エラー」を区別できず、`read_bytes` の返却長契約も型に現れません。`QueueInterface` は現状の crate 内で利用箇所が見当たりません。

内部処理が実際に必要とする操作へ絞り、たとえば `Transport` の `read(&mut [u8]) -> Result<usize, E>` と `write_all(&[u8]) -> Result<(), E>` にします。組み込み HAL を直接固定せず、関連型 `type Error` で実装固有エラーを受け取れる形が適しています。移行時は既存 `BufferInterface` の blanket adapter を用意すると破壊的変更を分離できます。

あわせて、`DynamixelProtocolHandler` の `uart` と `ctd` が public fieldである必要性を見直し、必要な accessor のみ公開します。

### P2-2. テストを責務別に分け、異常系を増やす

`packet_handler.rs` の約半分は同一モジュール内の mock とテストです。実装の分割に合わせて次の層へ整理します。

- unit: CRC、stuffing、長さ計算、命令 decode、Control Table field。
- state-machine: byte を分割投入したときの状態遷移と deadline。
- integration: mock transport を通した Ping/Read/Write/Sync 系の end-to-end。
- robustness: 任意 byte 列、不正 CRC、不正長、容量境界、範囲外アドレス。

特に `read_large_data` は期待結果がコメントアウトされており、panic しないこと以外の保証が弱い状態です。テスト名に意図を反映し、期待する Status Error または内部 error を assert します。`buffer::tests::enqueue` の `1 + 1 == 2` のように実装を検証しないテストは置き換えます。

CI の完了条件は、段階的に以下へ揃えます。

```text
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo doc --no-deps
```

### P3-1. 公開名と dead code を整理する

以下は機能変更と分離して、小さな互換性 PR で扱います。

- `ExtendedPosionControlMode` → `ExtendedPositionControlMode`
- `PWMControMode` → `PwmControlMode`
- `ProfileAccleration` / `profile_accleration` → `ProfileAcceleration` / `profile_acceleration`
- `VelocityPgain` → `VelocityPGain` など、gain 名の規則を統一
- `baudrate: baudrate` → field init shorthand
- `is_empty` の if 式 → 比較式を直接返す
- `#![allow(unused_imports)]` と広い `#[allow(dead_code)]` を外し、必要箇所だけ理由付きで許可
- コメントアウトされた trait、テスト、フィールドを issue または履歴へ移し、ソースから除去
- `new` を持つ型へ `Default` を実装

公開名の修正は既存利用者を壊すため、旧名を deprecated alias として一リリース残すか、次の breaking release でまとめて変更します。

## 推奨する実施順

### Phase 1: 振る舞いの固定と panic 除去

1. 異常長、範囲外 Read/Write、Sync Read の非対象 ID、broadcast、return delay の characterization test を追加する。
2. checked arithmetic と範囲検証を追加する。
3. 容量エラーを返せる内部エラー型を導入する。

この段階では大きなファイル分割をせず、安全性の差分をレビューしやすくします。

### Phase 2: codec と命令実行の抽出

1. CRC と stuffing を純粋関数として抽出する。
2. frame decoder / encoder を抽出する。
3. `DecodedInstruction` と命令 executor を抽出する。
4. handler を状態遷移と I/O に限定する。

既存 end-to-end テストの byte 列が変わらないことを移行条件にします。

### Phase 3: Control Table の単一定義化

1. 現行フィールドを網羅するメタデータテストを先に作る。
2. 5～10 フィールドで macro の形を検証する。
3. 全フィールドを移行し、手書きの重複定義を削除する。
4. アクセス属性、reset value、範囲制約を必要なものから追加する。

生成結果を一度に全面置換するより、既存 API と byte 配置の一致を各段階で確認します。

### Phase 4: 公開 API と品質ゲート

1. transport/error/poll API を互換層付きで導入する。
2. public field と不要な re-export を整理する。
3. deprecated alias を経由して命名を修正する。
4. Clippy の既存指摘を解消し、CI で warning を拒否する。

## 最初の変更単位として推奨する PR

最初の PR は「不正パケットで panic しない」に限定するのが安全です。

- `PacketError` と checked length helper を追加。
- Read/Write/Sync Read/Sync Write の最小長と Control Table 範囲を検証。
- `receive_packet` の総 packet 長を `MAX_PACKET_LEN` 以下に制限。
- `heapless::Vec` の容量エラーを伝播。
- 正常系 packet の出力が既存テストと一致することを確認。
- 境界値と不正入力のテーブル駆動テストを追加。

これにより、後続の責務分割前に安全な土台を作れます。Control Table の macro 化や公開 API 変更は差分が大きいため、同じ PR へ混ぜない方がレビューと切り戻しが容易です。

## 実装前に決める事項

- 対象モデルは XC330 固定か、モデルごとに Control Table を差し替えるか。
- EEPROM/RAM、read-only、値域制約をライブラリが強制するか、アプリケーションへ委ねるか。
- handler は Status Packet を即時送信するか、送信 deadline と packet を呼び出し側へ返すか。
- main loop と割り込みのどちらが Control Table を所有・更新するか。
- 既存公開 API の互換性を何リリース維持するか。

これらは設計の分岐点ですが、P0 の境界検証と panic 除去はどの選択でも先行できます。
