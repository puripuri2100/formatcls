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
- `val u-register-cross-reference-num : string -> string -> unit` : 数の登録用（`f key title`）と使います。
- `val ib-register-cross-reference-page : string -> string -> inline-boxes` : ページの登録用（`f key title`）と使って得たinline-boxesを必要な箇所にくっつけます。
- `val s-get-cross-reference-num : string -> string` : 数の取得用
- `val s-get-cross-reference-page : string -> string` : 数の取得用
- `val ib-register-location-frame : string -> inline-boxes -> inline-boxes` :リンクを貼ります 
- `val ib-link-to-location-frame : string -> inline-boxes -> inline-boxes` : リンクに飛びます
- `direct \ref : [string] inline-cmd` : 相互参照
- `direct \ref-page : [string] inline-cmd` : 相互参照（ページ）
- `+p : [inline-text] block-cmd` : 段落用
- `+pn : [inline-text] block-cmd` : 段落用

ドキュメント関数は
```
val document : 'a -> block-text -> document
  constraint 'a :: (|
    title : inline-text;
    author : inline-text;
    other : 'b;
  |)
```
という型を持っています。`other`の部分は`title-fun`で指定したタイトル作成関数に依存します。デフォルトでは`'a`です。

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
- main-font : メインで使うフォントをcjkとlatinでそれぞれ設定します。以下の子データをあたえてください。
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
- require-package : `@require`で読み込むパッケージを指定する文字列のリストです。
- import-package : `@import`で読み込むパッケージを指定する文字列のリストです。
- header-fun : ヘッダーを作成する関数（型は`context -> int -> block-boxes`）を指定してください。デフォルトでは
  - `make-header`
  - `make-footer`
  - `empty`
  が用意されています。
- hooter-fun : フッターを作成する関数を指定してください。`header-fun`と大体共通です。
- sec-depth : 章や節の関数をいくつまで生成するかです。
- sec-name-list : 章や節のコマンド名を与えます。sec-depthでの値よりもリストの長さが短いと、コマンド名は自動生成になります。
- sec-fun : 章や節のタイトルなどを作る関数です。デフォルトでは`make-sec-bb`ですが、自分で指定することもできます）。型は`ctx:context -> label:string -> count-lst:int list -> i:int -> title:inline-text -> outline-lst:((int, string, string, bool) list) ref -> outline-title-opt:inline-text option -> main:block-text -> block-boxes`です。
- toc-depth : 目次の表示深さです。sec-depthより大きいと、自動的にsec-depthと同じ大きさになります。
- toc-fun : sec-funと同じような感じです。デフォルトは`make-toc-bb`で、型は`ctx:context -> text-width:length -> label:string -> count-lst:int list -> i:int -> title:inline-text -> inline-boxes`です。
- toc-title-fun : 目次にデコレーションをする関数です。デフォルトは`make-toc-title-bb`で、型は`ctx:contex -> main-bb:block-boxes -> block-boxes`です。
- title-fun : sec-funと同じような感じです。デフォルトは`make-title-bb`で、型は`ctx:context -> title:inline-text -> author:inline-text -> other:'a -> block-boxes`です。otherの中は何でも含めることができますが、ドキュメント関数に必要なレコードの`other`の部分に必要になります。
- title-page : タイトルを独立のページにするかどうかの真偽値です。
- toc-page : 目次を独立のページにするかどうかの真偽値です。

デフォルトの設定が知りたい場合は`-d`オプションをつけて
```sh
formatcls -d <output file>
```
のように起動してください。