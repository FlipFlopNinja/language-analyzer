<div id="table-of-contents">
<h2>Table of Contents</h2>
<div id="text-table-of-contents">
<ul>
<li><a href="#org5ff0ae8">1. Task</a>
<ul>
<li><a href="#orgd965e43">1.1. What are terms?</a></li>
<li><a href="#orgb2fc488">1.2. Requirements</a>
<ul>
<li><a href="#org0e73b53">1.2.1. Flexibility and Genericy</a></li>
<li><a href="#orgf5b7c08">1.2.2. Usability</a></li>
<li><a href="#org7071e58">1.2.3. Performance</a></li>
<li><a href="#orgc06abee">1.2.4. Functionality</a></li>
<li><a href="#org7a34cce">1.2.5. Stability</a></li>
</ul>
</li>
</ul>
</li>
<li><a href="#org12d0855">2. How to start</a>
<ul>
<li><a href="#orgc1f8b5b">2.1. Installing Rust</a></li>
<li><a href="#org7c192b5">2.2. Running the example tokenizer</a></li>
<li><a href="#org2adf673">2.3. Writing and running unittests</a></li>
<li><a href="#org5f6ff51">2.4. Resources</a></li>
</ul>
</li>
<li><a href="#org76e2a75">3. Where to start</a></li>
</ul>
</div>
</div>

<a id="org5ff0ae8"></a>

# Task

Build a tool that takes a string through stdin and prints resulting terms to stdout seperated by newline.


<a id="orgd965e43"></a>

## What are terms?

Terms are the result of [tokenization](https://en.wikipedia.org/wiki/Tokenization_(lexical_analysis)), [normalization](https://en.wikipedia.org/wiki/Canonicalization) and [stemming](https://en.wikipedia.org/wiki/Stemming) a string. 
For example the sentence "This is a house" has four terms: "this" "is" "a" "house".


<a id="orgb2fc488"></a>

## Requirements


<a id="org0e73b53"></a>

### Flexibility and Genericy

You want to build sustainable and reusable software.
That is why you should think of a common [trait](https://doc.rust-lang.org/book/traits.html) for each tokenizers, normalizers and stemmers.
Maybe you can also think of a common trait for all three of them. Or for a wrapper structure.


<a id="orgf5b7c08"></a>

### Usability

You should also think of a way how to chain them.
If there are two tokenizers, three normalizers and seven stemmers the user should be able to chain them to his need.


<a id="org7071e58"></a>

### Performance

Make these analyzers lazy! They should not do anything unless asked for results. 
You might consider using [iterators](https://doc.rust-lang.org/std/iter/trait.Iterator.html) to achieve this. 


<a id="orgc06abee"></a>

### Functionality

For now the interfaces are more important than functionality. 
A tokenizer that splits at non-alphanumeric chars, 
a normalizer that lowercases every token and a very basic stemmer should suffice.


<a id="org7a34cce"></a>

### Stability

While using existing [crates](https://crates.io/) is ok, please make sure they are stable enough to be used in a stable product.
Please also refrain from using the nightly release channel of rust.


<a id="org12d0855"></a>

# How to start

This repository contains a whitespace tokenizer in its simplest form.
You can compile it using \`cargo build\` and run it with \`cargo run\`.
But first you have to install rust:


<a id="orgc1f8b5b"></a>

## Installing Rust

See [rustup.rs](https://rustup.rs/) the official Rust installer.


<a id="org7c192b5"></a>

## Running the example tokenizer

    echo "this is a house" | cargo run

<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-left" />
</colgroup>
<tbody>
<tr>
<td class="org-left">this</td>
</tr>


<tr>
<td class="org-left">is</td>
</tr>


<tr>
<td class="org-left">a</td>
</tr>


<tr>
<td class="org-left">house</td>
</tr>
</tbody>
</table>


<a id="org2adf673"></a>

## Writing and running unittests

The example code contains the structure of a test. 
You can execute tests by running \`cargo test\`.
Check [the Book](https://doc.rust-lang.org/book/testing.html) for more!


<a id="org5f6ff51"></a>

## Resources

[Standard Library Reference](https://doc.rust-lang.org/std/)
[The Book](https://doc.rust-lang.org/book/)
[The Guide to Rust Strings](http://www.steveklabnik.com/rust-issue-17340/)


<a id="org76e2a75"></a>

# Where to start

Fork this repo. Clone it locally.

Then I would propose the following iterations:

-   Get comfortable with stdin
-   Write a function signature that takes a string and returns a vector of terms
-   Implement that function using the [split<sub>whitespace</sub>](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) Method
-   Write unit tests for that function
-   Implement this function as an iterator
-   Adapt your unit tests to handle iterators. (Tip: use [collect](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect))
-   Repeat for normalizers and stemmers

By then you should feel comfortable enough to go on on your own!

