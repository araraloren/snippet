# snippet

Generate and run simple c/c++/rust code in the command line.

## Usage

### Install snippet

cargo install snippet

### Install compiler and language plugin

snippet f snippet-compiler-gcc

snippet f snippet-language-c

Checkout plugins list in [`plugins.ini`](https://github.com/araraloren/snippet/blob/main/packages/snippet/plugins.ini)

### Run code


```sh
snippet c -e "printf(\"hello from wasip1\")" -p -fmt="clang-format=--style=GNU" -cat="bat=-l=C"
```

