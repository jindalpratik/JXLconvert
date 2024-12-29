# JXLConvert

A Rust CLI app that converts images in comic archives to JPEG XL format.

## How to Use

Currently, the app only supports converting images in comic archives to JPEG XL format. To use the app, you need to have Rust installed on your system. If you don't have Rust installed, you can install it by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

The app also requires libjxl to be installed on your system and to be in your PATH. You can install libjxl by following the instructions on the [Official JPEGxl Repo](https://github.com/libjxl/libjxl)

After installing Rust and libjxl, you can clone the repository and run the app by running the following commands:

```bash
git clone https://github.com/jindalpratik/JXLconvert.git
cd JXLconvert
cargo run --release
```

The app will prompt you to enter the path to directory containing the comic archives. After entering the path, the app will convert all the images in the comic archives to JPEG XL format.

## Disclaimer

This project is still in development and may not work as expected. Use it at your own risk.

Currently the app overwrites the original comic archives with the converted images. Make sure to backup your comic archives before using the app.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
