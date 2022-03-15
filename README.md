# MailDotter

This project aims to embed a hidden hash into an email address from a provider that ignores dots (such as gmail.com).

This can be useful to know which website leaked your email address !

## Usage

```
mail_dotter 0.1.0

USAGE:
    mail_dotter --email <EMAIL> --website <WEBSITE>

OPTIONS:
    -e, --email <EMAIL>        Email you want to dot
    -h, --help                 Print help information
    -V, --version              Print version information
    -w, --website <WEBSITE>    Website you want to generate a dotted email for
```

For example :
```
mail_dotter --email google@gmail.com --website stackoverflow.com
```

Will output the following email : `go.ogl.e@gmail.com`
The logic behind it is the following :
- We take the SHA256 hash of the website (here `stackoverflow.com`) : "96589e5f7a71b926dc76df85c95ae55a3deecca8f27dbbb6fec537dc733216b5"
- Instead of the hexdigest representation of the SHA256 hash, we take the byte array : `[150, 88, 158, 95, 122, 113, 185, 38, 220, 118, 223, 133, 201, 90, 229, 90, 61, 238, 204, 168, 242, 125, 187, 182, 254, 197, 55, 220, 115, 50, 22, 181]`
- We sum the array : 4754 and store it as `WEBSITE_HASH`
- We calculate the maximum amount of bits we can hide into the email address which is basically the amount of the local part of the email address minus one (in our example, we can hide 5 bits inside google) and store it as `MAX_HIDDEN_BITS`
- We calculate `WEBSITE_HASH % 2^MAX_HIDDEN_BITS` and store it as `WEBSITE_HASH_MOD` (4764 % 2^5 = 18)
- We rebuild the local part of the email address and add a dot to each letter whom index is a "true" bit on the `WEBSITE_HASH_MOD` variable

On our example, 18 is equal to 0b10010. Note that the bitmask is applied from right to left !

```
  0 1 0 0 1
 g o o g l e
 g o.o g l.e
```
