<!--
## Control Flow
-->

## 制御フロー

<!--
The ability to run some code depending on whether a condition is `true` and to
run some code repeatedly while a condition is `true` are basic building blocks
in most programming languages. The most common constructs that let you control
the flow of execution of Rust code are `if` expressions and loops.
-->

条件が`true`かどうかによってコードを走らせたり、
条件が`true`の間繰り返しコードを走らせたりできることは、多くのプログラミング言語において、基本的な構成ブロックです。
Rustコードの実行フローを制御する最も一般的な文法要素は、`if`式とループです。

<!--
### `if` Expressions
-->

### `if`式

<!--
An `if` expression allows you to branch your code depending on conditions. You
provide a condition and then state, “If this condition is met, run this block
of code. If the condition is not met, do not run this block of code.”
-->

if式によって、条件に依存して枝分かれをさせることができます。条件を与え、以下のように宣言します。
「もし条件が合ったら、この一連のコードを実行しろ。条件に合わなければ、この一連のコードは実行するな」と。

<!--
Create a new project called *branches* in your *projects* directory to explore
the `if` expression. In the *src/main.rs* file, input the following:
-->

*projects*ディレクトリに*branches*という名のプロジェクトを作って`if`式について掘り下げていきましょう。
*src/main.rs*ファイルに、以下のように入力してください:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

<!--
All `if` expressions start with the keyword `if`, followed by a condition. In
this case, the condition checks whether or not the variable `number` has a
value less than 5. We place the block of code to execute if the condition is
`true` immediately after the condition inside curly brackets. Blocks of code
associated with the conditions in `if` expressions are sometimes called *arms*,
just like the arms in `match` expressions that we discussed in the [“Comparing
the Guess to the Secret Number”][comparing-the-guess-to-the-secret-number]
section of Chapter 2.
-->

`if`式は全て、キーワードの`if`から始め、条件式を続けます。今回の場合、
条件式は変数`number`が5未満の値になっているかどうかをチェックします。
条件が`true`の時に実行する一連のコードを条件式の直後に波かっこで包んで配置します。`if`式の条件式と紐付けられる一連のコードは、
時として*アーム*と呼ばれることがあります。
第2章の[「予想と秘密の数字を比較する」][comparing-the-guess-to-the-secret-number]の節で議論した`match`式のアームと同じです。

<!--
Optionally, we can also include an `else` expression, which we chose to do
here, to give the program an alternative block of code to execute should the
condition evaluate to `false`. If you don’t provide an `else` expression and
the condition is `false`, the program will just skip the `if` block and move on
to the next bit of code.
-->

オプションとして、`else`式を含むこともでき(ここではそうしています)、これによりプログラムは、
条件式が偽になった時に実行するコードを与えられることになります。仮に、`else`式を与えずに条件式が偽になったら、
プログラムは単に`if`ブロックを飛ばして次のコードを実行しにいきます。

<!--
Try running this code; you should see the following output:
-->

このコードを走らせてみましょう; 以下のような出力を目の当たりにするはずです:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

<!--
Let’s try changing the value of `number` to a value that makes the condition
`false` to see what happens:
-->

`number`の値を条件が`false`になるような値に変更してどうなるか確かめてみましょう:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

<!--
Run the program again, and look at the output:
-->

再度プログラムを実行して、出力に注目してください:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

<!--
It’s also worth noting that the condition in this code *must* be a `bool`. If
the condition isn’t a `bool`, we’ll get an error. For example, try running the
following code:
-->

このコード内の条件式は、`bool`型で*なければならない*ことにも触れる価値があります。
条件式が、`bool`型でない時は、エラーになります。例えば、試しに以下のコードを実行してみてください:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

<!--
The `if` condition evaluates to a value of `3` this time, and Rust throws an
error:
-->

今回、`if`の条件式は`3`という値に評価され、コンパイラがエラーを投げます:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

