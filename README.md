[![Gitpod ready-to-code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/abplus-lab/makbe)

# makbe-ff

makbeとは、「~~Modulo Architecture の KeyBoard は Eよね！~~ Modulo Architecture KeyBoard Enhancer」の短縮形で、壺を愛でる大佐とは関係ありません。

冗談はさておき、**makbe** は作者（kazhida）が提供するModuloアーキテクチャに基づいて設計されたキーボードについてのあれこれ関連サービスの総称です。

そのなかで、makbe-ffは、Firmware Frameworkを扱います。

## 方針

* I2Cによるキーのスキャン以降は原則として[keyberon](https://github.com/TeXitoi/keyberon) を利用します。なので、makbe-ffで実現できるキーボードとしての機能はkeyberonに依存します。
* ただし、moduloアーキテクチャなので、Row, Colという考え方は持ちません。
* キーアサインは、何らかの方法で動的に変更できるようにします（出来れば、Remapをそのまま使えるようにしたい）。

### 最初のGOAL

examplesにあるように、Seeeduino XIAO で4x4のキーパッドを使えるようにします。



