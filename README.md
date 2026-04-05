# Zero OS v0.5: Edited Reality Magick for Permission

![Zero OS First Boot](ここにスクリーンショットをドラッグ＆ドロップ)

> **"A collision of two realities."**
> QEMUのBIOSメッセージの上に、Zero OSが直接意志を書き込んだ、記念すべき最初の起動画面。

---

**Zero OS** は、既存のOSという「編集済み現実」を解体し、ハードウェアとのダイレクトな対話を通じて「現実を再編集」するための、Rust製最小構成カーネル（微小OS）です。

### 🔮 Concept: Edited Reality (ER)
このプロジェクトは、OSという巨大なブラックボックスを解体し、最小限の「Permission（許可）」を得ることで、独自の宇宙を構築する実験場です。

### 🛠 Features (v0.5)
- **Direct VGA Text Mode**: 外部ライブラリを一切介さない、メモリ直結の文字描画。
- **Chromatic Magick**: F4キーによる画面色のリアルタイム属性変更。
- **Sonic Waveform**: PCスピーカーの周波数を直接制御し、音を生成（F5/F6）。
- **Memory Persistence**: F1-F3キーによる、VGAメモリ上の状態（文字・色）の保存と復元。
- **Clean Boot**: 強力な初期化により、BIOSの残置メッセージを消去し漆黒の空間を支配。

### 🚀 How to Build & Run
ビルドにはRustのナイトリー機能（build-std等）を使用します。

```bash
# Build (JSONターゲットスペックの使用許可を明示)
cargo build --target i686-zero_os.json -Z build-std=core,compiler_builtins -Z json-target-spec

# Run with QEMU (オーディオデバイスを有効化)
qemu-system-x86_64 -kernel target/i686-zero_os/debug/zero_os_kernel -audiodev coreaudio,id=audio0 -machine pcspk-audiodev=audio0
