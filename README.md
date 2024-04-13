# lop

A command line utility for shortening links and sharing code and files.

## Installation

### Homebrew (Linux/macOS)

```bash
brew install jake-walker/tap/lop
```

### Cargo

```bash
cargo install --git https://github.com/jake-walker/lop.git
```

## Usage

```bash
# show help
lop help

# shorten a url
lop shorten https://github.com/jake-walker/lop

# upload a file and display qr code
lop upload -Q ./photos.zip
```

## Features

- [x] Shortening URLs
- [x] Uploading files
- [x] Uploading code from clipboard or a file
- [x] Generate QR codes for outputs
- [ ] Upload multiple files (zipping as required)
- [ ] Take inputs from stdin
- [x] Set expiry date
- [ ] Encrypt

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit)
