[back](/readme.md)

# About

Project ini dibuat untuk coba coba bahasa pemrograman rust.
Project ini dibuat di sistem operasi linux, cross-platfrom belum di coba

## Daftar isi

- Cara kerja
- pencarian?
- akses file system lewat browser?
- Rocket?
- Improvisasi
- Todo

## Cara kerja

Cara kerja dari program ini cukup sederhana.

- progam ini akan membaca seluruh isi dari sebuah folder.
- ketika pengguna memberikan sebuah query, lakukan pencarian.
  - jika di dalam query mengandung "/", akan dilakukan mode pencarian folder.
- hasil dari query adalah point, urutkan point dari yang tertinggi lalu kembalikan sebanyak user inginkan

## pencarian?

Pencarian menggunakan algoritma fuzzy matching. lebih tepatnya menggunakan ini https://www.forrestthewoods.com/blog/reverse_engineering_sublime_texts_fuzzy_match/

simpel nya, query dan nama file akan di pecah menjadi "array slice of character". lalu akan dilakukan proses matching.

Hasil dari matching adalah point. semakin tinggi pointnya, semakin relevan file nya terhadap query-nya, Setiap salah satu dari syarat dibawah ini, akan ada perubahan point

- Jika hutuf sesuai, point + 0
- jika huruf tidak sesuai, point - 1
- jika 3 huruf pertama tidak sesuai, point - 3 (maks 9)
- jika huruf yang ditemukan adalah huruf besar. point + 10
- jika huruf sebelumnya adalah "-" atau "\_" atau " ". point + 10
- jika huruf sebelumnya juga telah ditemukan. point + 5

proses yang sama juga dilakukan untuk pencarian folder (sintaks `folder/file`). namun proses matching akan dilakukan 2x (untuk directory dan nama file). point yang dikalkulasikan adalah

\[PointDirectory . 2 + PointFile\]

misalnya, file yang dicari adalah "sr/CL".
di dalam sebuah hierarki folder

```
- root
  - src
    - CommandLine.ts
    - commandline.js
  - build
    - CommandLine.js
  - searching
    - CLine.js
```

hasil dari pemetaan folder adalah

```
/root/src/CommandLine.ts
/root/src/commandline.js
/root/build/CommandLine.js
/root/searching/CLine.js
```

akan dilakukan pencarian folder, sehingga query akan dipecah menjadi "sr" dan "app". Sama halnya untuk matching, akan dipecah untuk setiap "/" dan diambil 2 terbelakang

```
src => sr = 5 point
build => sr = -14 point
searching => sr = -8 point

CommandLine.ts => CL = 5
commandline.js => CL = -15
CommandLine.js => CL = 5
CLine.js => CL = 25
```

jika diurutkan maka yang keluar adalah

```
/root/src/CommandLine.ts => 15
/root/searching/CLine.js => 14
/root/build/CommandLine.js => - 9
/root/src/commandline.js => -5
```

Huruf besar (dan tanda pemisah seperti "-" / "\_") akan sangat mempengaruhi perhitungan

## Akses file system lewat browser?

Pada dasarnya, browser tidak memperbolehkan untuk mengakses file system, (yang mana ini diperlukan agar pengguna dapat memilih folder mana yang akan dicari).

ini dapat diakali dengan menggunakan RESTfulAPI [[docs]](/docs/rest.md). misalnya jika kita memerlukan folder path untuk pencarian, browser hanya perlu mengirimkan request ke /api/open untuk membuka dialog box. folderPath dari dialog box itu akan dikirimkan sebagai response dari server

Sama hal nya dengan fitur "open di folder" dan "preview", browser mengirim request ke server lalu aksi akan dieksekusi oleh rust di server

setahu saya, localhost (127.0.0.1) tidak bisa diakses dari luar komputer, seharusnya ini aman dari gangguan orang (seperti spam)

## Kenapa tidak pake Rocket?

Rocket terlalu overkill untuk project kecil ini. saya menantang diri saya untuk meminimalisir penggunaan library.

## Improvisasi

ada banyak bagian untuk yang bisa di perbaiki

- UI/UX, highlight untuk bagian yang matching dapat membantu pengguna
- Exclude folder di UI.
- Cache folder yang akan dicari
- Multithreading. karena setiap request akan memblok program

## Todo

- Beneran membereskan project ini.✅
- Implementasi "Fuzzy Searching" (Reference https://www.forrestthewoods.com/blog/reverse_engineering_sublime_texts_fuzzy_match/) . ✅
  - Tambahkan fitur mencari didalam sebuah folder ("folder/file" syntax). ✅
- Implementasi Web server untuk UI.✅
  - Pilih folder yang di query via ui (bukan cli) ✅
  - Buat dokumentasi untuk RESTfapi. ✅
  - Highlight huruf yang "match" nya.
- Implementasi "buka di folder" ✅ dan "preview" ✅ untuk text file.
  - support photo