<!--
The error indicates that Rust expected a `bool` but got an integer. Unlike
languages such as Ruby and JavaScript, Rust will not automatically try to
convert non-Boolean types to a Boolean. You must be explicit and always provide
`if` with a Boolean as its condition. If we want the `if` code block to run
only when a number is not equal to `0`, for example, we can change the `if`
expression to the following:
-->

このエラーは、コンパイラは`bool`型を予期していたのに、整数だったことを示唆しています。
RubyやJavaScriptなどの言語とは異なり、Rustでは、論理値以外の値が、自動的に論理値に変換されることはありません。
明示し、必ず`if`には条件式として、`論理値`を与えなければなりません。
例えば、数値が`0`以外の時だけ`if`のコードを走らせたいなら、以下のように`if`式を変更することができます:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

<!--
Running this code will print `number was something other than zero`.
-->

このコードを実行したら、`number was something other than zero`と表示されるでしょう。

<!--
#### Handling Multiple Conditions with `else if`
-->

#### `else if`で複数の条件を扱う

<!--
You can use multiple conditions by combining `if` and `else` in an `else if`
expression. For example:
-->

`if`と`else`を組み合わせて`else if`式にすることで複数の条件を使うこともできます。例です:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

<!--
This program has four possible paths it can take. After running it, you should
see the following output:
-->

このプログラムには、通り道が4つあります。実行後、以下のような出力を目の当たりにするはずです:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

<!--
When this program executes, it checks each `if` expression in turn and executes
the first body for which the condition evaluates to `true`. Note that even
though 6 is divisible by 2, we don’t see the output `number is divisible by 2`,
nor do we see the `number is not divisible by 4, 3, or 2` text from the `else`
block. That’s because Rust only executes the block for the first `true`
condition, and once it finds one, it doesn’t even check the rest.
-->

このプログラムを実行すると、`if`式が順番に吟味され、最初に条件が`true`に評価された本体が実行されます。
6は2で割り切れるものの、`number is devisible by 2`や、
`else`ブロックの`number is not divisible by 4, 3, or 2`という出力はされないことに注目してください。
それは、Rustが最初の`true`な条件のブロックのみを実行し、
条件に合ったものが見つかったら、残りはチェックすらしないからです。

<!--
Using too many `else if` expressions can clutter your code, so if you have more
than one, you might want to refactor your code. Chapter 6 describes a powerful
Rust branching construct called `match` for these cases.
-->

`else if`式を使いすぎると、コードがめちゃくちゃになってしまうので、1つ以上あるなら、
コードをリファクタリングしたくなるかもしれません。これらのケースに有用な`match`と呼ばれる、
強力なRustの枝分かれ文法要素については第6章で解説します。

<!--
#### Using `if` in a `let` Statement
-->

#### `let`文内で`if`式を使う

<!--
Because `if` is an expression, we can use it on the right side of a `let`
statement to assign the outcome to a variable, as in Listing 3-2.
-->

`if`は式なので、`let`文の右辺に持ってきて結果を変数に代入することができます。リスト3-2のようにですね。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

<!--
<span class="caption">Listing 3-2: Assigning the result of an `if` expression
to a variable</span>
-->

<span class="caption">リスト3-2: `if`式の結果を変数に代入する</span>

<!--
The `number` variable will be bound to a value based on the outcome of the `if`
expression. Run this code to see what happens:
-->

この`number`変数は、`if`式の結果に基づいた値に束縛されます。このコードを走らせてどうなるか確かめてください:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

<!--
Remember that blocks of code evaluate to the last expression in them, and
numbers by themselves are also expressions. In this case, the value of the
whole `if` expression depends on which block of code executes. This means the
values that have the potential to be results from each arm of the `if` must be
the same type; in Listing 3-2, the results of both the `if` arm and the `else`
arm were `i32` integers. If the types are mismatched, as in the following
example, we’ll get an error:
-->

一連のコードは、そのうちの最後の式に評価され、数値はそれ単独でも式になることを思い出してください。
今回の場合、この`if`式全体の値は、どのブロックのコードが実行されるかに基づきます。これはつまり、
`if`の各アームの結果になる可能性がある値は、同じ型でなければならないということになります;
リスト3-2で、`if`アームも`else`アームも結果は、`i32`の整数でした。以下の例のように、
型が合わない時には、エラーになるでしょう:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

