# Tauri GUI Migration Plan for Social CLI

## 概要

このプランは、既存のsocial-cli（Rust製CLI）をTauri GUIアプリケーション（macOS + iOS対応）に移行するための実装計画です。

**現状:**
- CLI（clap使用）
- 3つのプラットフォームモジュール（bluesky.rs, x.rs, threads.rs）
- .envファイルで認証情報管理
- 順次投稿（エラー分離あり）

**目標:**
- Tauri 2.0 GUIアプリ
- macOS + iOS対応
- Keychain認証（.env廃止）
- 同じ投稿機能をGUIで実現
- 開発者モードのみ（App Store配布なし）

---

## 1. 技術スタック

### バックエンド

Rust

**追加する依存関係:**
- tauri
- serde
- keyring
- tauri-plugin-shell

**既存の依存関係を維持:**
- tokio
- reqwest
- atrium-api
- atrium-xrpc-client
- serde_json
- anyhow
- reqwest-oauth1

**削除する依存関係:**
- clap - GUI化により不要
- dotenvy - Keychainに置き換え
- tempfile - エディタモード不要

### フロントエンド

React + TypeScript

理由:
- 既存スキルを活用可能
- 豊富なエコシステムとコミュニティサポート
- 個人用途ではバンドルサイズの差は許容範囲

**UI: Base UI + Tailwind CSS**

