[back](/readme.md)

# Developing

clone project ini. lalu coba jalankan `cargo build`
untuk menjalankan, tulis `cargo run`
untuk membuka dokumentasi yang otomatis dibuat cargo `cargo doc --open`

## Frontend

Jika kamu tidak menyukai bentuk dari ui nya, kamu bisa langsung mengedit folder `public`.
Note: Server hanya akan membaca `index.html`, `404.html`, `index.js`, dan `style.css`. sehingga tidak memungkinkan untuk memanggil file lain (kecuali kamu ubah sendiri source codenya tentunya)

ini bisa diakali menggunakan tools seperti scss/tailwind (untuk css) dan webpack atau sejenisnya (untuk js)

## Aplikasi

`main.rs` entry point untuk aplikasi, disini tempat mengatur argument yang diberikan
`model.rs` file yang menyediakan struct `Files` sesuatu yang berhubungan dengan searching.
`search.rs` file yang mengatur argument `search`
`serve.rs` file yang mengatur argument `serve`. Routing dan file hosting ada disini
`utils.rs` utility seperti open dialog box, get preview, parse config, dll