<!--
When we try to compile this code, we’ll get an error. The `if` and `else` arms
have value types that are incompatible, and Rust indicates exactly where to
find the problem in the program:
-->

このコードをコンパイルしようとすると、エラーになります。`if`と`else`アームは互換性のない値の型になり、
コンパイラがプログラム内で問題の見つかった箇所をズバリ指摘してくれます:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

<!--
The expression in the `if` block evaluates to an integer, and the expression in
the `else` block evaluates to a string. This won’t work because variables must
have a single type, and Rust needs to know at compile time what type the
`number` variable is, definitively. Knowing the type of `number` lets the
compiler verify the type is valid everywhere we use `number`. Rust wouldn’t be
able to do that if the type of `number` was only determined at runtime; the
compiler would be more complex and would make fewer guarantees about the code
if it had to keep track of multiple hypothetical types for any variable.
-->

`if`ブロックの式は整数に評価され、`else`ブロックの式は文字列に評価されます。これでは動作しません。
変数は単独の型でなければならず、コンパイラは、コンパイル時に`number`変数の型を確実に把握する必要があるからです。
`number`の型を把握していることで、コンパイラは、コンパイル時に`number`が使われている箇所全部で型が有効であるか検証できるのです。
`number`の型が実行時にしか決まらないのであれば、コンパイラはそれを実行することができなくなってしまいます;
どの変数に対しても、架空の複数の型があることを追いかけなければならないのであれば、コンパイラはより複雑になり、
コードに対して行える保証が少なくなってしまうでしょう。

<!--
### Repetition with Loops
-->

### ループでの繰り返し

<!--
It’s often useful to execute a block of code more than once. For this task,
Rust provides several *loops*, which will run through the code inside the loop
body to the end and then start immediately back at the beginning. To experiment
with loops, let’s make a new project called *loops*.
-->

一連のコードを1回以上実行できると、しばしば役に立ちます。この作業用に、
Rustにはいくつかの*ループ*が用意されています。ループは、本体内のコードを最後まで実行し、
直後にまた最初から処理を開始します。
ループを試してみるのに、*loops*という名の新プロジェクトを作りましょう。

<!--
Rust has three kinds of loops: `loop`, `while`, and `for`. Let’s try each one.
-->

Rustには3種類のループが存在します: `loop`と`while`と`for`です。それぞれ試してみましょう。

<!--
#### Repeating Code with `loop`
-->

#### `loop`でコードを繰り返す

<!--
The `loop` keyword tells Rust to execute a block of code over and over again
forever or until you explicitly tell it to stop.
-->

`loop`キーワードを使用すると、同じコードを何回も何回も永遠に、明示的にやめさせるまで実行します。

<!--
As an example, change the *src/main.rs* file in your *loops* directory to look
like this:
-->

例として、*loops*ディレクトリの*src/main.rs*ファイルを以下のような感じに書き換えてください:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

<!--
When we run this program, we’ll see `again!` printed over and over continuously
until we stop the program manually. Most terminals support the keyboard
shortcut <span class="keystroke">ctrl-c</span> to interrupt a program that is
stuck in a continual loop. Give it a try:
-->

このプログラムを実行すると、プログラムを手動で止めるまで、何度も何度も続けて`again!`と出力するでしょう。
ほとんどの端末で<span class="keystroke">ctrl-c</span>というショートカットが使え、
永久ループに囚われてしまったプログラムに割り込むことができます。試しにやってみましょう:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

<!--
The symbol `^C` represents where you pressed <span
class="keystroke">ctrl-c</span>. You may or may not see the word `again!`
printed after the `^C`, depending on where the code was in the loop when it
received the interrupt signal.
-->

`^C`という記号が出た場所が、<span class="keystroke">ctrl-c</span>を押した場所です。`^C`の後には`again!`と表示されたり、
されなかったりします。割り込みシグナルをコードが受け取った時にループのどこにいたかによります。

