# afes_gamelauncher
文化祭用ミニゲームランチャー

## 準備
`afes_gamelaunchaer.exe`(リリースビルド)と`assets`ディレクトリを同じディレクトリに配置

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

- crateをアップデートするとビルドがこける可能性があるので、分からないのであればしないこと(`cargo update`等をしなければ`Cargo.lock`通りのクレートが使われるので、それでおｋ)

何か質問などがあれば気軽にどうぞ
