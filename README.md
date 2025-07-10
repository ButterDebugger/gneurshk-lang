# Gneurshk

A easy to use and blazingly fast programming language written in Rust using LLVM as a backend

> [!NOTE]
> This project is still in its early stages so things are subject to change _including the name_

## Contributing

Suggestions and contributions are always welcome and appreciated :)

## Compiling

If you would like to contribute to the project, please follow the steps below to get started.

### Prerequisites

-   Rust
-   LLVM 18.1.x

> [!IMPORTANT]
> On Windows, you should install LLVM by building it from its source as the windows installer does not come with `llvm-config`
>
> The install location should not contain spaces, so it is best to install it at `C:\LLVM`
>
> Lastly two environment variables need to be updated:
>
> -   `LLVM_SYS_181_PREFIX` should be set to the path that LLVM is installed at
> -   `PATH` should include the path to the LLVM's `bin` directory

### Installation

1. Clone the Repo:<br>
   `git clone https://github.com/ButterDebugger/gneurshk-lang.git`

2. Build the project:<br>
   `cargo build`

3. Run the CLI:<br>
   `cargo run -- help`

## License

Gneurshk is licensed under [`MIT License`](LICENSE).