<!--
Fortunately, Rust also provides a way to break out of a loop using code. You
can place the `break` keyword within the loop to tell the program when to stop
executing the loop. Recall that we did this in the guessing game in the
[“Quitting After a Correct Guess”][quitting-after-a-correct-guess]
section of Chapter 2 to exit the program when the user won the game by
guessing the correct number.
-->

幸いなことに、Rustにはコードによってループを抜け出す手段もあります。
ループ内に`break`キーワードを配置することで、プログラムに実行を終了すべきタイミングを教えることができます。
第2章の[「正しい予想をした後に終了する」][quitting-after-a-correct-guess]節の数当てゲーム内でこれをして、ユーザが予想を的中させ、
ゲームに勝った時にプログラムを終了させたことを思い出してください。

<!--
We also used `continue` in the guessing game, which in a loop tells the program
to skip over any remaining code in this iteration of the loop and go to the
next iteration.
-->

数当てゲームで`continue`を使用しました。`continue`はループの中で残っているコードをスキップして次のループに移るためのものです。

<!--
#### Returning Values from Loops
-->

#### ループから値を返す

<!--
One of the uses of a `loop` is to retry an operation you know might fail, such
as checking whether a thread has completed its job. You might also need to pass
the result of that operation out of the loop to the rest of your code. To do
this, you can add the value you want returned after the `break` expression you
use to stop the loop; that value will be returned out of the loop so you can
use it, as shown here:
-->

`loop`の使用法のひとつとして、失敗するかもしれないと分かっている操作、
例えばスレッドがそのジョブを完了したかを確認する操作などを、リトライするというのがあります。
さらに、コードの他の部分で使うために、ループの外に操作の結果を渡す必要があるかもしれません。
これを行うには、ループを止めるために使っている`break`式の後ろに、返したい値を付け加えてください;
その値はループを抜けて返され、使うことができます。このように:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

<!--
Before the loop, we declare a variable named `counter` and initialize it to
`0`. Then we declare a variable named `result` to hold the value returned from
the loop. On every iteration of the loop, we add `1` to the `counter` variable,
and then check whether the `counter` is equal to `10`. When it is, we use the
`break` keyword with the value `counter * 2`. After the loop, we use a
semicolon to end the statement that assigns the value to `result`. Finally, we
print the value in `result`, which in this case is `20`.
-->

ループの前で`counter`という変数を宣言し、`0`に初期化しています。
次に、ループから返された値を保持するために、`result`という変数を宣言しています。
ループの繰り返しごとに、`counter`変数に`1`を追加し、`counter`が`10`に等しいかチェックします。
等しい場合は、値`counter * 2`とともに`break`キーワードを使用しています。
ループの後には、値を`result`に代入する文を終了するためのセミコロンが使われています。
最後に、`result`の値を出力していて、この場合は`20`です。

<!--
#### Loop Labels to Disambiguate Between Multiple Loops
-->

#### 複数のループを区別するループラベル

<!--
If you have loops within loops, `break` and `continue` apply to the innermost
loop at that point. You can optionally specify a *loop label* on a loop that
you can then use with `break` or `continue` to specify that those keywords
apply to the labeled loop instead of the innermost loop. Loop labels must begin
with a single quote. Here’s an example with two nested loops:
-->

ループ内にループがある場合、`break`と`continue`は最も内側のループに適用されます。
*ループラベル*を使用することで、`break`や`continue`が適用されるループを指定することができます。
ループラベルはシングルクオートで始める必要があります。
以下に例を示します。


```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

<!--
The outer loop has the label `'counting_up`, and it will count up from 0 to 2.
The inner loop without a label counts down from 10 to 9. The first `break` that
doesn’t specify a label will exit the inner loop only. The `break
'counting_up;` statement will exit the outer loop. This code prints:
-->

