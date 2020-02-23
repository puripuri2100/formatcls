# どういうソフトか

jsonファイルを基にしてSATySFiのクラスファイルを自動で生成するソフトウェアです。

# インストール

RustとCargoをインストールしてください。

次にリポジトリをクローンします。

```
git clone https://github.com/puripuri2100/formatcls.git
```

最期にインストールをします。

```
make install
```

# 起動の仕方

設定ファイルを与えるオプションと、出力ファイル名を指定するオプションが必須です。
なので、このような形で起動することになります。

```
formatcls -c <json file> -o <output file>
```

# 出力ファイルの特徴


## 依存パッケージ

以下のパッケージを要求します。
今のところ標準のものだけなので別に用意する必要はありません。

- annot
- option
- list
- math
- gr
- color
- vdecoset

## 提供コマンド・関数

今のところドキュメント型を返すための関数と、以下のコマンドになります。

- `+p : [inline-text] block-cmd` 段落用
- `+pn : [inline-text] block-cmd` 段落用

※標準のクラスファイルではドキュメント関数はトップレベルで使えますが、このソフトウェアが生成するクラスファイルではそのための特別な処理はしていません。そのため、使う時は`Module.document-function`のようにモジュール名を付ける必要があります。

# 設定ファイルに書くもの

- module : クラスファイルのモジュール名を指定できます。文字列であたえてください。
- doc-fun-name : ドキュメント関数の名前を指定できます。文字列であたえてください。
- page-size : a4やb5といったページサイズを文字列で与えてください。
- page-width : ページの横幅です。SATySFiでのlength型を文字列で与えてください。
- page-height : ページの縦幅です。
- left-space : 左側の余白です。SATySFiでのlength型を文字列で与えてください。
- right-space : 右側余白
- top-space : 上側余白
- bottom-space : 下側余白
- main-fong : メインで使うフォントをcjkとlatinでそれぞれ設定します。以下の子データをあたえてください。
  - size : フォントサイズです。
  - cjk-name : cjk用のフォントの名前です。
  - cjk-ratio : cjkの拡大率です。
  - cjk-correction : cjkの補正値です。
  - latin-name : latin用のフォントの名前です。
  - latin-ratio : latinの拡大率です。
  - latin-correction : latinの補正値です。
- font-data : フォントを登録します。以下の子データのリストをあたえてください。
  - tag : 書体です。
    - roman
    - bold
    - italic
    - sans
    - mono
    - mincho
    - gothic
  を指定すると、それぞれのフォントを上書きすることができます。
  - name : フォントの名前です。
  - ratio : 拡大率です。
  - correction : 補正値です。