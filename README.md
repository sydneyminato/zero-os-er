# Zero OS v0.5: Edited Reality Magick for Permission

**Zero OS** は、既存のOSという「編集済み現実」を解体し、ハードウェアとのダイレクトな対話を通じて「現実を再編集」するための、Rust製最小構成カーネルです。

### 🔮 Concept: Edited Reality (ER)
このプロジェクトは、OSという巨大なブラックボックスを解体し、最小限の「Permission（許可）」を得ることで、独自の宇宙を構築する実験場です。

### 🛠 Features (v0.5)
- **Direct VGA Text Mode**: 外部ライブラリを介さない、メモリ直結の文字描画。
- **Chromatic Magick**: F4キーによる画面色のリアルタイム属性変更。
- **Sonic Waveform**: PCスピーカーの周波数を直接制御し、音を生成（F5/F6）。
- **Memory Persistence**: F1-F3キーによる、VGAメモリ上の状態保存と復元。
- **Clean Boot**: BIOSの残置メッセージを消去し、漆黒の空間から起動。

### 🚀 How to Build & Run
```bash
# Build
cargo build --target i686-zero_os.json -Z build-std=core,compiler_builtins -Z json-target-spec

# Run with QEMU
qemu-system-x86_64 -kernel target/i686-zero_os/debug/zero_os_kernel -audiodev coreaudio,id=audio0 -machine pcspk-audiodev=audio0
