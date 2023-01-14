KeeLoq
======

This project is an open source pure `Rust` implementation of Microchip's KeeLoq proprietary
encryption algorithm, used in their series of KeeLoq remote key entry (RKE) devices.

Amongst others, these devices include:

 - [Microchip HCS200](http://ww1.microchip.com/downloads/en/devicedoc/40138c.pdf)
 - [Microchip HCS301](http://ww1.microchip.com/downloads/en/devicedoc/21143b.pdf)
 - [Electronic Giant EG301](https://www.egmicro.com/download/EG301_datasheet.pdf)

Usage example
-------------

This project literally has two methods, so using it is pretty straightforward:


**Encrypt**
```rust
use leekoq::LeeKoq;

let plain: u32 = 0x12345678;
let key: u64 = 0xCAFED00D;
assert_eq!(LeeKoq::encrypt(plain, key), 0xD0FB287C);
```
**Decrypt**
```rust
use leekoq::LeeKoq;

let cipher: u32 = 0xD0FB287C;
let key: u64 = 0xCAFED00D;
assert_eq!(LeeKoq::decrypt(cipher, key), 0x12345678);
```

Keys are 64-bit integers, and cipher- and plaintexts are 32-bit integers.

License
-------
KeeLoq is a registered trademark of Microchip Technologies Inc.

Please note that neither the author is not affiliated with, nor this project is endorsed by
Microchip Technologies, Inc.