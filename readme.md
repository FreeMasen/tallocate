# Text Allocate

A simple terminal utility to create text files of a specific size.

## Usage
```bash
textallocate -s [size] -o [output]
```

## Details

The program will output a lorem ipsum preceded with the iteration required to reach at least the required to reach the desired size. If the lorem doesn't perfectly fit in the desired size it will overflow one iteration.

The ideal use case for this application is for testing data integrity after transforming or moving data. Since the lorem ipsums are preceded with their iteration it is easy to detect differences.