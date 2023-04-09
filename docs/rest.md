[back](/readme.md)

### /api/search

- Method: Post
- Description: Lakukan searching
- Body : {
  path: String, //File path
  query: String,
  quantity: number,
  }
- return {
  data : \[String,Number][]
  }

### /api/open

- Method: Post
- Description: Buka file di folder
- Body : {
  path: String,
  }
- return {
  status_code : number,
  success: boolean,
  reason?: String,
  }

### /api/dialog

\*SELAMA DIALOG BOX TERBUKA, SELURUH REQUEST AKAN TERBLOCK\*

- Method: Post
- Description: Buka dialogbox
- return {
  status_code : number,
  data: String,
  success: boolean,
  reason?: String,
  }

### /api/preview

- Method: Post
- Description: Preview content
- return {
  status_code : number,
  data: String,
  success: boolean,
  reason?: String,
  }