- **Base UI** ([base-ui.com](https://base-ui.com/)): Material UIチーム製ヘッドレスコンポーネント、WAI-ARIA完全準拠
- **Tailwind CSS**: モバイルレスポンシブ、iOS/macOS風カスタムスタイリング

**ルーティング**: 
- react-router **不要**
- `useState`で状態ベースの画面切り替え（画面数が少ないため）

---

## 2. プロジェクト構成

```
social-cli-tauri/
├── src-tauri/                    # Rustバックエンド
│   ├── src/
│   │   ├── main.rs               # Tauriエントリーポイント
│   │   ├── commands.rs           # Tauriコマンド定義
│   │   ├── credentials.rs        # Keychain統合
│   │   └── platforms/            # プラットフォームモジュール
│   │       ├── mod.rs
│   │       ├── bluesky.rs        # 既存コードを微修正
│   │       ├── x.rs
│   │       └── threads.rs
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── icons/
│
├── src/                          # フロントエンド
│   ├── main.tsx
│   ├── App.tsx
│   ├── lib/
│   │   ├── api.ts                # Tauriコマンドラッパー
│   │   └── types.ts
│   └── components/
│       ├── PostForm.tsx          # 投稿フォーム（結果表示含む）
│       └── CredentialsForm.tsx
│
├── package.json
├── tsconfig.json
├── vite.config.ts
└── tailwind.config.js
```

---

## 3. 主要な実装ポイント

### 3.1 認証情報管理（Keychain）

**使用クレート**:
- keyring

**実装方針**:
```rust
// src-tauri/src/credentials.rs
use keyring::Entry;

const SERVICE_NAME: &str = "com.social-cli.credentials";

// 基本操作
fn get_credential(key: &str) -> Result<String> {
    let entry = Entry::new(SERVICE_NAME, key)?;
    entry.get_password()
}

fn set_credential(key: &str, value: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, key)?;
    entry.set_password(value)
}

// PlatformCredentials構造体
pub struct PlatformCredentials {
    pub bluesky_identifier: String,
    pub bluesky_app_password: String,
    pub x_consumer_key: String,
    pub x_consumer_secret: String,
    pub x_access_token: String,
    pub x_access_token_secret: String,
    pub threads_user_id: String,
    pub threads_access_token: String,
}

impl PlatformCredentials {
    pub fn load() -> Result<Self>
    pub fn save(&self) -> Result<()>
}
```

**セキュリティ**:
- macOS: Keychain暗号化、バンドルIDで隔離
- iOS: Secure Enclave暗号化、ハードウェアセキュリティ

**参考**: [keyring公式ドキュメント](https://docs.rs/keyring)

### 3.2 プラットフォームモジュールの適応

**変更内容**:
```rust
// 変更前
pub async fn post(message: &str) -> Result<String> {
    let identifier = env::var("BLUESKY_IDENTIFIER")?;
    // ...
}

// 変更後: 認証情報を引数で受け取る
pub async fn post(message: &str, identifier: &str, password: &str) -> Result<String> {
    // env::var()削除、引数を使用
    // それ以外のロジックは変更なし
}
```

**適用ファイル**: bluesky.rs, x.rs, threads.rs
**再利用率**: 約90%（env::var()部分のみ変更）

### 3.3 Tauriコマンド層

**役割**: フロントエンド（JavaScript）⇔ バックエンド（Rust）の橋渡し

**実装するコマンド** (`src-tauri/src/commands.rs`):
```rust
#[derive(serde::Serialize)]
pub struct PostResult {
    pub platform: String,
    pub success: bool,
    pub url: Option<String>,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn post_to_bluesky(message: String) -> Result<PostResult, String>

#[tauri::command]
pub async fn post_to_x(message: String) -> Result<PostResult, String>

#[tauri::command]
pub async fn post_to_threads(message: String) -> Result<PostResult, String>

#[tauri::command]
pub async fn post_to_all(message: String) -> Vec<PostResult>

#[tauri::command]
pub async fn save_credentials(creds: PlatformCredentials) -> Result<(), String>

#[tauri::command]
pub async fn load_credentials() -> Result<PlatformCredentials, String>

#[tauri::command]
pub async fn check_credentials_exist() -> bool
```

**main.rsでの登録**:
```rust
// src-tauri/src/main.rs
mod commands;
mod credentials;
mod platforms;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::post_to_bluesky,
            commands::post_to_x,
            commands::post_to_threads,
            commands::post_to_all,
            commands::save_credentials,
            commands::load_credentials,
            commands::check_credentials_exist,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3.4 フロントエンド実装

**画面構成**:
1. **App.tsx**: 状態管理（loading → setup or main）
2. **CredentialsForm.tsx**: 認証情報入力フォーム
3. **PostForm.tsx**: 投稿フォーム（textarea + プラットフォーム選択 + 投稿ボタン + 結果表示）

**API層** (`src/lib/api.ts`):
```typescript
import { invoke } from '@tauri-apps/api/core';

export interface PostResult {
  platform: string;
  success: boolean;
  url?: string;
  error?: string;
}

export interface PlatformCredentials {
  bluesky_identifier: string;
  bluesky_app_password: string;
  // ... 他の認証情報
}

export async function postToAll(message: string): Promise<PostResult[]> {
  return await invoke('post_to_all', { message });
}

export async function saveCredentials(creds: PlatformCredentials): Promise<void> {
  await invoke('save_credentials', { creds });
}

export async function checkCredentials(): Promise<boolean> {
  return await invoke('check_credentials_exist');
}
```

**状態管理**:
```tsx
// src/App.tsx
type Screen = 'loading' | 'setup' | 'main' | 'settings';
const [screen, setScreen] = useState<Screen>('loading');

// 起動時に認証情報チェック
useEffect(() => {
  checkCredentials().then(hasCredentials => {
    setScreen(hasCredentials ? 'main' : 'setup');
  });
}, []);
```

**使用コンポーネント**:
- Base UI: `Button`, `Select`, `Dialog`, `TextField`
- Tailwind CSS: レスポンシブスタイリング

**参考**:
- [Base UI公式](https://base-ui.com/)
- [Tauri API](https://v2.tauri.app/develop/)

### 3.5 iOS対応の考慮点

**1. タッチファーストUI**:
- タップターゲット最小44x44pt
- ボトムナビゲーション（親指に優しい配置）
- ネイティブキーボード統合

**2. ビルド設定** (`src-tauri/tauri.conf.json`):
```json
{
  "bundle": {
    "iOS": {
      "minimumSystemVersion": "13.0",
      "developmentTeam": "YOUR_TEAM_ID"
    }
  }
}
```

**3. UIの適応**:
```tsx
// カスタムフック
function useIsMobile() {
  const [isMobile, setIsMobile] = useState(false);
  useEffect(() => {
    platform().then(p => setIsMobile(p === 'ios'));
  }, []);
  return isMobile;
}

// コンポーネントで使用
const isMobile = useIsMobile();
<div className={isMobile ? 'text-lg' : 'text-base'}>
```

**4. Tauri権限設定** (`src-tauri/capabilities/default.json`):
```json
{
  "permissions": [
    "core:default",
    "shell:allow-open",
    "http:default"
  ]
}
```

---

## 4. 実装フェーズ

### Phase 1: プロジェクトセットアップ

**タスク**:
1. Tauriプロジェクト初期化
   ```bash
   pnpm create tauri-app
   # React + TypeScriptを選択
   ```
2. iOS設定: `cargo tauri ios init`
3. 依存関係追加（keyring, tauri-plugin-shell）
4. macOS動作確認: `pnpm tauri dev`
5. iOSシミュレータ確認: `pnpm tauri ios dev`

**検証**:
- [ ] macOSアプリがエラーなく起動
- [ ] iOSシミュレータでアプリ表示
- [ ] ホットリロード動作

### Phase 2: バックエンド移行

**タスク**:
1. `src-tauri/src/platforms/` ディレクトリ作成
2. bluesky.rs, x.rs, threads.rsをコピー
3. 関数シグネチャ更新（認証情報を引数化）
4. `env::var()` 呼び出し削除
5. `credentials.rs` 作成（Keychain統合）
6. `commands.rs` 作成（Tauriコマンド）
7. `main.rs` でコマンド登録

**検証**:
- [ ] Tauriコマンド経由で認証情報保存可能
- [ ] Keychainから認証情報読み込み可能
- [ ] 各プラットフォームへの投稿成功
- [ ] macOS Keychain Accessで認証情報確認可能

### Phase 3: フロントエンド開発

**タスク**:
1. フロントエンド依存関係インストール
   ```bash
   pnpm add @tauri-apps/api @base-ui/react
   pnpm add -D tailwindcss autoprefixer postcss
   ```
2. APIラッパー作成（`src/lib/api.ts`）
3. コンポーネント構築（CredentialsForm.tsx, PostForm.tsx）
4. App.tsx フロー作成（状態ベース画面切り替え）
5. Tailwindスタイリング

**検証**:
- [ ] 認証情報の入力と保存可能
- [ ] 全プラットフォームへメッセージ投稿可能
- [ ] 単一プラットフォームへ投稿可能
- [ ] 成功時にクリック可能なURL表示
- [ ] エラーが明確に表示

### Phase 4: iOS最適化

**タスク**:
1. iOSシミュレータでテスト
2. モバイルUI調整（タッチターゲット、キーボード、ナビゲーション）
3. iOS固有設定（Bundle ID、開発チーム署名、最小iOS 13.0）
4. 実機テスト: `cargo tauri ios dev --device`
5. iOS固有動作対応（バックグラウンド化、ダークモード）
6. アプリアイコン追加（1024x1024 + 各種サイズ）

**検証**:
- [ ] 実機iPhoneにアプリインストール
- [ ] iOSで認証情報保存可能
- [ ] iOSデバイスから投稿可能
- [ ] キーボードが入力欄を隠さない
- [ ] 縦横両方の向きで動作
- [ ] ダークモードが正しく表示

### Phase 5: 仕上げ・テスト

**タスク**:
1. エラーハンドリング改善（ネットワークタイムアウト、無効認証情報、APIレート制限）
2. UX改善（ローディング状態、成功アニメーション、文字数カウンター）
3. テスト（単体テスト、統合テスト、手動テストチェックリスト）
4. ドキュメント更新（README、ユーザーガイド、トラブルシューティング）
5. ビルド最適化: `cargo tauri build` / `cargo tauri ios build --release`

**検証**:
- [ ] すべてのエラーケースを適切に処理
- [ ] ローディング状態が二重投稿を防止
- [ ] 文字数制限を強制
- [ ] macOS .dmgビルド成功
- [ ] iOS .ipaビルド成功
- [ ] ドキュメント完成

---

## 5. 想定される課題と解決策

### 課題1: 初回起動時のKeychain設定
- ガイド付きオンボーディングウィザード
- 部分的な認証情報設定を許可（一部プラットフォームはオプション）
- 入力時の認証情報テスト（即座に検証）

### 課題2: iOS開発環境
- 開発ビルドのみ（App Store提出なし）
- Xcodeで自動署名を使用
- 無料Apple Developerアカウントで十分

### 課題3: 非同期処理
- 現状で許容可能（既存CLIも順次実行）
- 将来的に並列化可能（`tokio::join!`使用）

### 課題4: 既存ユーザーの移行
- 手動で認証情報を再入力（最もシンプル）
- .envファイルを参照しながらGUIで入力
- 移行完了後、.envファイルは削除推奨

---

## 6. ビルド・デプロイワークフロー

### 開発ワークフロー

**macOS**:
```bash
pnpm tauri dev  # ホットリロード有効
```

**iOS**:
```bash
pnpm tauri ios dev          # シミュレータ
pnpm tauri ios dev --device # 実機
```

### 本番ビルド

**macOS**:
```bash
pnpm tauri build
# 出力: src-tauri/target/release/bundle/macos/Social CLI.app
# .dmgインストーラーも作成
```

**iOS**:
```bash
pnpm tauri ios build --release
# Xcodeプロジェクトが開く → Product → Archive
# 出力: .ipaファイル
```

### 配布（開発モード）

**macOS**: .dmgファイルを共有、Applicationsフォルダにドラッグ  
**iOS**: Xcode/Apple Configurator経由でインストール（年間最大100デバイス）

---

## 7. 重要ファイル

実装の核となるファイル:

1. **src-tauri/src/commands.rs** - Tauriコマンド定義（フロントエンド⇔バックエンド統合）
2. **src-tauri/src/credentials.rs** - Keychain実装（セキュリティクリティカル）
3. **src-tauri/src/platforms/bluesky.rs** - プラットフォームモジュール適応の参考パターン
4. **src/lib/api.ts** - Tauriコマンド呼び出しラッパー
5. **src/App.tsx** - 状態管理とルーティングロジック
6. **src/components/PostForm.tsx** - メインUI投稿機能（投稿フォーム + 結果表示）

---

## 8. 成功基準

### 機能要件
- ✅ 認証情報を安全に保存可能（Keychain）
- ✅ Bluesky、X、Threadsに個別投稿可能
- ✅ 全プラットフォームに同時投稿可能
- ✅ 成功した投稿のURLを表示
- ✅ 失敗時に明確なエラーメッセージ表示
- ✅ macOS（Sonoma以降）で動作
- ✅ iOS（13.0以降）で動作

### 非機能要件
- ✅ アプリ起動3秒以内
- ✅ 投稿完了10秒以内（全プラットフォーム）
- ✅ UIレスポンシブ（投稿中フリーズなし）
- ✅ オフライン時も認証情報管理可能
- ✅ 明確なエラーメッセージ（技術的スタックトレースなし）

### セキュリティ要件
- ✅ 平文での認証情報保存なし
- ✅ すべてのAPI呼び出しはHTTPS
- ✅ 認証情報は他のアプリからアクセス不可
- ✅ 認証情報のコンソールログ出力なし
- ✅ 最小限のTauri権限

---

## 参考資料

- [Tauri 2.0 公式ドキュメント](https://v2.tauri.app/)
- [Tauri Mobile Alpha リリース](https://v2.tauri.app/blog/tauri-mobile-alpha/)
- [Tauri iOS配布ガイド](https://v2.tauri.app/distribute/app-store/)
- [Keyring Rust Crate](https://docs.rs/keyring)
- [Base UI](https://base-ui.com/)
- [Tauri セキュアストレージ議論](https://github.com/tauri-apps/tauri/discussions/7846)
