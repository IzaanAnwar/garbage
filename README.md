# Garbage - CLI Based Garbage Bin

Garbage is a simple command-line tool for managing trashed files and directories. It provides basic functionality to delete, restore, and empty the trash directory.

## Installation

To install Garbage, make sure you have Rust and Cargo (the Rust package manager) installed. Then run the following command:

```bash
apt install garbage-cli
```

# Usage

## Deleting Files 
`garbage myfile.txt`

## Cleaning the Garbage Box
`garbage --empty`

## Restoring the deleted files
`garbage --restore` 
*It will give you list of all the deleted files in the pwd*

## License

This project is licensed under the [MIT License](LICENSE).

## Contact

For questions, suggestions, or support, please feel free to contact us at [mdizaan67@gmail.com].