外側のループには`'counting_up`というラベルがついていて、0から2まで数え上げます。
内側のラベルのないループは10から9までカウントダウンします。最初のラベルの無い`break`は内側のループを終了させます。
`break 'counting_up;`は外側のループを終了させます。
このコードは以下のような出力をします。

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```



<!--
#### Conditional Loops with `while`
-->

#### `while`で条件付きループ

<!--
A program will often need to evaluate a condition within a loop. While the
condition is `true`, the loop runs. When the condition ceases to be `true`, the
program calls `break`, stopping the loop. It’s possible to implement behavior
like this using a combination of `loop`, `if`, `else`, and `break`; you could
try that now in a program, if you’d like. However, this pattern is so common
that Rust has a built-in language construct for it, called a `while` loop. In
Listing 3-3, we use `while` to loop the program three times, counting down each
time, and then, after the loop, print a message and exit.
-->

プログラムは、ループ内で条件式を評価することがよく必要になるでしょう。条件が`true`の間、
ループが走るわけです。条件が`true`でなくなった時にプログラムは`break`を呼び出し、ループを終了します。
このような挙動は、`loop`、`if`、`else`、`break`を組み合わせることでも実装できます;
お望みなら、プログラムで今、試してみるのもいいでしょう。
しかし、このパターンは頻出するので、Rustにはそれ用の文法要素が用意されていて、`while`ループと呼ばれます。
リスト3-3では、プログラムを3回ループさせるために`while`を使用しています。
繰り返しごとにカウントダウンして、ループ後にメッセージを表示して終了します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

<!--
<span class="caption">Listing 3-3: Using a `while` loop to run code while a
condition holds true</span>
-->

<span class="caption">リスト3-3: 条件が真の間、コードを走らせる`while`ループを使用する</span>

<!--
This construct eliminates a lot of nesting that would be necessary if you used
`loop`, `if`, `else`, and `break`, and it’s clearer. While a condition
evaluates to `true`, the code runs; otherwise, it exits the loop.
-->

この文法要素により、`loop`、`if`、`else`、`break`を使った時に必要になるネストがなくなり、
より明確になります。条件が`true`に評価される間、コードは実行されます; そうでなければ、ループを抜けます.

<!--
#### Looping Through a Collection with `for`
-->

#### `for`でコレクションを覗き見る

<!--
You can choose to use the `while` construct to loop over the elements of a
collection, such as an array. For example, the loop in Listing 3-4 prints each
element in the array `a`.
-->

配列などのコレクションの要素を覗き見るために、`while`要素を使うこともできます。
例えば、リスト3-4のループは配列`a`の各要素を出力します。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

<!--
<span class="caption">Listing 3-4: Looping through each element of a collection
using a `while` loop</span>
-->

<span class="caption">リスト3-4: `while`ループでコレクションの各要素を覗き見る</span>

<!--
Here, the code counts up through the elements in the array. It starts at index
`0`, and then loops until it reaches the final index in the array (that is,
when `index < 5` is no longer `true`). Running this code will print every
element in the array:
-->

ここで、コードは配列の要素を順番にカウントアップして覗いています。番号0から始まり、
配列の最終番号に到達するまでループします(つまり、`index < 5`が`true`でなくなる時です)。
このコードを走らせると、配列内の全要素が出力されます:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

<!--
All five array values appear in the terminal, as expected. Even though `index`
will reach a value of `5` at some point, the loop stops executing before trying
to fetch a sixth value from the array.
-->

予想通り、配列の5つの要素が全てターミナルに出力されています。`index`変数の値はどこかで`5`という値になるものの、
配列から6番目の値を拾おうとする前にループは実行を終了します。

<!--
However, this approach is error prone; we could cause the program to panic if
the index value or test condition is incorrect. For example, if you changed the
definition of the `a` array to have four elements but forgot to update the
condition to `while index < 4`, the code would panic. It’s also slow, because
the compiler adds runtime code to perform the conditional check of whether the
index is within the bounds of the array on every iteration through the loop.
-->

しかし、このアプローチは間違いが発生しやすいです; 添え字の値や判定条件が間違っていれば、
プログラムはパニックしてしまいます。例えば、`a`配列の定義を4要素を持つように変更したのに、
条件を`while index < 4`に更新し忘れた場合、コードはパニックするでしょう。また遅いです。
実行時にループの各回ごとに添字が配列の境界内にあるかチェックするコードを、コンパイラが追加するからです。

<!--
As a more concise alternative, you can use a `for` loop and execute some code
for each item in a collection. A `for` loop looks like the code in Listing 3-5.
-->

より簡潔な対立案として、`for`ループを使ってコレクションの各アイテムに対してコードを実行することができます。
`for`ループはリスト3-5のコードのようになります。

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

<!--
<span class="caption">Listing 3-5: Looping through each element of a collection
using a `for` loop</span>
-->

<span class="caption">リスト3-5: `for`ループを使ってコレクションの各要素を覗き見る</span>

<!--
When we run this code, we’ll see the same output as in Listing 3-4. More
importantly, we’ve now increased the safety of the code and eliminated the
chance of bugs that might result from going beyond the end of the array or not
going far enough and missing some items.
-->

このコードを走らせたら、リスト3-4と同じ出力が得られるでしょう。より重要なのは、
コードの安全性を向上させ、配列の終端を超えてアクセスしたり、
終端に届く前にループを終えてアイテムを見逃してしまったりするバグの可能性を完全に排除したことです。

<!--
Using the `for` loop, you wouldn’t need to remember to change any other code if
you changed the number of values in the array, as you would with the method
used in Listing 3-4.
-->

`for`ループを使っていれば、配列の要素数を変えても、
リスト3-4で使った方法のように他のコードをいじることを覚えておく必要はなくなるわけです。

<!--
The safety and conciseness of `for` loops make them the most commonly used loop
construct in Rust. Even in situations in which you want to run some code a
certain number of times, as in the countdown example that used a `while` loop
in Listing 3-3, most Rustaceans would use a `for` loop. The way to do that
would be to use a `Range`, provided by the standard library, which generates
all numbers in sequence starting from one number and ending before another
number.
-->

`for`ループのこの安全性と簡潔性により、Rustで使用頻度の最も高いループになっています。
リスト3-3で`while`ループを使ったカウントダウンサンプルのように、一定の回数、同じコードを実行したいような状況であっても、
多くのRustaceanは、`for`ループを使うでしょう。どうやってやるかといえば、
標準ライブラリで提供される`Range`型を使うのです。`Range`型は、片方の数字から始まって、
もう片方の数字未満の数値を順番に生成する型です。

<!--
Here’s what the countdown would look like using a `for` loop and another method
we’ve not yet talked about, `rev`, to reverse the range:
-->

`for`ループと、まだ話していない別のメソッド`rev`を使って範囲を逆順にしたカウントダウンはこうなります:

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">ファイル名: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

<!--
This code is a bit nicer, isn’t it?
-->

こちらのコードの方が少しいいでしょう？

<!--
## Summary
-->

## まとめ

<!--
You made it! This was a sizable chapter: you learned about variables, scalar
and compound data types, functions, comments, `if` expressions, and loops! To
practice with the concepts discussed in this chapter, try building programs to
do the following:
-->

やりましたね！結構長い章でした: 変数、スカラー値と複合データ型、関数、コメント、`if`式、そして、ループについて学びました！
この章で議論した概念について経験を積むために、以下のことをするプログラムを組んでみてください:

<!--
* Convert temperatures between Fahrenheit and Celsius.
* Generate the *n*th Fibonacci number.
* Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
  taking advantage of the repetition in the song.
-->

* 温度を華氏と摂氏で変換する。
* フィボナッチ数列の*n*番目を生成する。
* クリスマスキャロルの定番、"The Twelve Days of Christmas"の歌詞を、
  曲の反復性を利用して出力する。

<!--
When you’re ready to move on, we’ll talk about a concept in Rust that *doesn’t*
commonly exist in other programming languages: ownership.
-->

次に進む準備ができたら、他の言語にはあまり存在*しない*Rustの概念について話しましょう: 所有権です。

<!--
[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]:
ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess
-->

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#予想と秘密の数字を比較する
[quitting-after-a-correct-guess]:
ch02-00-guessing-game-tutorial.html#正しい予想をした後に終了する
