# afes_2022_gamelauncher
2022年文化祭用ミニゲームランチャー

## 準備
`afes_2022_gamelaunchaer.exe`(リリースビルド)と`assets`ディレクトリを同じディレクトリに配置

## ゲームの追加手順

1. `assets/games/`以下にゲームごとにディレクトリを作る
1. 各ディレクトリには
- ゲームのexeファイル
- 各ゲームが依存している(ゲームが必要としているもので、当然ゲームによって違う)
- `launcher_manifest.toml`ファイル
- `screenshot.png`
を置く  
`screenshot.png`はゲーム選択画面の背景に表示される画像となる  
  
`launcher_manifest.toml`は
```
title = "<タイトル>"
description = "<ゲームの説明>"
author = "<著者名>"
game_exe_name = "<ゲームの実行ファイルの名前(拡張子.exeは必要ない)>"
```
のように記述する  
例えば実行ファイルが`パチンコGAME!!.exe`の去年のゲームだと
```
title = "Advance_cube"
description = "プレイヤーADで回転,スペースで前進します。壁や敵の間をかいくぐってスコアを伸ばしましょう！"
author = "Metain"
game_exe_name = "パチンコGAME!!"
```
となる

## 注意

- UIがあまりに長いゲームタイトルや大量のゲームに対応していません
- crateをアップデートするとビルドがこける可能性があるので、分からないのであればしないこと(`cargo update`等をしなければ`Cargo.lock`通りのクレートが使われるので、それでおｋ)
- 去年の文化祭で動いていたのは[#3ba61c3](https://github.com/nost15459/afes_2022_gamelauncher/commit/3ba61c3775ab05f7af6aae9cf5f34086e20254ec)時点のものです、それ以降は私が**勝手**に手を加えたものなので、無理に採用する必要はありません(以後のコミットも同様)。どちらを使っても(そもそも使わなくても)構いませんし、煮るなり焼くなりあなたが手を加えたものを使用するのも良いでしょう。好きにしてください。

何か質問などがあれば気軽にどうぞ
