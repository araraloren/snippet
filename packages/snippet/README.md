# snippet

Generate and run simple c/c++/rust code in the command line.

## Usage

### Install snippet

cargo install snippet

### Install compiler and language plugin

snippet install snippet-compiler-c
snippet install snippet-language-c

### Run code


```sh
snippet c -e "printf(\"hello from wasip1\")" -p -fmt="clang-format=--style=GNU" -cat="bat=-l=C"
```

