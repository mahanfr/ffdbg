# ffdbg - File Format Debugger

***"ffdbg is a versatile program designed for file format debugging.
it provides a seamless interface for iterating over files, byte by byte, 
facilitating in-depth analysis and troubleshooting of file structures.
you can use this tool for decoding property formats or debugging complex
data structures."*** **- chatgpt**

## How to use
start the program by giving it a file path then the repl program starts whaiting for your next command
```bash
$ ffdbg <file_path>
```

### commands
You can control which bytes you want to be shown using this commands:
| Command | Description |
| --- | --- |
| `<number>` | Shows the next `<number>` of bytes and moves the offset to the end of shown bytes |
| offset `<number>` | moves the offset to the `<number>` |
| reset | resets the offset to the beginning of the file |
| exit | Exits the program |


## Installation

### from Source
To install the program clone this reposetroy and run `make install` while having **the rust compiler** installed
```bash
$ sudo apt install file # optional
# installing the rust compiler (If you dont already have it)
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ git clone https://github.com/mahanfr/ffdbg.git
$ cd ./ffdbg
$ make
$ make install
```

## Contributing
Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE.txt` for more information.


